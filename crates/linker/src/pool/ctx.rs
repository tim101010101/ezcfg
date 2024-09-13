use std::sync::{
    atomic::{AtomicUsize, Ordering},
    mpsc::Receiver,
    Condvar, Mutex,
};

use super::thread_pool::Action;

pub struct ThreadPoolContext {
    pub receiver: Mutex<Receiver<Action>>,

    pub actived_count: AtomicUsize,
    pub pending_count: AtomicUsize,
    pub packing_count: AtomicUsize,

    pub cond_lock: Mutex<()>,
    pub cond: Condvar,
}

impl ThreadPoolContext {
    pub fn has_works(&self) -> bool {
        let actived_count = self.actived_count.load(Ordering::SeqCst);
        let pendding_count = self.pending_count.load(Ordering::SeqCst);
        actived_count > 0 || pendding_count > 0
    }

    pub fn no_work_notify_all(&self) {
        // Lock the current thread
        // then notify the waiting threads
        let _lock = self.cond_lock.lock().unwrap();
        self.cond.notify_all();
    }
}
