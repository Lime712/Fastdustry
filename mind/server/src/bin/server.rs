use std::fs::File;

static ROUND_EXTRA_TIME: i32 = 12;
static MAX_LOG_LENGTH: i32 = 1024 * 1024 * 5;

static mut ARGS: Vec<String> = Vec::new();

fn main() {
    println!("Hello, world!");
    unsafe {
        ARGS = std::env::args().collect();
    }
}

struct ServerControl {
    pub handler: CommandHandler,
    pub log_folder: File,
    current_log_file: File,
    in_game_over_wait: bool,
    last_task: Option<Task>,
    last_mode: Gamemode,
    next_map_override: Option<Map>,
    auto_save_count: Interval,

    socket_thread: Thread,
    server_socket: ServerSocket,
    socket_output: SocketOutput,
    suggested: String,
    auto_paused: bool,
}