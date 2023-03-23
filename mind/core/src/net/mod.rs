use std::collections::{HashMap, HashSet};
use arc::arc_core::util::interval::Interval;
use arc::arc_core::util::{RateKeeper, string};

/// Handles chat messages from players and changes their contents.
pub trait ChatFilter {
    /// return the filtered message; None signals that the message should not be sent.
    fn filter(&self, player: &Player, message: &str) -> Option<String>;
}

/// Allows or disallows player actions.
pub trait ActionFilter {
    /// return whether this action should be permitted. if applicable, make sure to send this player a message specify why the action was prohibited.
    fn allow(&self, action: PlayerAction) -> bool;
}

pub struct PlayerInfo {
    pub id: String,
    pub last_name: String,
    pub last_ip: String,
    pub ips: Vec<String>,
    pub names: Vec<String>,
    pub admin_usid: String,
    pub times_kicked : i32,
    pub times_joined : i32,
    pub banned: bool,
    pub admin: bool,
    /// last kicked time to expiration
    pub last_kick: i64,
    pub last_message_time: i64,
    pub last_sync_time: i64,
    pub last_sent_message: String,
    pub message_infractions: i32,
    pub rate: RateKeeper,
    pub message_timer: Interval,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            last_name: "<unknown>".to_string(),
            last_ip: "<unknown>".to_string(),
            ips: Vec::new(),
            names: Vec::new(),
            admin_usid: String::new(),
            times_kicked: 0,
            times_joined: 0,
            banned: false,
            admin: false,
            last_kick: 0,
            last_message_time: 0,
            last_sync_time: 0,
            last_sent_message: String::new(),
            message_infractions: 0,
            rate: RateKeeper::new(),
            message_timer: Interval::new(0),
        }
    }
}

impl PlayerInfo {
    pub fn new(id: String) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn plain_last_name(&self) -> String {
        string::strip_colors(&self.last_name)
    }
}

pub struct Administration {
    pub banned_ips: HashSet<String>,
    pub whitelist: HashSet<String>,
    pub chat_filters: HashSet<dyn ChatFilter>,
    pub action_filters: HashSet<dyn ActionFilter>,
    pub subnet_bans: HashSet<String>,
    pub dos_black_list: HashSet<String>,
    pub kicked_ips: HashMap<String, i64>,
    modified: bool,
    loaded: bool,
    /// All player info. Maps UUIDs to info. This persists throughout restarts. Do not access directly.
    pub player_info: HashMap<String, PlayerInfo>,
}