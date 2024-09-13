use std::{
    fmt::{self, Debug, Formatter},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{channel, Sender},
        Arc, Condvar, Mutex, OnceLock,
    },
    thread::spawn,
};

use super::{ctx::ThreadPoolContext, sentinel::Sentinel};

pub enum Message<T> {
    NewJob(T),
    Shutdown,
}

pub type Action = Message<Box<dyn FnOnce() + Send + 'static>>;

/// Global single instance
static POOL: OnceLock<ThreadPool> = OnceLock::<ThreadPool>::new();

/// Main struct of the thread pool
///
/// # Note
///
/// The `nested-join` is not allowed, it may cause a deadlock.
///
/// But this will not happen under the current design (with global singulation).
///
/// # Example
///
/// ```rust
/// use ezcfg_linker::ThreadPool;
///
/// let pool = ThreadPool::global();
///
/// let task_count = 10;
/// for _ in 0..task_count {
///    pool.execute(|| {
///       println!("Hello, world!");
///   });
/// }
///
/// pool.join();
/// ```
pub struct ThreadPool {
    sender: Sender<Action>,
    ctx: Arc<ThreadPoolContext>,
}

impl ThreadPool {
    /// Get the global thread pool instance
    /// If the global instance does not exist, it will be created
    /// with the number of CPUs as the size of the thread pool.
    ///
    /// # Example
    /// ```rust
    /// use ezcfg_linker::ThreadPool;
    ///
    /// let pool = ThreadPool::global();
    /// ```
    pub fn global() -> &'static ThreadPool {
        POOL.get_or_init(|| ThreadPool::new(num_cpus::get()))
    }

    /// Create a new thread pool with the specified size.
    /// The size must be greater than 0.
    ///
    /// # Panic
    /// If the size is less than or equal to 0, it will panic.
    fn new(size: usize) -> Self {
        assert!(size > 0, "ThreadPool size must be greater than 0");

        let (sender, receiver) = channel();

        let ctx = Arc::new(ThreadPoolContext {
            receiver: Mutex::new(receiver),

            actived_count: AtomicUsize::new(0),
            pending_count: AtomicUsize::new(0),
            packing_count: AtomicUsize::new(0),

            cond_lock: Mutex::new(()),
            cond: Condvar::new(),
        });

        for _ in 0..size {
            spawn_in_pool(ctx.clone());
        }

        ThreadPool { sender, ctx }
    }

    /// Get the number of running threads in the thread pool
    ///
    /// # Example
    /// ```rust
    /// use ezcfg_linker::ThreadPool;
    ///
    /// let pool = ThreadPool::global();
    /// assert_eq!(pool.actived_count(), 0);
    /// ```
    pub fn actived_count(&self) -> usize {
        self.ctx.actived_count.load(Ordering::SeqCst)
    }

    /// Get the number of pending tasks in the thread pool
    ///
    /// # Example
    /// ```rust
    /// use ezcfg_linker::ThreadPool;
    ///
    /// let pool = ThreadPool::global();
    ///
    /// for _ in 0..4 {
    ///    pool.execute(|| {
    ///      println!("Hello, world!");
    ///   });
    /// }
    ///
    /// pool.join();
    /// assert_eq!(pool.pending_count(), 0);
    /// ```
    pub fn pending_count(&self) -> usize {
        self.ctx.pending_count.load(Ordering::Relaxed)
    }

    /// Get the number of panicing threads in the thread pool
    ///
    /// # Examples
    ///
    /// ```
    /// use ezcfg_linker::ThreadPool;
    ///
    /// let pool = ThreadPool::global();
    /// for n in 0..10 {
    ///     pool.execute(move || {
    ///         // simulate a panic
    ///         if n % 2 == 0 {
    ///             panic!()
    ///         }
    ///     });
    /// }
    /// pool.join();
    ///
    /// assert_eq!(5, pool.panicing_count());
    /// ```
    pub fn panicing_count(&self) -> usize {
        self.ctx.packing_count.load(Ordering::Relaxed)
    }

    /// Execute a task in the thread pool
    /// The task will be executed in parallel
    ///
    /// # Example
    /// ```rust
    /// use ezcfg_linker::ThreadPool;
    ///
    /// let pool = ThreadPool::global();
    /// pool.execute(|| {
    ///   println!("Hello, world!");
    /// });
    /// ```
    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.ctx.pending_count.fetch_add(1, Ordering::SeqCst);
        self.sender.send(Message::NewJob(Box::new(job))).unwrap();
    }

    /// Shutdown the thread pool
    ///
    /// All pending tasks will be executed and the thread pool will be closed
    ///
    /// # Example
    /// ```rust
    /// use ezcfg_linker::ThreadPool;
    ///
    /// let pool = ThreadPool::global();
    /// pool.shutdown();
    /// ```
    pub fn shutdown(&self) {
        if !self.ctx.has_works() {
            return;
        }

        let actived_count = self.ctx.actived_count.load(Ordering::SeqCst);
        for _ in 0..actived_count {
            self.sender.send(Message::Shutdown).unwrap()
        }
    }

    /// Wait for all tasks to complete
    ///
    /// # Example
    /// ```rust
    /// use ezcfg_linker::ThreadPool;
    ///
    /// let pool = ThreadPool::global();
    /// pool.execute(|| {
    ///  println!("Hello, world!");
    /// });
    /// pool.join();
    /// ```
    pub fn join(&self) {
        if !self.ctx.has_works() {
            return;
        }

        let mut lock = self.ctx.cond_lock.lock().unwrap();
        while self.ctx.has_works() {
            lock = self.ctx.cond.wait(lock).unwrap();
        }
    }
}

