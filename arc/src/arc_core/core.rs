use crate::arc_core::application::Application;
use crate::arc_core::settings::Settings;
use lazy_static::lazy_static;

pub static mut APP: Option<&dyn Application> = None;
// pub graphics: Graphics,
// pub audio: Audio,
// pub input: Input,
// pub files: Files,
// lazy_static! {
pub static mut SETTINGS: Option<Settings> = None;
// }
// pub keyBinds: Keybinds = new KeyBinds(),

// pub bundle: I18NBundle  = I18NBundle.createEmptyBundle(),
// pub camera: Camera,
// pub batch: Batch,
// pub scene: Scene,
// // pub assets: AssetManager,
// pub atlas: TextureAtlas,
// //
// pub gl: GL20,
// pub gl20: GL20,
// pub gl30: GL30,
