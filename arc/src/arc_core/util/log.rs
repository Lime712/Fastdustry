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

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => (arc::arc_core::util::log::log(arc::arc_core::util::log::LogLevel::Debug, format_args!($($arg)*));)
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => (arc::arc_core::util::log::log(arc::arc_core::util::log::LogLevel::Info, format_args!($($arg)*));)
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (arc::arc_core::util::log::log(arc::arc_core::util::log::LogLevel::Warn, format_args!($($arg)*));)
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (arc::arc_core::util::log::log(arc::arc_core::util::log::LogLevel::Error, format_args!($($arg)*));)
}

pub fn log(level: LogLevel, args: fmt::Arguments) {
    let t = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let default = "\x1B[0m".to_string();
    match level {
        LogLevel::Debug => println!("[{}] \x1B[36m[D] {}{}", t, args, default),
        LogLevel::Info => println!("[{}] [I] {}{}", t, args, default),
        LogLevel::Warn => println!("[{}] \x1B[33m[W] {}{}", t, args, default),
        LogLevel::Error => println!("[{}] \x1B[31m[E] {}{}", t, args, default),
        LogLevel::None => {}
    }
}