use std::{
    sync::{atomic::Ordering, Arc},
    thread::panicking,
};

use super::{ctx::ThreadPoolContext, thread_pool::spawn_in_pool};

pub struct Sentinel<'ctx> {
    active: bool,
    ctx: &'ctx Arc<ThreadPoolContext>,
}

impl<'ctx> Sentinel<'ctx> {
    pub fn new(ctx: &'ctx Arc<ThreadPoolContext>) -> Self {
        Sentinel { active: true, ctx }
    }

    pub fn done(&mut self) {
        self.active = false
    }
}

impl Drop for Sentinel<'_> {
    fn drop(&mut self) {
        // `active == false` means that the thread has being shut down
        // just return
        if !self.active {
            return;
        }

        self.ctx.actived_count.fetch_sub(1, Ordering::SeqCst);
        if panicking() {
            self.ctx.packing_count.fetch_add(1, Ordering::SeqCst);
        }

        // Spawn a new thread to replace the current one
        self.ctx.no_work_notify_all();
        spawn_in_pool(self.ctx.clone())
    }
}
