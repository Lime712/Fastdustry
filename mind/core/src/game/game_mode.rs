// in java this is a enum, but i think its better to just have a constant for each gamemode, cause in rust its not possible to create a enum with a value

use crate::game::rules::Rules;
use crate::maps::map::Map;

pub const SURVIVAL: GameMode =
    GameMode::new({
                      let mut rules = Rules::default();
                      rules.wave_timer = true;
                      rules.waves = true;
                      rules
                  }, |map| map.spawns > 0, false);
pub const SANDBOX: GameMode =
    GameMode::new({
                      let mut rules = Rules::default();
                      rules.infinite_resources = true;
                      rules.waves = true;
                      rules.wave_timer = false;
                      rules
                  }, |_| true, false);
pub const ATTACK: GameMode =
    GameMode::new({
                      let mut rules = Rules::default();
                      rules.attack_mode = true;
                      //TODO waves is now a bad idea
                      //rules.waves = true;
                      rules.wave_timer = true;

                      rules.wave_spacing = 2f32 * 60f32;
                      rules.wave_team.rules.infinite_resources = true;
                      rules
                  }, |map| map.teams.len() > 1, false);
pub const PVP: GameMode =
    GameMode::new({
                      let mut rules = Rules::default();
                      rules.pvp = true;
                      rules.enemy_core_build_radius = 600f32;
                      rules.build_cost_multiplier = 1f32;
                      rules.build_speed_multiplier = 1f32;
                      rules.unit_build_speed_multiplier = 2f32;
                      rules.attack_mode = true;
                      rules
                  }, |map| map.teams.len() > 1, false);
pub const EDITOR: GameMode =
    GameMode::new({
                      let mut rules = Rules::default();
                      rules.infinite_resources = true;
                      rules.editor = true;
                      rules.waves = false;
                      rules.wave_timer = false;
                      rules
                  }, |_| true, true);

// in java its called gamemode, but i thought this is a better name
pub struct GameMode {
    rules: Rules,
    validator: fn(&Map) -> bool,
    pub hidden: bool,
}

impl GameMode {
    pub const fn new(rules: Rules, validator: fn(&Map) -> bool, hidden: bool) -> GameMode {
        GameMode {
            rules,
            validator,
            hidden,
        }
    }

    pub fn validate(&self, map: &Map) -> bool {
        (self.validator)(map)
    }
}