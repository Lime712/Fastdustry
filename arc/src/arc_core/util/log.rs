use std::fmt;

use chrono::prelude::*;

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    None,
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => ({
        if $level as usize <= $crate::util::log::LOG_LEVEL as usize {
            $crate::util::log::log($level, format_args!($($arg)*));
        }
    })
}

#[cfg(debug_assertions)]
static mut DEBUG: bool = true;

#[cfg(not(debug_assertions))]
static mut DEBUG: bool = false;

pub fn set_debug(debug: bool) {
    unsafe {
        DEBUG = debug;
    }
}

pub fn is_debug() -> bool {
    unsafe { DEBUG }
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug {
    ($($arg:tt)*) => ($crate::arc_core::util::log::log($crate::arc_core::util::log::LogLevel::Debug, format_args!($($arg)*));)
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        debug!("[TRACE]: [{}:{}] {}\x1B[0m", file!(), line!(), format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ($crate::arc_core::util::log::log($crate::arc_core::util::log::LogLevel::Info, format_args!($($arg)*));)
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (arc::arc_core::util::log::log(arc::arc_core::util::log::LogLevel::Warn, format_args!($($arg)*));)
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (arc::arc_core::util::log::log(arc::arc_core::util::log::LogLevel::Error, format_args!($($arg)*));)
}

/// This function does not check if debug is enabled it just prints the message
pub fn log(level: LogLevel, args: fmt::Arguments) {
    let t = get_current_time_string();
    let default = "\x1B[0m".to_string();
    match level {
        LogLevel::Debug => {
            if unsafe { DEBUG } {
                println!("[{}] \x1B[36m[D] {}{}", t, args, default)
            }
        }
        LogLevel::Info => println!("[{}] [I] {}{}", t, args, default),
        LogLevel::Warn => println!("[{}] \x1B[33m[W] {}{}", t, args, default),
        LogLevel::Error => println!("[{}] \x1B[31m[E] {}{}", t, args, default),
        LogLevel::None => {}
    }
}

pub fn get_current_time_string() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
