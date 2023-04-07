use std::collections::{HashMap, HashSet};

use arc::arc_core::graphics::color::Color;

use crate::content::items::COPPER;
use crate::game::game_mode::{ATTACK, EDITOR, GameMode, PVP, SANDBOX, SURVIVAL};
use crate::game::team::{ALL, CRUX, SHARDED, Team};
use crate::r#type::item::Item;
use crate::r#type::item_stack::{create_item_stacks, ItemStack};
use crate::vars;
use crate::world::block::Block;
use crate::world::blocks::attributes::Attributes;
use crate::world::meta::block_enums::Env;

pub struct Rules {
    /// Sandbox mode: Enables infinite resources, build range and build speed.
    pub infinite_resources: bool,
    /// Team-specific rules.
    pub teams: TeamRules,
    /// Whether the waves come automatically on a timer. If not, waves come when the play button is pressed.
    pub wave_timer: bool,
    /// Whether the waves can be manually summoned with the play button.
    pub wave_sending: bool,
    /// Whether waves are spawnable at ALL.
    pub waves: bool,
    /// Whether the game objective is PvP. Note that this enables automatic hosting.
    pub pvp: bool,
    /// Whether is waiting for players enabled in PvP.
    pub pvp_auto_pause: bool,
    /// Whether to pause the wave timer until ALL enemies are destroyed.
    pub wait_enemies: bool,
    /// Determines if game mode is Attack mode.
    pub attack_mode: bool,
    /// Whether this is the Editor gamemode.
    pub editor: bool,
    /// Whether a gameover can happen at ALL. Set this to false to implement custom gameover conditions.
    pub can_game_over: bool,
    /// Whether cores change teams when they are destroyed.
    pub core_capture: bool,
    /// Whether reactors can explode and damage other blocks.
    pub reactor_explosions: bool,
    /// Whether to allow manual unit control.
    pub possession_allowed: bool,
    /// Whether schematics are allowed.
    pub schematics_allowed: bool,
    /// Whether friendly explosions can occur and set fire/damage other blocks.
    pub damage_explosions: bool,
    /// Whether fire (and neoplasm spread) is enabled.
    pub fire: bool,
    /// Whether units use and require ammo.
    pub unit_ammo: bool,
    /// EXPERIMENTAL! If true, blocks will update in units and share power.
    pub unit_payload_update: bool,
    /// Whether cores add to unit limit
    pub unit_cap_variable: bool,
    /// If true, unit spawn points are shown.
    pub show_spawns: bool,
    /// Multiplies power output of solar panels.
    pub solar_multiplier: f32,
    /// How fast unit factories build units.
    pub unit_build_speed_multiplier: f32,
    /// Multiplier of resources that units take to build.
    pub unit_cost_multiplier: f32,
    /// How much damage units deal.
    pub unit_damage_multiplier: f32,
    /// How much damage unit crash damage deals. (Compounds with unitDamageMultiplier)
    pub unit_crash_damage_multiplier: f32,
    /// If true, ghost blocks will appear upon destruction, letting builder blocks/units rebuild them.
    pub ghost_blocks: bool,
    /// Whether to allow units to build with logic.
    pub logic_unit_build: bool,
    /// If true, world processors no longer update. Used for testing.
    pub disable_world_processors: bool,
    /// How much HEALTH blocks start with.
    pub block_health_multiplier: f32,
    /// How much damage blocks (Turrets) deal.
    pub block_damage_multiplier: f32,
    /// Multiplier for buildings resource cost.
    pub build_cost_multiplier: f32,
    /// Multiplier for building speed.
    pub build_speed_multiplier: f32,
    /// Multiplier for percentage of materials refunded when deconstructing.
    pub deconstruct_refund_multiplier: f32,
    /// No-build zone around enemy core radius.
    pub enemy_core_build_radius: f32,
    /// If true, no-build zones are calculated based on the closest core.
    pub polygon_core_protection: bool,
    /// If true, blocks cannot be placed near blocks that are near the enemy team.
    pub place_range_check: bool,
    /// If true, dead teams in PvP automatically have their blocks & units converted to derelict upon death.
    pub cleanup_dead_teams: bool,
    /// If true, items can only be deposited in the core.
    pub only_deposit_core: bool,
    /// If true, every enemy block in the radius of the (enemy) core is destroyed upon death. Used for campaign maps.
    pub core_destroy_clear: bool,
    /// If true, banned blocks are hidden from the build menu.
    pub hide_banned_blocks: bool,
    /// If true, bannedBlocks becomes a whitelist.
    pub block_whitelist: bool,
    /// If true, bannedUnits becomes a whitelist.
    pub unit_whitelist: bool,
    /// Radius around enemy wave drop zones.
    pub drop_zone_radius: f32,
    /// Time between waves in ticks.
    pub wave_spacing: f32,
    /// Starting wave spacing; if <=0, uses waveSpacing * 2.
    pub initial_wave_spacing: f32,
    /// Wave after which the player 'wins'. Used in sectors. Use a value <= 0 to disable.
    pub win_wave: f32,
    /// Base unit cap. Can still be increased by blocks.
    pub unit_cap: f32,
    /// Environment drag multiplier.
    pub drag_multiplier: f32,
    /// Environmental flags that dictate visuals & how blocks function.
    pub env: Env,
    /// Attributes of the environment.
    pub attributes: Attributes,
    /// Spawn layout.
    pub spawns: Vec<SpawnGroup>,
    /// Starting items put in cores.
    pub loadout: Vec<ItemStack>,
    /// Weather events that occur here.
    pub weather: Vec<WeatherEntry>,
    /// Blocks that cannot be placed.
    pub banned_blocks: HashSet<&'static Block>,
    /// Units that cannot be built.
    pub banned_units: HashSet<UnitType>,
    /// Reveals blocks normally hidden by build visibility.
    pub revealed_blocks: HashSet<&'static Block>,
    /// Unlocked content names. Only used in multiplayer when the campaign is enabled.
    pub researched: HashSet<String>,
    /// Block containing these items as requirements are hidden.
    pub hidden_build_items: HashSet<Item>,
    /// In-map objective executor.
    pub objectives: MapObjectives,
    /// Flags set by objectives. Used in world processors.
    pub objective_flags: HashSet<String>,
    /// If true, fog of war is enabled. Enemy units and buildings are hidden unless in radar view.
    pub fog: bool,
    /// If fog = true, this is whether static (black) fog is enabled.
    pub static_fog: bool,
    /// Color for static, undiscovered fog of war areas.
    pub static_color: Color,
    /// Color for discovered but un-monitored fog of war areas.
    pub dynamic_color: Color,
    /// Whether ambient lighting is enabled.
    pub lighting: bool,
    /// Ambient light color, used when lighting is enabled.
    pub ambient_light: Color,
    /// team of the player by default.
    pub default_team: &'static Team,
    /// team of the enemy in waves/sectors.
    pub wave_team: &'static Team,
    /// color of clouds that is displayed when the player is landing
    pub cloud_color: Color,
    /// name of the custom mode that this ruleset describes, or null.
    pub mode_name: Option<String>,
    /// Mission string displayed instead of wave/core counter. Null to disable.
    pub mission: Option<String>,
    /// Whether cores incinerate items when full, just like in the campaign.
    pub core_incinerates: bool,
    /// If false, borders fade out into darkness. Only use with custom backgrounds!
    pub border_darkness: bool,
    /// If true, the map play area is cropped based on the rectangle below.
    pub limit_map_area: bool,
    /// Map area limit rectangle.
    pub limit_x: i32,
    pub limit_y: i32,
    pub limit_width: i32,
    pub limit_height: i32,
    /// If true, blocks outside the map area are disabled.
    pub disable_outside_area: bool,
    /// special tags for additional info.
    pub tags: HashMap<String, String>,
    /// Name of callback to call for background rendering in mods; see Renderer#addCustomBackground. Runs last.
    pub custom_background_callback: Option<String>,
    /// path to background texture with extension (e.g. "sprites/space.png")
    pub background_texture: Option<String>,
    /// background texture move speed scaling - bigger numbers mean slower movement. 0 to disable.
    pub background_speed: f32,
    /// background texture scaling factor
    pub background_scl: f32,
    /// background UV offsets
    pub background_offset_x: f32,
    pub background_offset_y: f32,
    /// Parameters for planet rendered in the background. Cannot be changed once a map is loaded.
    pub planet_background: Option<PlanetParams>,
}

