use std::any::Any;
use std::collections::HashMap;
use crate::entities::bullet::bullet_type::BulletType;
use crate::r#type::category::Category;
use crate::r#type::item::Item;
use crate::r#type::item_stack::ItemStack;
use crate::r#type::liquid::Liquid;
use crate::world::blocks::attributes::Attributes;
use crate::world::meta::block_enums::{BlockFlag, BlockGroup, Env};

pub struct Block {
    /// If true, buildings have an ItemModule.
    pub has_items: bool,
    /// If true, buildings have a LiquidModule.
    pub has_liquids: bool,
    /// If true, buildings have a PowerModule.
    pub has_power: bool,
    /// Flag for determining whether this block outputs liquid somewhere; used for connections.
    pub outputs_liquid: bool,
    /// Used by certain power blocks (nodes) to flag as non-consuming of power. True by default, even if this block has no power.
    pub consumes_power: bool,
    /// If true, this block is a generator that can produce power.
    pub outputs_power: bool,
    /// If false, power nodes cannot connect to this block.
    pub connected_power: bool,
    /// If true, this block can conduct power like a cable.
    pub conductive_power: bool,
    /// If true, this block can output payloads; affects blending.
    pub outputs_payload: bool,
    /// If true, payloads will attempt to move into this block.
    pub accepts_payload: bool,
    /// Visual flag use for blending of certain transportation blocks.
    pub accepts_items: bool,
    /// If true, all item capacities of this block are separate instead of pooled as one number.
    pub separate_item_capacity: bool,
    /// Maximum items this block can carry (usually, this is per-type of item)
    pub item_capacity: i32,
    /// Maximum total liquids this block can carry if hasLiquids = true
    pub liquid_capacity: f32,
    /// Higher numbers increase liquid output speed; TODO remove and replace with better liquids system
    pub liquid_pressure: f32,
    /// If true, this block outputs to its facing direction, when applicable.
    /// Used for blending calculations.
    pub output_facing: bool,
    /// If true, this block does not accept input from the sides (used for armored conveyors)
    pub no_side_blend: bool,
    /// Whether to display flow rate
    pub display_flow: bool,
    /// Whether this block is visible in the editor
    pub in_editor: bool,
    /// The last configuration value applied to this block.
    pub last_config: Option<Box<dyn Any>>,
    /// Whether to save the last config and apply it to newly placed blocks
    pub save_config: bool,
    /// Whether to allow copying the config through middle click
    pub copy_config: bool,
    /// If true, double-tapping this configurable block clears configuration.
    pub clear_on_double_tap: bool,
    /// Whether this block has a tile entity that updates
    pub update: bool,
    /// Whether this block has health and can be destroyed
    pub destructible: bool,
    /// Whether unloaders work on this block
    pub unloadable: bool,
    /// If true, this block acts a duct and will connect to armored ducts from the side.
    pub is_duct: bool,
    /// Whether units can resupply by taking items from this block
    pub allow_resupply: bool,
    /// Whether this is solid
    pub solid: bool,
    /// Whether this block CAN be solid.
    pub solidifies: bool,
    /// if true, this counts as a non-solid block to this team.
    pub team_passable: bool,
    /// if true, this block cannot be hit by bullets unless explicitly targeted.
    pub under_bullets: bool,
    /// whether this is rotatable
    pub rotate: bool,
    /// if rotate is true and this is false, the region won't rotate when drawing
    pub rotate_draw: bool,
    /// if true, schematic flips with this block are inverted.
    pub invert_flip: bool,
    /// number of different variant regions to use
    pub variants: u8,
    /// whether to draw a rotation arrow - this does not apply to lines of blocks
    pub draw_arrow: bool,
    /// whether to draw the team corner by default
    pub draw_team_overlay: bool,
    /// for static blocks only: if true, tile data() is saved in world data.
    pub save_data: bool,
    /// whether you can break this with rightclick
    pub breakable: bool,
    /// whether to add this block to brokenblocks
    pub rebuildable: bool,
    /// if true, this logic-related block can only be used with privileged processors (or is one itself)
    pub privileged: bool,
    /// whether this block can only be placed on water
    pub requires_water: bool,
    /// whether this block can be placed on any liquids, anywhere
    pub placeable_liquid: bool,
    /// whether this block can be placed directly by the player via PlacementFragment
    pub placeable_player: bool,
    /// whether this floor can be placed on.
    pub placeable_on: bool,
    /// whether this block has insulating properties.
    pub insulated: bool,
    /// whether the sprite is a full square.
    pub square_sprite: bool,
    /// whether this block absorbs laser attacks.
    pub absorb_lasers: bool,
    /// if false, the status is never drawn
    pub enable_draw_status: bool,
    /// whether to draw disabled status
    pub draw_disabled: bool,
    /// whether to automatically reset enabled status after a logic block has not interacted for a while.
    pub auto_reset_enabled: bool,
    /// if true, the block stops updating when disabled
    pub no_update_disabled: bool,
    /// if true, this block updates when it's a payload in a unit.
    pub update_in_units: bool,
    /// if true, this block updates in payloads in units regardless of the experimental game rule
    pub always_update_in_units: bool,
    /// Whether to use this block's color in the minimap. Only used for overlays.
    pub use_color: bool,
    /// item that drops from this block, used for drills
    pub item_drop: Option<Item>,
    /// if true, this block cannot be mined by players. useful for annoying things like sand.
    pub player_unmineable: bool,
    /// Array of affinities to certain things.
    pub attributes: Attributes,
    /// Health per square tile that this block occupies; essentially, this is multiplied by size * size. Overridden if health is > 0. If <0, the default is 40.
    pub scaled_health: f32,
    /// building health; -1 to use scaledHealth
    pub health: i32,
    /// damage absorption, similar to unit armor
    pub armor: f32,
    /// base block explosiveness
    pub base_explosiveness: f32,
    /// bullet that this block spawns when destroyed
    pub destroy_bullet: Option<BulletType>,
    /// liquid used for lighting
    pub light_liquid: Option<Liquid>,
    /// whether cracks are drawn when this block is damaged
    pub draw_cracks: bool,
    /// whether rubble is created when this block is destroyed
    pub create_rubble: bool,
    /// whether this block can be placed on edges of liquids.
    pub floating: bool,
    /// multiblock size
    pub size: u8,
    /// multiblock offset
    pub offset: f32,
    /// offset for iteration (internal use only)
    pub size_offset: u8,
    /// Clipping size of this block. Should be as large as the block will draw.
    pub clip_size: f32,
    /// When placeRangeCheck is enabled, this is the range checked for enemy blocks.
    pub place_overlap_range: f32,
    /// Multiplier of damage dealt to this block by tanks. Does not apply to crawlers.
    pub crush_damage_multiplier: f32,
    /// Max of timers used.
    pub timers: u8,
    // /// Cache layer. Only used for 'cached' rendering.
    // pub cache_layer: CacheLayer,
    /// Special flag; if false, floor will be drawn under this block even if it is cached.
    pub fills_tile: bool,
    /// If true, this block can be covered by darkness / fog even if synthetic.
    pub force_dark: bool,
    /// whether this block can be replaced in all cases
    pub always_replace: bool,
    /// if false, this block can never be replaced
    pub replaceable: bool,
    /// The block group. Unless `canReplace` is overridden, blocks in the same group can replace each other
    pub group: BlockGroup,
    /// List of block flags. Used for AI indexing
    pub flags: BlockFlag,
    /// Targeting priority of this block, as seen by enemies
    pub priority: f32,
    /// How much this block affects the unit cap by. The block flags must contain unitModifier in order for this to work
    pub unit_cap_modifier: i32,
    /// Whether the block can be tapped and selected to configure
    pub configurable: bool,
    /// If true, this building can be selected like a unit when commanding
    pub commandable: bool,
    /// If true, the building inventory can be shown with the config
    pub allow_config_inventory: bool,
    /// Defines how large selection menus, such as that of sorters, should be
    pub selection_rows: i32,
    /// Defines how large selection menus, such as that of sorters, should be
    pub selection_columns: i32,
    /// If true, this block can be configured by logic
    pub logic_configurable: bool,
    /// Whether this block consumes touchDown events when tapped
    pub consumes_tap: bool,
    /// Whether to draw the glow of the liquid for this block, if it has one
    pub draw_liquid_light: bool,
    /// Environmental flags that are *all* required for this block to function. 0 = any environment
    pub env_required: i32,
    /// The environment flags that this block can function in. If the env matches any of these, it will be enabled
    pub env_enabled: Env,
    /// The environment flags that this block *cannot* function in. If the env matches any of these, it will be *disabled*
    pub env_disabled: Env,
    /// Whether to periodically sync this block across the network
    pub sync: bool,
    /// Whether this block uses conveyor-type placement mode
    pub conveyor_placement: bool,
    /// If false, diagonal placement (ctrl) for this block is not allowed
    pub allow_diagonal: bool,
    /// Whether to swap the diagonal placement modes
    pub swap_diagonal_placement: bool,
    /// Build queue priority in schematics
    pub schematic_priority: i32,
    /// The color of this block when displayed on the minimap or map preview. Do not set manually! This is overridden when loading for most blocks
    // pub map_color: Color, // = new Color(0, 0, 0, 1);
    /// Whether this block has a minimap color
    pub has_color: bool,
    /// Whether units target this block
    pub targetable: bool,
    /// If true, this block attacks and is considered a turret in the indexer. Building must implement Ranged
    pub attacks: bool,
    /// If true, this block is mending-related and can be suppressed with special units/missiles
    pub suppressable: bool,
    /// Whether the overdrive core has any effect on this block
    pub can_overdrive: bool,
    /// Outlined icon color
    // pub outlineColor: Color, // = Color.valueOf("404049");
    /// Whether any icon region has an outline added
    pub outline_icon: bool,
    /// Outline icon radius
    pub outline_radius: i32,
    /// Which of the icon regions gets the outline added. Uses last icon if <= 0
    pub outlined_icon: i32,
    /// Whether this block has a shadow under it
    pub has_shadow: bool,
    /// If true, a custom shadow (name-shadow) is drawn under this block
    pub custom_shadow: bool,
    /// Should the sound made when this block is built change in pitch
    pub place_pitch_change: bool,
    /// Should the sound made when this block is deconstructed change in pitch
    pub break_pitch_change: bool,
    // /// Sound made when this block is built
    // pub placeSound: Sound,
    // /// Sound made when this block is deconstructed
    // pub breakSound: Sound,
    // /// Sounds made when this block is destroyed
    // pub destroySound: Sound,
    /// How reflective this block is
    pub albedo: f32,
    /// Environmental passive light color
    // pub lightColor: Color, // = Color.white.cpy();
    /// Whether this environmental block passively emits light. Does not change behavior for non-environmental blocks, but still updates clipSize
    pub emit_light: bool,
    /// Radius of the light emitted by this block
    pub light_radius: f32,