pub fn spawn_in_pool(ctx: Arc<ThreadPoolContext>) {
    spawn(move || {
        // Create a sentinel to monitor the status of the thread
        let mut sentinel = Sentinel::new(&ctx);

        loop {
            // Release the mutex lock as soon as possible
            let msg = {
                let lock = ctx.receiver.lock().unwrap();
                match lock.recv() {
                    Ok(msg) => msg,
                    Err(_) => break,
                }
            };

            let job = match msg {
                Message::NewJob(job) => job,
                Message::Shutdown => break,
            };

            ctx.actived_count.fetch_add(1, Ordering::SeqCst);
            ctx.pending_count.fetch_sub(1, Ordering::SeqCst);

            job();

            ctx.actived_count.fetch_sub(1, Ordering::SeqCst);
            ctx.no_work_notify_all();
        }

        // Only when the internal code is executed correctly can it get here.
        // That means the thread is not in a panic state.
        //
        // If the thread is in a panic state, the sentinel will be dropped
        // and it will be spawned again.
        sentinel.done();
    });
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.shutdown();
        self.join();
    }
}

impl Clone for ThreadPool {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            ctx: self.ctx.clone(),
        }
    }
}

impl Debug for ThreadPool {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("ThreadPool")
            .field("pending_count", &self.pending_count())
            .field("actived_count", &self.actived_count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        thread::sleep,
        time::{Duration, Instant},
    };

    use super::*;

    #[test]
    fn it_should_create_successfully() {
        let _ = ThreadPool::new(4);
    }

    #[test]
    #[should_panic]
    fn it_should_panic_with_zero_size() {
        ThreadPool::new(0);
    }

    #[test]
    fn it_should_get_from_global_var() {
        let _ = ThreadPool::global();
    }

    // -----------------------------------------
    // ThreadPool::execute                     -
    // -----------------------------------------

    #[test]
    fn it_should_run_tasks_in_parallel() {
        let pool = ThreadPool::new(4);

        let task_count: usize = 10;
        let time_cost: usize = 100;

        let (sender, receiver) = channel::<usize>();
        let sender = Arc::new(sender);

        let start = Instant::now();

        for _ in 0..task_count {
            let sender = sender.clone();
            pool.execute(move || {
                sleep(Duration::from_millis(time_cost as u64));
                sender.send(1).unwrap();
            })
        }
        pool.join();

        let time = start.elapsed();

        assert!(time < Duration::from_millis((task_count * time_cost) as u64));
        assert_eq!(receiver.iter().take(task_count).sum::<usize>(), task_count);
    }