impl Default for Rules {
    const fn default() -> Self {
        Self {
            // maybe not everything is correct here
            infinite_resources: false,
            teams: TeamRules::new(),
            wave_timer: true,
            wave_sending: true,
            waves: true,
            pvp: false,
            pvp_auto_pause: true,
            wait_enemies: true,
            attack_mode: false,
            editor: false,
            can_game_over: true,
            core_capture: true,
            reactor_explosions: true,
            possession_allowed: true,
            schematics_allowed: true,
            damage_explosions: true,
            fire: true,
            unit_ammo: true,
            unit_payload_update: false,
            unit_cap_variable: false,
            show_spawns: false,
            solar_multiplier: 1.0,
            unit_build_speed_multiplier: 1.0,
            unit_cost_multiplier: 1.0,
            unit_damage_multiplier: 1.0,
            unit_crash_damage_multiplier: 1.0,
            ghost_blocks: true,
            logic_unit_build: false,
            disable_world_processors: false,
            block_health_multiplier: 1.0,
            block_damage_multiplier: 1.0,
            build_cost_multiplier: 1.0,
            build_speed_multiplier: 1.0,
            deconstruct_refund_multiplier: 1.0,
            enemy_core_build_radius: 0.0,
            polygon_core_protection: false,
            place_range_check: false,
            cleanup_dead_teams: false,
            only_deposit_core: false,
            core_destroy_clear: false,
            hide_banned_blocks: false,
            block_whitelist: false,
            unit_whitelist: false,
            drop_zone_radius: 0.0,
            wave_spacing: 0.0,
            initial_wave_spacing: 0.0,
            win_wave: 0.0,
            unit_cap: 0.0,
            drag_multiplier: 1.0,
            env: vars::DEFAULT_ENV.clone(),
            attributes: Default::default(),
            spawns: vec![],
            // was: loadout: create_item_stacks(vec![&COPPER.lock().unwrap()], vec![100]),
            loadout: vec![],
            weather: vec![],
            banned_blocks: Default::default(),
            banned_units: Default::default(),
            revealed_blocks: Default::default(),
            researched: Default::default(),
            hidden_build_items: Default::default(),
            objectives: (),
            objective_flags: Default::default(),
            fog: false,
            static_fog: true,
            static_color: Color::new(0.0, 0.0, 0.0, 1.0),
            dynamic_color: Color::new(0.0, 0.0, 0.0, 0.5),
            lighting: false,
            ambient_light: Color::new(0.01, 0.01, 0.04, 0.99),
            default_team: SHARDED,
            wave_team: CRUX,
            cloud_color: Color::new(0.0, 0.0, 0.0, 0.0),
            mode_name: None,
            mission: None,
            core_incinerates: false,
            border_darkness: true,
            limit_map_area: false,
            limit_x: 0,
            limit_y: 0,
            limit_width: 1,
            limit_height: 1,
            disable_outside_area: true,
            tags: Default::default(),
            custom_background_callback: None,
            background_texture: None,
            background_speed: 27000.0,
            background_scl: 1.0,
            background_offset_x: 0.1,
            background_offset_y: 0.1,
            planet_background: None,
        }
    }
}

