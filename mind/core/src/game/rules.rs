// original java code:
// /** Sandbox mode: Enables infinite resources, build range and build speed. */
//     public boolean infiniteResources;
//     /** Team-specific rules. */
//     public TeamRules teams = new TeamRules();
//     /** Whether the waves come automatically on a timer. If not, waves come when the play button is pressed. */
//     public boolean waveTimer = true;
//     /** Whether the waves can be manually summoned with the play button. */
//     public boolean waveSending = true;
//     /** Whether waves are spawnable at ALL. */
//     public boolean waves;
//     /** Whether the game objective is PvP. Note that this enables automatic hosting. */
//     public boolean pvp;
//     /** Whether is waiting for players enabled in PvP. */
//     public boolean pvpAutoPause = true;
//     /** Whether to pause the wave timer until ALL enemies are destroyed. */
//     public boolean waitEnemies = false;
//     /** Determines if gamemode is attack mode. */
//     public boolean attackMode = false;
//     /** Whether this is the editor gamemode. */
//     public boolean editor = false;
//     /** Whether a gameover can happen at ALL. Set this to false to implement custom gameover conditions. */
//     public boolean canGameOver = true;
//     /** Whether cores change teams when they are destroyed. */
//     public boolean coreCapture = false;
//     /** Whether reactors can explode and damage other blocks. */
//     public boolean reactorExplosions = true;
//     /** Whether to allow manual unit control. */
//     public boolean possessionAllowed = true;
//     /** Whether schematics are allowed. */
//     public boolean schematicsAllowed = true;
//     /** Whether friendly explosions can occur and set fire/damage other blocks. */
//     public boolean damageExplosions = true;
//     /** Whether fire (and neoplasm spread) is enabled. */
//     public boolean fire = true;
//     /** Whether units use and require ammo. */
//     public boolean unitAmmo = false;
//     /** EXPERIMENTAL! If true, blocks will update in units and share power. */
//     public boolean unit_payload_update = false;
//     /** Whether cores add to unit limit */
//     public boolean unitCapVariable = true;
//     /** If true, unit spawn points are shown. */
//     public boolean showSpawns = false;
//     /** Multiplies power output of solar panels. */
//     public float solarMultiplier = 1f;
//     /** How fast unit factories build units. */
//     public float unitBuildSpeedMultiplier = 1f;
//     /** Multiplier of resources that units take to build. */
//     public float unitCostMultiplier = 1f;
//     /** How much damage units deal. */
//     public float unitDamageMultiplier = 1f;
//     /** How much damage unit crash damage deals. (Compounds with unitDamageMultiplier) */
//     public float unitCrashDamageMultiplier = 1f;
//     /** If true, ghost blocks will appear upon destruction, letting builder blocks/units rebuild them. */
//     public boolean ghostBlocks = true;
//     /** Whether to allow units to build with logic. */
//     public boolean logicUnitBuild = true;
//     /** If true, world processors no longer update. Used for testing. */
//     public boolean disableWorldProcessors = false;
//     /** How much HEALTH blocks start with. */
//     public float blockHealthMultiplier = 1f;
//     /** How much damage blocks (turrets) deal. */
//     public float blockDamageMultiplier = 1f;
//     /** Multiplier for buildings resource cost. */
//     public float buildCostMultiplier = 1f;
//     /** Multiplier for building speed. */
//     public float buildSpeedMultiplier = 1f;
//     /** Multiplier for percentage of materials refunded when deconstructing. */
//     public float deconstructRefundMultiplier = 0.5f;
//     /** No-build zone around enemy core radius. */
//     public float enemyCoreBuildRadius = 400f;
//     /** If true, no-build zones are calculated based on the closest core. */
//     public boolean polygonCoreProtection = false;
//     /** If true, blocks cannot be placed near blocks that are near the enemy team.*/
//     public boolean placeRangeCheck = false;
//     /** If true, dead teams in PvP automatically have their blocks & units converted to derelict upon death. */
//     public boolean cleanupDeadTeams = true;
//     /** If true, items can only be deposited in the core. */
//     public boolean onlyDepositCore = false;
//     /** If true, every enemy block in the radius of the (enemy) core is destroyed upon death. Used for campaign maps. */
//     public boolean coreDestroyClear = false;
//     /** If true, banned blocks are hidden from the build menu. */
//     public boolean hideBannedBlocks = false;
//     /** If true, bannedBlocks becomes a whitelist. */
//     public boolean blockWhitelist = false;
//     /** If true, bannedUnits becomes a whitelist. */
//     public boolean unitWhitelist = false;
//     /** Radius around enemy wave drop zones.*/
//     public float dropZoneRadius = 300f;
//     /** Time between waves in ticks. */
//     public float waveSpacing = 2 * Time.toMinutes;
//     /** Starting wave spacing; if <=0, uses waveSpacing * 2. */
//     public float initialWaveSpacing = 0f;
//     /** Wave after which the player 'wins'. Used in sectors. Use a value <= 0 to disable. */
//     public int winWave = 0;
//     /** Base unit cap. Can still be increased by blocks. */
//     public int unitCap = 0;
//     /** Environment drag multiplier. */
//     public float dragMultiplier = 1f;
//     /** Environmental flags that dictate visuals & how blocks function. */
//     public int env = Vars.defaultEnv;
//     /** Attributes of the environment. */
//     public Attributes attributes = new Attributes();
//     /** Sector for saves that have them. */
//     public @Nullable Sector sector;
//     /** Spawn layout. */
//     public Seq<SpawnGroup> spawns = new Seq<>();
//     /** Starting items put in cores. */
//     public Seq<ItemStack> loadout = ItemStack.list(Items.copper, 100);
//     /** Weather events that occur here. */
//     public Seq<WeatherEntry> weather = new Seq<>(1);
//     /** Blocks that cannot be placed. */
//     public ObjectSet<Block> bannedBlocks = new ObjectSet<>();
//     /** Units that cannot be built. */
//     public ObjectSet<UnitType> bannedUnits = new ObjectSet<>();
//     /** Reveals blocks normally hidden by build visibility. */
//     public ObjectSet<Block> revealedBlocks = new ObjectSet<>();
//     /** Unlocked content names. Only used in multiplayer when the campaign is enabled. */
//     public ObjectSet<String> researched = new ObjectSet<>();
//     /** Block containing these items as requirements are hidden. */
//     public ObjectSet<Item> hiddenBuildItems = Items.erekirOnlyItems.asSet();
//     /** In-map objective executor. */
//     public MapObjectives objectives = new MapObjectives();
//     /** Flags set by objectives. Used in world processors. */
//     public ObjectSet<String> objectiveFlags = new ObjectSet<>();
//     /** If true, fog of war is enabled. Enemy units and buildings are hidden unless in radar view. */
//     public boolean fog = false;
//     /** If fog = true, this is whether static (black) fog is enabled. */
//     public boolean staticFog = true;
//     /** Color for static, undiscovered fog of war areas. */
//     public Color staticColor = new Color(0f, 0f, 0f, 1f);
//     /** Color for discovered but un-monitored fog of war areas. */
//     public Color dynamicColor = new Color(0f, 0f, 0f, 0.5f);
//     /** Whether ambient lighting is enabled. */
//     public boolean lighting = false;
//     /** Ambient light color, used when lighting is enabled. */
//     public Color ambientLight = new Color(0.01f, 0.01f, 0.04f, 0.99f);
//     /** team of the player by default. */
//     public Team defaultTeam = Team.sharded;
//     /** team of the enemy in waves/sectors. */
//     public Team waveTeam = Team.crux;
//     /** color of clouds that is displayed when the player is landing */
//     public Color cloudColor = new Color(0f, 0f, 0f, 0f);
//     /** name of the custom mode that this ruleset describes, or null. */
//     public @Nullable String modeName;
//     /** Mission string displayed instead of wave/core counter. Null to disable. */
//     public @Nullable String mission;
//     /** Whether cores incinerate items when full, just like in the campaign. */
//     public boolean coreIncinerates = false;
//     /** If false, borders fade out into darkness. Only use with custom backgrounds!*/
//     public boolean borderDarkness = true;
//     /** If true, the map play area is cropped based on the rectangle below. */
//     public boolean limitMapArea = false;
//     /** Map area limit rectangle. */
//     public int limitX, limitY, limitWidth = 1, limitHeight = 1;
//     /** If true, blocks outside the map area are disabled. */
//     public boolean disableOutsideArea = true;
//     /** special tags for additional info. */
//     public StringMap tags = new StringMap();
//     /** Name of callback to call for background rendering in mods; see Renderer#addCustomBackground. Runs last. */
//     public @Nullable String customBackgroundCallback;
//     /** path to background texture with extension (e.g. "sprites/space.png")*/
//     public @Nullable String backgroundTexture;
//     /** background texture move speed scaling - bigger numbers mean slower movement. 0 to disable. */
//     public float backgroundSpeed = 27000f;
//     /** background texture scaling factor */
//     public float backgroundScl = 1f;
//     /** background UV offsets */
//     public float backgroundOffsetX = 0.1f, backgroundOffsetY = 0.1f;
//     /** Parameters for planet rendered in the background. Cannot be changed once a map is loaded. */
//     public @Nullable PlanetParams planetBackground;

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
    /// How much damage blocks (turrets) deal.
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
    pub environment: i32,
    // Attributes of the environment.
    // pub attributes: Attributes,
    // /// Spawn layout.
    // pub spawns: Vec<SpawnGroup>,
    // /// Starting items put in cores.
    // pub loadout: Vec<ItemStack>,
}
