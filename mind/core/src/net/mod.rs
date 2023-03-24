pub mod net;
pub mod packet;

use std::collections::{HashMap, HashSet};

use arc::arc_core::util::{RateKeeper, string};
use arc::arc_core::util::interval::Interval;
use arc::arc_core::util::time::{millis, time_since_millis};

use crate::gen::player::Player;
use crate::net::config::{ANTI_SPAM, INTERACT_RATE_LIMIT, INTERACT_RATE_WINDOW, MESSAGE_RATE_LIMIT, MESSAGE_SPAM_KICK};
use crate::r#type::item::Item;
use crate::world::block::Block;

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
    pub times_kicked: i32,
    pub times_joined: i32,
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

impl Default for Administration {
    fn default() -> Self {
        Self {
            banned_ips: HashSet::new(),
            whitelist: HashSet::new(),
            chat_filters: HashSet::new(),
            action_filters: HashSet::new(),
            subnet_bans: HashSet::new(),
            dos_black_list: HashSet::new(),
            kicked_ips: HashMap::new(),
            modified: false,
            loaded: false,
            player_info: HashMap::new(),
        }
    }
}

impl Administration {
    pub fn new() -> Self {
        let mut a = Self::default();
        // a.load();

        // anti-spam
        struct SpamFilter;
        impl ChatFilter for SpamFilter {
            fn filter(&self, player: &Player, message: &str) -> Option<String> {
                let reset_time = config::MESSAGE_RATE_LIMIT.int() * 1_000;
                if ANTI_SPAM.bool() && !player.is_local() && !player.admin {
                    // prevent people from spamming messages quickly
                    if reset_time > 0 && time_since_millis(player.get_info().last_message_time) < reset_time as u128 {
                        // supress message
                        player.send_message("[scarlet]You may only send messages every " + MESSAGE_RATE_LIMIT.int() + " seconds.");
                        player.get_info().message_infractions += 1;
                        // kick player for spamming and prevent connection if they've dont this serveral times
                        if player.get_info().message_infractions >= MESSAGE_SPAM_KICK.int() && MESSAGE_SPAM_KICK.int() != 0 {
                            player.con.kick("You have been kicked for spamming.", 1000 * 60 * 2);
                        }
                        None
                    } else {
                        player.get_info().message_infractions = 0;
                    }

                    // prevent players from sending the same message twice in the span of 10 seconds
                    if message == player.get_info().last_sent_message && time_since_millis(player.get_info().last_message_time) < 10_000 {
                        player.send_message("[scarlet]You may not send the same message twice in a row.");
                        None
                    }
                    player.get_info().last_sent_message = message.to_string();
                    player.get_info().last_message_time = millis();
                }
                Some(message.parse().unwrap())
            }
        }

        // block interaction rate limit
        struct BlockFilter;
        impl ActionFilter for BlockFilter {
            fn allow(&self, action: PlayerAction) -> bool {
                if action.ty != ActionType::BreakBlock &&
                    action.ty != ActionType::PlaceBlock &&
                    action.ty != ActionType::CommandUnits &&
                    ANTI_SPAM.bool() {
                    let mut rate: RateKeeper = action.player.get_info().rate;
                    return if rate.allow((INTERACT_RATE_WINDOW.int() * 1000) as u128, INTERACT_RATE_LIMIT.int() as u32) {
                        true
                    } else {
                        if rate.occurrences > INTERACT_RATE_LIMIT.int() as u32 {
                            action.player.kick("You are interacting with too many blocks.", 1000 * 30);
                        } else if action.player.get_info().message_timer.get(60.0 * 2.0) {
                            action.player.send_message("[scarlet]You are interacting with blocks too quickly.");
                        }

                        false
                    };
                }
                true
            }
        }

        a.add_chat_filter(Box::new(SpamFilter));
        a.add_action_filter(Box::new(BlockFilter));
        a
    }

    pub fn add_chat_filter(&mut self, filter: Box<dyn ChatFilter>) {
        self.chat_filters.insert(filter);
    }

    pub fn add_action_filter(&mut self, filter: Box<dyn ActionFilter>) {
        self.action_filters.insert(filter);
    }
}