impl Rules {
    pub fn mode(&self) -> &GameMode {
        if self.pvp {
            &PVP
        } else if self.editor {
            &EDITOR
        } else if self.attack_mode {
            &ATTACK
        } else if self.infinite_resources {
            &SANDBOX
        } else {
            &SURVIVAL
        }
    }

    pub fn has_env(&self, env: i32) -> bool {
        (self.env & env) != 0
    }

    pub fn unit_build_speed(&self, team: &Team) -> f32 {
        self.unit_build_speed_multiplier * self.teams.get(team).unit_build_speed_multiplier
    }

    pub fn unit_cost(&self, team: &Team) -> f32 {
        self.unit_cost_multiplier * self.teams.get(team).unit_cost_multiplier
    }

    pub fn unit_damage(&self, team: &Team) -> f32 {
        self.unit_damage_multiplier * self.teams.get(team).unit_damage_multiplier
    }

    pub fn unit_crash_damage(&self, team: &Team) -> f32 {
        self.unit_crash_damage_multiplier * self.teams.get(team).unit_crash_damage_multiplier
    }

    pub fn block_health(&self, team: &Team) -> f32 {
        self.block_health_multiplier * self.teams.get(team).block_health_multiplier
    }

    pub fn block_damage(&self, team: &Team) -> f32 {
        self.block_damage_multiplier * self.teams.get(team).block_damage_multiplier
    }

