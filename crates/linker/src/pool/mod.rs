//! This mod is a simple thread pool implementation.
//!
//! Inspired by [rust-threadpool](https://github.com/rust-threadpool/rust-threadpool),
//! I don't need most of the other functions, so I simplified its implementation.

mod ctx;
mod sentinel;
mod thread_pool;

pub use thread_pool::ThreadPool;
