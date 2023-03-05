use arc::arc_core::application::ApplicationType::Headless;
use arc::arc_core::application_listener::ApplicationListener;
use arc::arc_core::backend::headless::HeadlessApplication;
use arc::{debug, info};
use std::fs::File;
use std::thread::Thread;
use arc::arc_core::settings::Settings;

static ROUND_EXTRA_TIME: i32 = 12;
static MAX_LOG_LENGTH: i32 = 1024 * 1024 * 5;

static mut ARGS: Vec<String> = Vec::new();

fn main() {
    println!("Hello, world!");
    debug!("hello");
    unsafe {
        ARGS = std::env::args().collect();
    }
    HeadlessApplication::start(Box::new(ServerLauncher::new()), 1.0 / 60.0);
}

struct ServerControl {
    // pub handler: CommandHandler,
    // pub log_folder: File,
    // current_log_file: File,
    // in_game_over_wait: bool,
    // last_task: Option<Task>,
    // last_mode: Gamemode,
    // // next_map_override: Option<Map>,
    // auto_save_count: Interval,
    //
    // socket_thread: Thread,
    // server_socket: ServerSocket,
    // socket_output: SocketOutput,
    // suggested: String,
    // auto_paused: bool,
}

pub struct ServerLauncher {}

impl ServerLauncher {
    pub fn new() -> ServerLauncher {
        ServerLauncher {}
    }
}

impl ApplicationListener for ServerLauncher {
    fn init(&self) {
        info!("ServerLauncher init");
        use arc::arc_core::core::*;
        unsafe {
            SETTINGS = Some(Settings::new());
            let mut settings = match SETTINGS {
                Some(ref mut s) => s,
                None => panic!("Settings not initialized"),
            };
            settings.set_data_directory("./config".to_string());
            debug!("{}", settings.data_directory);
            // HEAD_LOCALS = false;
            settings.save_values();
        }
    }
}
