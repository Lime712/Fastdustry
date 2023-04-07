use std::collections::HashSet;
use arc::arc_core::math::geom::vec2::Vec2;
use arc::arc_core::util::command_handler::{CommandHandler, CommandResponse};
use arc::arc_core::util::interval::Interval;
use crate::game::team::Team;
use crate::gen::player::Player;
use crate::vars::{STATE, TILESIZE};
use crate::net::Administration;

/// note that snapshots are compressed, so the max snapshot size here is above the typical UDP safe limit
const MAX_SNAPSHOT_SIZE: usize = 800;
const TIMER_BLOCK_SYNC: usize = 0;
const TIMER_HEALTH_SYNC: usize = 1;
const BLOCK_SYNC_TIME: f32 = 60.0 * 6.0;
const HEALTH_SYNC_TIME: f32 = 30.0;
static mut HIDDEN_IDS: HashSet<i32> = HashSet::new();
static mut HEALTH_HASH_SET: HashSet<i32> = HashSet::new();
static mut VECTOR: Vec2 = Vec2::new(0.0, 0.0);
/// If a player goes away of their server-side coordinates by this distance, they get teleported back.
const CORRECT_DIST: f32 = TILESIZE * 14.0;

pub struct NetServer {
    pub admins: Administration,
    pub client_commands: CommandHandler, // prefix: "/"
    pub assigner: dyn TeamAssigner,
    /// Converts a message + NULLABLE player sender into a single string. Override for custom prefixes/suffixes.
    pub chat_formatter: dyn ChatFormatter,
    /// Handles an incorrect command response. Returns text that will be sent to player. Override for customisation.
    pub invalid_handler: dyn InvalidCommandHandler,
    closing: bool,
    pvp_auto_paused: bool,
    timer: Interval,
    build_health_changed :HashSet<i32>
    // todo: the rest
}

impl Default for NetServer {
    fn default() -> Self {
        Self {
            admins: Administration::default(),
            client_commands: CommandHandler::default(),
            // todo fix this
            assigner: |player, players| {
                if unsafe { STATE.unwrap().rules.pvp } {
                    //find team with minimum amount of players and auto-assign player to that.
                    let re = unsafe {STATE.unwrap().teams.get_active().min}
                }
            },
            chat_formatter: |player, message| {
                if let Some(player) = player {
                    format!("[{}]: {}", player.name, message)
                } else {
                    format!("[SERVER]: {}", message)
                }
            },
            invalid_handler: DefaultInvalidCommandHandler,
            closing: false,
            pvp_auto_paused: false,
            timer: Interval::default(),
            build_health_changed: HashSet::new(),
        }
    }
}

pub trait TeamAssigner {
    fn assign(&mut self, player: &mut Player, players: &mut Vec<Player>) -> Team;
}

pub trait ChatFormatter {
    /// return text to be placed before player name
    fn format(&mut self, player: Option<&Player>, message: &str) -> String;
}

pub trait InvalidCommandHandler {
    fn handle(&mut self, player: &mut Player, response: CommandResponse) -> String;
}