/// Defines a (potentially dangerous) action that a player has done in the world.
/// These objects are pooled; do not cache them! (idk if this is true, but its true in the java version)
/// todo: find out ^^^
pub struct PlayerAction<'a> {
    pub player: &'a Player,
    pub ty: ActionType,
    pub tile: Option<Tile>,

    /// valid for block placement events only
    pub block: Option<Block>,
    pub rotation: i32,

    /// valid for configure and rotation-type events only.
    /// todo: fix this, cause in java its an Object (tbh i hate that you can make a variable with type Object)
    pub config: Option<Vec<u8>>,

    /// valid for item-type events only.
    pub item: Option<Item>,
    pub item_amount: i32,

    /// valid for unit-type events only, and even in that case may be null.
    pub unit: Option<Unit>,

    /// valid only for removePlanned events only; contains packed positions.
    pub plans: Option<Vec<i32>>,

    /// valid only for command unit events
    pub unit_ids: Option<Vec<i32>>,

    /// valid only for command building events
    pub building_positions: Option<Vec<i32>>,
}

impl<'a> Default for PlayerAction<'a> {
    fn default() -> Self {
        Self {
            player: &Player::default(),
            ty: ActionType::BreakBlock,
            tile: None,
            block: None,
            rotation: 0,
            config: None,
            item: None,
            item_amount: 0,
            unit: None,
            plans: None,
            unit_ids: None,
            building_positions: None,
        }
    }
}

pub enum ActionType {
    BreakBlock,
    PlaceBlock,
    Rotate,
    Configure,
    WithdrawItem,
    DepositItem,
    Control,
    BuildSelect,
    Command,
    RemovePlanned,
    CommandUnits,
    CommandBuilding,
    Respawn,
}

impl<'a> PlayerAction<'a> {
    pub fn set(&mut self, player: &'a Player, ty: ActionType, tile: Tile) -> &mut Self {
        self.player = player;
        self.ty = ty;
        self.tile = Some(tile);
        self
    }

    pub fn reset(&mut self) {
        self = &mut PlayerAction::default();
    }
}

mod config {
    use arc::arc_core::core::SETTINGS;
    use arc::arc_core::settings::Value;
    use arc::arc_core::util::log::{LogLevel, set_debug};

    use crate::vars;

    pub struct Config {
        pub default_value: Value,
        pub name: &'static str,
        pub description: &'static str,
        pub key: Option<&'static str>,
        changed: Option<Box<dyn Fn()>>,
    }

    pub static mut ALL_CONFIGS: Vec<&Config> = Vec::new();

    // all config values
    pub static SERVER_NAME: Config = Config::new("name", "The server name as displayed on clients.", Value::String("Server".to_string()), Some("server.name"), None);
    pub static DESC: Config = Config::new("desc", "The server description, displayed under the name. Max 100 characters.", Value::String("off".to_string()), None, None);
    pub static PORT: Config = Config::new("port", "The port to host on.", Value::Int(vars::PORT), None, None);
    pub static AUTO_UPDATE: Config = Config::new("autoUpdate", "Whether to auto-update and exit when a new bleeding-edge update arrives.", Value::Bool(false), None, None);
    pub static SHOW_CONNECT_MESSAGES: Config = Config::new("showConnectMessages", "Whether to display connect/disconnect messages.", Value::Bool(true), None, None);
    pub static ENABLE_VOTEKICK: Config = Config::new("enableVotekick", "Whether votekick is enabled.", Value::Bool(true), None, None);
    pub static START_COMMANDS: Config = Config::new("startCommands", "Commands run at startup. This should be a comma-separated list.", Value::String("".to_string()), None, None);
    pub static LOGGING: Config = Config::new("logging", "Whether to log everything to files.", Value::Bool(true), None, None);
    pub static STRICT: Config = Config::new("strict", "Whether strict mode is on - corrects positions and prevents duplicate UUIDs.", Value::Bool(true), None, None);
    pub static ANTI_SPAM: Config = Config::new("antiSpam", "Whether spammers are automatically kicked and rate-limited.", Value::Bool(vars::HEADLESS), None, None);
    pub static INTERACT_RATE_WINDOW: Config = Config::new("interactRateWindow", "Block interaction rate limit window, in seconds.", Value::Int(6), None, None);
    pub static INTERACT_RATE_LIMIT: Config = Config::new("interactRateLimit", "Block interaction rate limit.", Value::Int(25), None, None);
    pub static INTERACT_RATE_KICK: Config = Config::new("interactRateKick", "How many times a player must interact inside the window to get kicked.", Value::Int(60), None, None);
    pub static MESSAGE_RATE_LIMIT: Config = Config::new("messageRateLimit", "Message rate limit in seconds. 0 to disable.", Value::Int(0), None, None);
    pub static MESSAGE_SPAM_KICK: Config = Config::new("messageSpamKick", "How many times a player must send a message before the cooldown to get kicked. 0 to disable.", Value::Int(3), None, None);
    pub static PACKET_SPAM_LIMIT: Config = Config::new("packetSpamLimit", "Limit for packet count sent within 3sec that will lead to a blacklist + kick.", Value::Int(300), None, None);
    pub static CHAT_SPAM_LIMIT: Config = Config::new("chatSpamLimit", "Limit for chat packet count sent within 2sec that will lead to a blacklist + kick. Not the same as a rate limit.", Value::Int(20), None, None);
    pub static SOCKET_INPUT: Config = Config::new("socketInput", "Allows a local application to control this server through a local TCP socket.", Value::Bool(false), Some("socket"), Some(Box::new(|| Events::fire(Trigger::SocketConfigChanged))));
    pub static SOCKET_INPUT_PORT: Config = Config::new("socketInputPort", "The port for socket input.", Value::Int(6859), Some("socket-port"), Some(Box::new(|| Events::fire(Trigger::SocketConfigChanged))));
    pub static SOCKET_INPUT_ADDRESS: Config = Config::new("socketInputAddress", "The bind address for socket input.", Value::String("localhost".to_string()), Some("socket-address"), Some(Box::new(|| Events::fire(Trigger::SocketConfigChanged))));
    pub static ALLOW_CUSTOM_CLIENTS: Config = Config::new("allowCustomClients", "Whether custom clients are allowed to connect.", Value::Bool(!vars::HEADLESS), Some("allow-custom"), None);
    pub static WHITELIST: Config = Config::new("whitelist", "Whether the whitelist is used.", Value::Bool(false), None, None);
    pub static MOTD: Config = Config::new("motd", "The message displayed to people on connection.", Value::String("off".to_string()), None, None);
    pub static AUTOSAVE: Config = Config::new("autosave", "Whether the periodically save the map when playing.", Value::Bool(false), None, None);
    pub static AUTOSAVE_AMOUNT: Config = Config::new("autosaveAmount", "The maximum amount of autosaves. Older ones get replaced.", Value::Int(10), None, None);
    pub static AUTOSAVE_SPACING: Config = Config::new("autosaveSpacing", "Spacing between autosaves in seconds.", Value::Int(60 * 5), None, None);
    pub static DEBUG: Config = Config::new("debug", "Enable debug logging.", Value::Bool(false), Some("debug"), Some(Box::new(|| set_debug(DEBUG.get().get_bool()))));
    pub static SNAPSHOT_INTERVAL: Config = Config::new("snapshotInterval", "Client entity snapshot interval in ms.", Value::Int(200), None, None);
    pub static AUTO_PAUSE: Config = Config::new("autoPause", "Whether the game should pause when nobody is online.", Value::Bool(false), None, None);