    pub fn build_cost(&self, team: &Team) -> f32 {
        self.build_cost_multiplier * self.teams.get(team).build_cost_multiplier
    }

    pub fn build_speed(&self, team: &Team) -> f32 {
        self.build_speed_multiplier * self.teams.get(team).build_speed_multiplier
    }

    pub fn is_banned(&self, block: &Block) -> bool {
        self.block_whitelist != self.banned_blocks.contains(block)
    }

    pub fn is_banned_unit(&self, unit: &UnitType) -> bool {
        self.unit_whitelist != self.banned_units.contains(unit)
    }
}

pub struct TeamRules {
    values: [TeamRule; ALL.len()],
}

/// A simple map for storing TeamRules in an efficient way without hashing.
impl TeamRules {
    pub fn new() -> Self {
        Self {
            values: [TeamRule::default(); ALL.len()],
        }
    }

    pub fn get(&self, team: &Team) -> &TeamRule {
        // should work because new() initializes all values
        &self.values[team.id]
    }

    pub fn get_mut(&mut self, team: Team) -> &mut TeamRule {
        &mut self.values[team as usize]
    }
}

pub struct TeamRule {
    /// Whether, when AI is enabled, ships should be spawned from the core. TODO remove / unnecessary?
    pub ai_core_spawn: bool,
    /// If true, blocks don't require power or resources.
    pub cheat: bool,
    /// If true, resources are not consumed when building.
    pub infinite_resources: bool,
    /// If true, this team has infinite unit ammo.
    pub infinite_ammo: bool,
    /// Enables "RTS" unit AI.
    pub rts_ai: bool,
    /// Minimum size of attack squads.
    pub rts_min_squad: i32,
    /// Maximum size of attack squads.
    pub rts_max_squad: i32,
    /// Minimum "advantage" needed for a squad to attack. Higher -> more cautious.
    pub rts_min_weight: f32,
    /// How fast unit factories build units.
    pub unit_build_speed_multiplier: f32,
    /// How much damage units deal.
    pub unit_damage_multiplier: f32,
    /// How much damage unit crash damage deals. (Compounds with unitDamageMultiplier)
    pub unit_crash_damage_multiplier: f32,
    /// Multiplier of resources that units take to build.
    pub unit_cost_multiplier: f32,
    /// How much health blocks start with.
    pub block_health_multiplier: f32,
    /// How much damage blocks (turrets) deal.
    pub block_damage_multiplier: f32,
    /// Multiplier for building speed.
    pub build_speed_multiplier: f32,
}

impl Default for TeamRule {
    fn default() -> Self {
        Self {
            ai_core_spawn: true,
            cheat: false,
            infinite_resources: false,
            infinite_ammo: false,
            rts_ai: false,
            rts_min_squad: 4,
            rts_max_squad: 1000,
            rts_min_weight: 1.2,
            unit_build_speed_multiplier: 1.0,
            unit_damage_multiplier: 1.0,
            unit_crash_damage_multiplier: 1.0,
            unit_cost_multiplier: 1.0,
            block_health_multiplier: 1.0,
            block_damage_multiplier: 1.0,
            build_speed_multiplier: 1.0,
        }
    }
}

pub struct Player {
    pub team: &'static Team,
}

impl Player {
    pub fn new(team: &'static Team) -> Self {
        Self { team }
    }
}