    /// How much fog this block uncovers, in tiles. Cannot be dynamic. <= 0 to disable
    pub fog_radius: i32,

    /// The sound that this block makes while active. One sound loop. Do not overuse
    // pub loopSound: Sound,
    /// Active sound base volume
    // pub loopSoundVolume: f32,

    /// The sound that this block makes while idle. Uses one sound loop for all blocks
    // pub ambientSound: Sound,
    /// Idle sound base volume
    pub ambient_sound_volume: f32,

    /// Cost of constructing this block
    pub requirements: Vec<ItemStack>,
    /// Category in place menu
    pub category: Category,
    /// Time to build this block in ticks; do not modify directly!
    pub build_cost: f32,
    /// Whether this block is visible and can currently be built
    pub build_visibility: BuildVisibility,
    /// Multiplier for speed of building this block
    pub build_cost_multiplier: f32,
    /// Build completion at which deconstruction finishes
    pub deconstruct_threshold: f32,
    /// If true, this block deconstructs immediately. Instant deconstruction implies no resource refund
    pub instant_deconstruct: bool,
    /// Effect for placing the block. Passes size as rotation
    // pub placeEffect: Effect,
    // /// Effect for breaking the block. Passes size as rotation
    // pub breakEffect: Effect,
    // /// Effect for destroying the block
    // pub destroyEffect: Effect,
    /// Multiplier for cost of research in tech tree
    pub research_cost_multiplier: f32,
    /// Cost multipliers per-item
    pub research_cost_multipliers: HashMap<Item, f32>,
    /// Override for research cost. Uses multipliers above and building requirements if not set
    pub research_cost: Vec<ItemStack>,
    /// Whether this block has instant transfer
    pub instant_transfer: bool,
    /// Whether you can rotate this block after it is placed
    pub quick_rotate: bool,
    /// Main subclass. Non-anonymous
    pub subclass: Option<Struct>,
    /// Scroll position for certain blocks
    pub select_scroll: f32,
    /// Building that is created for this block. Initialized in init() via reflection. Set manually if modded
    pub build_type: Option<Building>,
    /// Configuration handlers by type
    pub configurations: HashMap<Class, Cons2>,
    /// Consumption filters
    pub item_filter: Vec<bool>, pub liquid_filter: Vec<bool>,
    /// Array of consumers used by this block. Only populated after init()
    pub consumers: Vec<Consume>, pub optional_consumers: Vec<Consume>, pub non_optional_consumers: Vec<Consume>, pub update_consumers: Vec<Consume>,
    /// Set to true if this block has any consumers in its array
    pub has_consumers: bool,
    /// The single power consumer, if applicable
    pub cons_power: Option<ConsumePower>,

    /// Map of bars by name
    pub bar_map: HashMap<String, Func<Building, Bar>>,
    /// List for building up consumption before init()
    pub consume_builder: Vec<Consume>,

    // pub generated_icons: Vec<TextureRegion>,
    // pub editor_variant_regions: Vec<TextureRegion>,

    /// Regions indexes from icons() that are rotated. If either of these is not -1, other regions won't be rotated in ConstructBlocks
    // pub regionRotated1: i32, pub regionRotated2: i32,
    // pub region: TextureRegion, pub editorIcon: TextureRegion,
    // pub customShadowRegion: TextureRegion, pub teamRegion: TextureRegion,
    // pub teamRegions: Vec<TextureRegion>, pub variantRegions: Vec<TextureRegion>, pub variantShadowRegions: Vec<TextureRegion>,

    pub timer_dump: i32,
}