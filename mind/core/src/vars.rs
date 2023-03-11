// use crate::ctype::content_type::ContentType;

use lazy_static::lazy_static;
use std::string::ToString;
use std::sync::Mutex;

use crate::maps::map::Map;
use arc::arc_core::core::SETTINGS;
use arc::arc_core::settings::Value;
use arc::{get_settings, info};
use crate::core::game_state::GameState;
use crate::world::meta::block_enums::Env;

pub static FAILED_TO_LAUNCH: bool = false;
pub static LOAD_LOCALES: bool = true;
pub static LOADED_LOGGER: bool = false;
pub static LOADED_FILE_LOGGER: bool = false;
pub static EXPERIMENTAL: bool = false;
pub static APP_NAME: &str = "Mindustry";
pub static HEADLESS: bool = true;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub static mut DATA_DIRECTORY: &str = "./";
pub static mut STATE: Option<GameState> = Some(GameState::default());
pub static DEFAULT_ENV: Env = Env::Terrestrial;
lazy_static! {
    pub static ref CUSTOM_MAP_DIRECTORY: Mutex<String> = Mutex::new("./".to_string());
    pub static ref SAVE_DIRECTORY: Mutex<String> = Mutex::new("./".to_string());
    pub static ref MOD_DIRECTORY: Mutex<String> = Mutex::new("./".to_string());
    pub static ref EMPTY_MAP: Map<'static> = Map::default();
}
// pub static DEFAULT_CONTENT_ICONS: Vec<ContentType> = vec![
//     ContentType::Item,
//     ContentType::Liquid,
//     ContentType::block,
//     ContentType::unit,
// ];

pub fn load_settings() {
    let mut settings;
    unsafe {
        settings = match SETTINGS {
            Some(ref mut s) => s,
            None => panic!("Settings not initialized"),
        };
    }
    settings.app_name = APP_NAME.to_string();
    settings.default("locale".to_string(), Value::String("default".to_string()));
    settings.default("blocksync".to_string(), Value::Bool(true));
    settings.should_auto_save = false;
    settings.load();
}

pub fn init() {
    // Groups.init();

    info!("[Mindustry] Version: {}", VERSION);
    let settings;
    get_settings!(settings);
    unsafe {
        DATA_DIRECTORY = settings.data_directory.as_str();
        *CUSTOM_MAP_DIRECTORY.lock().unwrap() = DATA_DIRECTORY.clone().to_owned() + "maps/";
        *SAVE_DIRECTORY.lock().unwrap() = DATA_DIRECTORY.clone().to_owned() + "saves/";
        *MOD_DIRECTORY.lock().unwrap() = DATA_DIRECTORY.clone().to_owned() + "mods/";

        // todo: load mods
    }
}