    #[test]
    fn it_should_run_large_num_of_tasks_in_parallel() {
        let pool = ThreadPool::global();

        let task_count: usize = 1_000;

        let (sender, receiver) = channel::<usize>();
        let sender = Arc::new(sender);

        for _ in 0..task_count {
            let sender = sender.clone();
            pool.execute(move || {
                sender.send(1).unwrap();
            })
        }
        pool.join();

        assert_eq!(receiver.iter().take(task_count).sum::<usize>(), task_count);
    }

    // -----------------------------------------
    // ThreadPool::join                        -
    // -----------------------------------------

    #[test]
    fn it_should_work_with_repeate_join() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..50 {
            let test_count = counter.clone();
            pool.execute(move || {
                test_count.fetch_add(1, Ordering::Release);
            });
        }
        pool.join();
        assert_eq!(50, counter.load(Ordering::Acquire));

        for _ in 0..50 {
            let test_count = counter.clone();
            pool.execute(move || {
                test_count.fetch_add(1, Ordering::Relaxed);
            });
        }
        pool.join();
        assert_eq!(100, counter.load(Ordering::Relaxed));
    }

    // -----------------------------------------
    // Sentinel                                -
    // -----------------------------------------

    #[test]
    fn it_should_recovery_from_sub_thread_panic() {
        let pool = ThreadPool::new(8);

        for _ in 0..8 {
            pool.execute(move || panic!(""));
        }
        pool.join();

        assert_eq!(pool.panicing_count(), 8);

        let (tx, rx) = channel();
        for _ in 0..8 {
            let tx = tx.clone();
            pool.execute(move || {
                tx.send(1).unwrap();
            });
        }

        assert_eq!(rx.iter().take(8).fold(0, |acc, num| acc + num), 8);
    }

    // -----------------------------------------
    // Traits                                  -
    // -----------------------------------------

    #[test]
    fn it_should_implement_debug() {
        let pool = ThreadPool::new(4);
        let debug = format!("{:?}", pool);
        assert_eq!(debug, "ThreadPool { pending_count: 0, actived_count: 0 }");

        let pool = ThreadPool::new(4);
        pool.execute(move || sleep(Duration::from_secs(5)));
        sleep(Duration::from_secs(1));
        let debug = format!("{:?}", pool);
        assert_eq!(debug, "ThreadPool { pending_count: 0, actived_count: 1 }");
    }

    #[test]
    fn it_should_implement_clone() {
        let pool = ThreadPool::new(4);

        for _ in 0..8 {
            pool.execute(move || {
                sleep(Duration::from_secs(2));
            });
        }

        let t1 = {
            let pool = pool.clone();
            spawn(move || {
                // wait for the first batch of tasks to finish
                pool.join();

                let (tx, rx) = channel();
                for i in 0..8 {
                    let tx = tx.clone();
                    pool.execute(move || {
                        tx.send(i).unwrap();
                    });
                }
                drop(tx);
                rx.iter().fold(0, |acc, num| acc + num)
            })
        };
        let t2 = {
            let pool = pool.clone();
            spawn(move || {
                // wait for the first batch of tasks to finish
                pool.join();

                let (tx, rx) = channel();
                for i in 1..8 {
                    let tx = tx.clone();
                    pool.execute(move || {
                        tx.send(i).unwrap();
                    });
                }
                drop(tx);
                rx.iter().fold(1, |acc, num| acc * num)
            })
        };

        assert_eq!(28, t1.join().unwrap());
        assert_eq!(5040, t2.join().unwrap());
    }

    #[test]
    fn it_should_implement_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ThreadPool>();
    }
}