    impl Config {
        pub fn new(name: &'static str, description: &'static str, default_value: Value, key: Option<&'static str>, changed: Option<Box<dyn Fn()>>) -> Self {
            let s = Self {
                name,
                description,
                key,
                default_value,
                changed,
            };
            unsafe {
                ALL_CONFIGS.push(&s);
            }
            s
        }

        pub fn set(&mut self, value: Value) {
            unsafe {
                SETTINGS.unwrap().set(self.key.parse().unwrap(), value);
            }
            if let Some(f) = &self.changed {
                f();
            }
        }

        /// Doesn't give a save the default value as a pointer, its a clone of the default value in the settings struct. But returns a reference to the value in the settings struct. i.e. dont edit the default value (why should you even do that?)
        pub fn get(&self) -> &Value {
            unsafe {
                &SETTINGS.unwrap().get_or_default(self.key.parse().unwrap(), self.default_value.clone())
            }
        }

        pub fn bool(&self) -> bool {
            self.get().get_bool()
        }

        pub fn int(&self) -> i32 {
            self.get().get_int()
        }

        pub fn float(&self) -> f32 {
            self.get().get_float()
        }

        /// doesnt return a reference but a clone of the string
        pub fn string(&self) -> String {
            self.get().get_string()
        }

        pub fn long(&self) -> i64 {
            self.get().get_long()
        }

        fn debug() -> bool {
            DEBUG.get().get_bool()
        }
    }
}

pub struct TraceInfo {
    pub ip: String,
    pub uuid: String,
    pub modded: bool,
    pub mobile: bool,
    pub times_joined: i32,
    pub times_kicked: i32,
}

impl Default for TraceInfo {
    fn default() -> Self {
        Self {
            ip: "<unknown>".to_string(),
            uuid: "<unknown>".to_string(),
            modded: false,
            mobile: false,
            times_joined: 0,
            times_kicked: 0,
        }
    }
}

impl TraceInfo {
    pub fn new(ip: String, uuid: String, modded: bool, mobile: bool, times_joined: i32, times_kicked: i32) -> Self {
        Self {
            ip,
            uuid,
            modded,
            mobile,
            times_joined,
            times_kicked,
        }
    }
}
