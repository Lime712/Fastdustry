pub mod command_handler;
pub mod interval;
pub mod log;
pub mod os;
pub mod string;
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

/// Keeps track of X actions in Y units of time.
pub struct RateKeeper {
    pub occurrences: u32,
    pub last_time: u128,
}

impl Default for RateKeeper {
    fn default() -> Self {
        Self {
            occurrences: 0,
            last_time: 0,
        }
    }
}

impl RateKeeper {
    pub fn new() -> Self {
        Self::default()
    }

    /// return whether an action is allowed.
    /// # Arguments
    /// * `spacing` - The spacing between action chunks in milliseconds
    /// * `cap` - The maximum amount of actions per chunk
    pub fn allow(&mut self, spacing: u128, cap: u32) -> bool {
        if time::time_since_millis(&self.last_time) > spacing {
            self.occurrences = 0;
            self.last_time = time::millis();
        }
        self.occurrences += 1;
        self.occurrences <= cap
    }
}
