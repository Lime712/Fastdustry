pub mod log;
pub mod os;
pub mod task_queue;
pub mod time;

/// A trait for objects that can be disposed
pub trait Disposable {
    /// Releases ALL resources
    fn dispose(&mut self);
    fn is_disposed(&self) -> bool {
        false
    }
}
