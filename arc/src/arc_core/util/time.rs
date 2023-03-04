use std::collections::HashSet;
use std::hash::Hash;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::arc_core::func::FloatP;

/// Conversion factors for ticks to other unit values.
pub const TO_SECONDS: f64 = 60.0;
pub const TO_MINUTES: f64 = 60.0 * 60.0;
pub const TO_HOURS: f64 = 60.0 * 60.0 * 60.0;

/// Global delta value. Do not change.
pub static mut DELTA: f64 = 1.0;
/// Global time values. Do not change.
pub static mut TIME: f64 = 0.0;
pub static mut GLOBAL_TIME: f64 = 0.0;
pub const NANOS_PER_MILLI: i64 = 1_000_000;

pub static mut TIME_RAW: f64 = 0.0;
const GLOBAL_TIME_RAW: f64 = 0.0;
lazy_static! {
    static ref RUNS: Arc<Mutex<HashSet<DelayRun>>> = Arc::new(Mutex::new(HashSet::new()));
    static ref DELTAIMPL: Arc<Mutex<FloatP>> = Arc::new(Mutex::new(FloatP::new(|| { 0.0 }))); // Math.min(Core.graphics.getDeltaTime() * 60f, 3f)
}
static mut MARKS: Vec<i64> = Vec::new();


/// Runs a task with a delay of several ticks. If Time.clear() is called, this task will be cancelled.
pub fn run(delay: i64, finish: Box<dyn Fn() + Send + 'static>) {
    unsafe {
        RUNS.lock().unwrap().insert(DelayRun::new(delay, finish));
    }
}

/// Runs a task with a delay of several ticks. Unless the application is closed, this task will always complete.
pub fn runTask(delay: i64, finish: Box<dyn Fn()>) {
    todo!()
}

pub fn mark() {
    unsafe {
        MARKS.push(nanos());
    }
}

/// A value of -1 means mark() wasn't called beforehand.
pub fn elapsed() -> f64 {
    return if unsafe { MARKS.len() } == 0 {
        -1.0
    } else {
        time_since_nanos(&unsafe { MARKS.pop().unwrap() }) as f64 / 1_000_000.0
    };
}

pub fn update_global() {
    unsafe {
        // GLOBAL_TIME_RAW += crate::arc_core::graphics.get_delta_time() * 60.0;
        DELTA = DELTAIMPL.lock().unwrap().run();

        if DELTA.is_finite() || DELTA.is_nan() {
            TIME_RAW = 0.0;
        }

        TIME = TIME_RAW;
        GLOBAL_TIME = GLOBAL_TIME_RAW;
    }
}

pub fn update() {
    unsafe {
        TIME_RAW += DELTA;

        if TIME_RAW.is_finite() || TIME_RAW.is_nan() {
            TIME_RAW = 0.0;
        }

        TIME = TIME_RAW;
        GLOBAL_TIME = GLOBAL_TIME_RAW;

        // run delayed tasks
        for run in RUNS.lock().unwrap().iter() {
            if run.delay <= 0 {
                (run.finish)();
            }
        }
    }
}

pub fn clear() {
    unsafe {
        RUNS.lock().unwrap().clear();
    }
}

pub fn set_delta_provider(provider: FloatP) {
    unsafe {
        *DELTAIMPL.lock().unwrap() = provider;
        DELTA = DELTAIMPL.lock().unwrap().run();
    }
}

/// return The current value of the system timer, in nanoseconds.
pub fn nanos() -> i64 {
    return std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as i64;
}

/// return The time in milliseconds
pub fn millis() -> i64 {
    return std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64;
}

pub fn nanos_to_millis(nanos: i64) -> i64 {
    return nanos / NANOS_PER_MILLI;
}

pub fn millis_to_nanos(millis: i64) -> i64 {
    return millis * NANOS_PER_MILLI;
}

pub fn time_since_nanos(prev: &i64) -> i64 {
    return nanos() - prev;
}

pub fn time_since_millis(prev: &i64) -> i64 {
    return millis() - prev;
}

pub struct DelayRun {
    pub delay: i64,
    pub finish: Box<dyn Fn() + Send + 'static>,
}

impl DelayRun {
    pub fn new(delay: i64, finish: Box<dyn Fn() + Send + 'static>) -> DelayRun {
        DelayRun {
            delay,
            finish,
        }
    }
}

impl Eq for DelayRun {}

impl Hash for DelayRun {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.delay.hash(state);
    }
}

impl PartialEq for DelayRun {
    fn eq(&self, other: &Self) -> bool {
        self.delay == other.delay
    }
}