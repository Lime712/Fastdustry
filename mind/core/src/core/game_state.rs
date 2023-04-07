use crate::game::game_stats::GameStats;
use crate::game::rules::Rules;
use crate::game::teams::Teams;
use crate::maps::map::Map;
use crate::world::blocks::attributes::Attributes;

pub struct GameState<'a> {
    /// Current wave number, can be anything in non-wave modes.
    pub wave: i32,
    // = 1;
    /// Wave countdown in ticks.
    pub wavetime: f32,
    // = 0.0;
    /// Logic tick.
    pub tick: f64,
    // = 0.0;
    /// Continuously ticks up every non-paused update.
    pub update_id: u64,
    // = 0;
    /// Whether the game is in game over state.
    pub game_over: bool,
    // = false;
    /// Whether the player's team won the match.
    pub won: bool,
    // = false;
    /// Server ticks/second. Only valid in multiplayer.
    pub server_tps: i32,
    // = - 1;
    /// Map that is currently being played on.
    pub map: Option<Map<'a>>,
    // = None;
    /// The current game rules.
    pub rules: Rules,
    // = Rules::default();
    /// Statistics for this save/game. Displayed after game over.
    pub stats: GameStats,
    // = GameStats::default();
    /// Global attributes of the environment, calculated by weather.
    pub env_attrs: Attributes,
    // = Attributes::default();
    /// Team data. Gets reset every new game.
    pub teams: Teams,
    // = Teams::default();
    /// Number of enemies in the game; only used clientside in servers.
    pub enemies: i32,
    // = 0;
    /// Map being playtested (not edited!)
    pub playtesting_map: Option<Map<'a>>,
    // = None;
    /// Current game state.
    state: State, // = State::Menu;
}

pub enum State {
    Paused,
    Playing,
    Menu,
}

impl Default for GameState<'_> {
    fn default() -> Self {
        GameState {
            wave: 1,
            wavetime: 0.0,
            tick: 0.0,
            update_id: 0,
            game_over: false,
            won: false,
            server_tps: -1,
            map: None,
            rules: Rules::default(),
            stats: GameStats::default(),
            env_attrs: Attributes::default(),
            teams: Teams::default(),
            enemies: 0,
            playtesting_map: None,
            state: State::Menu,
        }
    }
}

impl GameState<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_campaign(&self) -> bool {
        false
    }
}