use std::path::PathBuf;

const CORES: usize = num_cpus::get();
const NAME: String = whoami::username();
const USER_HOME: PathBuf = home::home_dir().unwrap();
const OS_NAME: String = whoami::distro();
const OS_ARCH: String = whoami::arch().to_string();
const IS_WINDOWS: bool = cfg!(windows);
const IS_MAC: bool = cfg!(macos);
const IS_LINUX: bool = cfg!(linux);
const IS_ANDROID: bool = cfg!(target_os = "android");
const IS_IOS: bool = cfg!(target_os = "ios");
const IS_WEB: bool = cfg!(target_os = "web");
const IS_WASM: bool = cfg!(target_os = "wasm");
const IS_UNIX: bool = cfg!(unix);
const IS_ARM: bool = cfg!(target_arch = "arm");
const IS_ARM64: bool = cfg!(target_arch = "arm64");
const IS_X86: bool = cfg!(target_arch = "x86");
const IS_X86_64: bool = cfg!(target_arch = "x86_64");
const IS_32_BIT: bool = cfg!(target_pointer_width = "32");
const IS_64_BIT: bool = cfg!(target_pointer_width = "64");


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
