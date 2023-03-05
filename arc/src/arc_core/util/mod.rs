pub mod log;
pub mod time;
pub mod task_queue;
pub mod os;

/// A trait for objects that can be disposed
pub trait Disposable {
    /// Releases all resources
    fn dispose(&mut self);
    fn is_disposed(&self) -> bool { false }
}
