use crate::r#type::item_stack::ItemStack;
use crate::vars;
use crate::world::blocks::attributes::Attributes;
use crate::world::meta::block_enums::Env;

pub struct Rules {
    /// Sandbox mode: Enables infinite resources, build range and build speed.
    pub infinite_resources: bool,
    /// Team-specific rules.
    // pub teams: Vec<TeamRules>,
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
    /// Determines if game mode is attack mode.
    pub attack_mode: bool,
    /// Whether this is the editor gamemode.
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
    // pub spawns: Vec<SpawnGroup>,
    /// Starting items put in cores.
    pub loadout: Vec<ItemStack>,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            infinite_resources: false,
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
            loadout: Vec::new(),
        }
    }
}