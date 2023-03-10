use arc::arc_core::application_listener::ApplicationListener;
use arc::arc_core::backend::headless::HeadlessApplication;
use arc::arc_core::settings::Settings;
use arc::{debug, get_settings, info};

pub static ROUND_EXTRA_TIME: i32 = 12;
pub static MAX_LOG_LENGTH: i32 = 1024 * 1024 * 5;

static mut ARGS: Vec<String> = Vec::new();
static mut START_TIME: i64 = 0;

fn main() {
    info!("ServerLauncher main");
    debug!("Debugging enabled");
    unsafe {
        ARGS = std::env::args().collect();
        START_TIME = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64;
    }
    HeadlessApplication::start(Box::new(ServerLauncher::new()), 1.0 / 60.0);
}

pub struct ServerControl {
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
            let settings;
            get_settings!(settings);
            settings.set_data_directory("./config".to_string());
            // settings.load_values();
            // settings.set_string("test".to_string(), "test".to_string());
            // settings.set_float("test2".to_string(), 1.0);
            // settings.set_bool("test3".to_string(), true);
            // settings.set_int("test4".to_string(), 1);
            // debug!("settings: {}", settings.to_string());
            // debug!("{}", settings.data_directory);
            // // HEAD_LOCALS = false;
            // settings.save_values();
            // settings.save_values();
            core::vars::load_settings();
            core::vars::init();
            settings.save_values();

            // print how long it took to load
            let end_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as i64;
            let time = end_time - START_TIME;
            info!(
                "ServerLauncher init complete in {:.2} ms",
                time as f64 / 1000000.0
            );
        }
    }
}
