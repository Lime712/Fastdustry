use std::path::PathBuf;

use lazy_static::lazy_static;

lazy_static! {
    static ref CORES: usize = num_cpus::get();
    static ref NAME: String = whoami::username();
    static ref USER_HOME: PathBuf = home::home_dir().unwrap();
    static ref OS_NAME: String = whoami::distro();
    static ref OS_ARCH: String = whoami::arch().to_string();
}
pub const IS_WINDOWS: bool = cfg!(windows);
pub const IS_MAC: bool = cfg!(macos);
pub const IS_LINUX: bool = cfg!(linux);
pub const IS_ANDROID: bool = cfg!(target_os = "android");
pub const IS_IOS: bool = cfg!(target_os = "ios");
pub const IS_WEB: bool = cfg!(target_os = "web");
pub const IS_WASM: bool = cfg!(target_os = "wasm");
pub const IS_UNIX: bool = cfg!(unix);
pub const IS_ARM: bool = cfg!(target_arch = "arm");
pub const IS_ARM64: bool = cfg!(target_arch = "arm64");
pub const IS_X86: bool = cfg!(target_arch = "x86");
pub const IS_X86_64: bool = cfg!(target_arch = "x86_64");
pub const IS_32_BIT: bool = cfg!(target_pointer_width = "32");
pub const IS_64_BIT: bool = cfg!(target_pointer_width = "64");

pub fn get_app_data_directory(app_name: String) -> PathBuf {
    if IS_WINDOWS {
        let mut path = USER_HOME.clone();
        path.push("AppData");
        path.push("Roaming");
        path.push(app_name);
        path
    } else if IS_MAC {
        let mut path = USER_HOME.clone();
        path.push("Library");
        path.push("Application Support");
        path.push(app_name);
        path
    } else if IS_LINUX {
        let mut path = USER_HOME.clone();
        path.push(".local");
        path.push("share");
        path.push(app_name);
        path
    } else {
        panic!("Unsupported operating system");
    }
}

pub fn has_env_var(var: String) -> bool {
    std::env::var(var).is_ok()
}

pub fn get_env_var(var: String) -> String {
    std::env::var(var).unwrap()
}
