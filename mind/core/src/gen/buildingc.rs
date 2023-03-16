use std::collections::HashSet;
use arc::arc_core::math::geom::vec2::Vec2;
use crate::ctype::content::Content;
use crate::logic::sensible::LAccess;
use crate::r#type::item::Item;
use crate::r#type::liquid::Liquid;

/// Interface for {@link mindustry.entities.comp.BuildingComp}
pub trait Buildingc {
    /// #Returns
    /// the building's 'warmup', a smooth value from 0 to 1.
    /// usually used for crafters and things that need to spin up before reaching full efficiency.
    /// many blocks will just return 0.
    fn warmup() -> f32;

    /// Called when a block is placed over some other blocks. This seq will always have at least one item.
    /// Should load some previous state, if necessary.
    fn overwrote(previous: HashSet<Building>);

    /// Called when a position is tapped while this building is selected.
    /// 
    /// #Returns
    /// whether the tap event is consumed - if true, the player will not start shooting or interact with things under the cursor.
    fn on_configure_tapped(x: f32, y: f32) -> bool;

    /// Called when another tile is tapped while this building is selected.
    /// #Returns
    /// whether this block should be deselected.
    fn on_configure_build_tapped(other: Building) -> bool;

    /// Called when this block's config menu is closed.
    fn on_configure_closed();

    /// Tries dumping a payload to any adjacent block.
    /// @param todump payload to dump.
    /// #Returns
    /// whether the payload was moved successfully
    fn dump_payload(todump: Payload) -> bool;

    /// Tries moving a payload forwards.
    /// @param todump payload to dump.
    /// #Returns
    /// whether the payload was moved successfully
    fn move_payload(todump: Payload) -> bool;

    /// Tries to put this item into a nearby container, if there are no available
    /// containers, it gets added to the block's inventory.
    fn offload(item: Item);

    /// Tries to put this item into a nearby container. Returns success. Unlike #offload(), this method does not change the block inventory.
    fn put(item: Item) -> bool;

    /// Try dumping a specific item near the building.
    /// @param todump Item to dump. Can be null to dump anything.
    fn dump_item_todump(todump: Item) -> bool;

    /// @param outputDir output liquid direction relative to rotation, or -1 to use any direction.
    fn dump_liquid_liquid_liquid(liquid: Liquid, scaling: f32, output_dir: i32);

    /// #Returns
    /// ambient sound volume scale.
    fn ambient_volume() -> f32;

    /// #Returns
    /// preferred rotation of main texture region to be drawn
    fn drawrot() -> f32;

    /// #Returns
    /// the cap for item amount calculations, used when this block explodes.
    fn explosion_item_cap() -> i32;

    /// #Returns
    /// the item module to use for flow rate calculations
    fn flow_items() -> ItemModule;

    /// #Returns
    /// the position that this block points to for commands, or null.
    fn get_command_position() -> Vec2;

    /// #Returns
    /// total time this block has been producing something; non-crafter blocks usually return Time.time.
    fn total_progress() -> f32;

    /// #Returns
    /// volume cale of active sound.
    fn active_sound_volume() -> f32;

    /// #Returns
    /// whether a building has regen/healing suppressed; if so, spawns particles on it.
    fn check_suppression() -> bool;

    /// #Returns
    /// whether the player can select (but not actually control) this building.
    fn can_control_select(player: Unit) -> bool;

    /// #Returns
    /// whether this block is allowed to update based on team/environment
    fn allow_update() -> bool;

    /// #Returns
    /// whether this block is currently "active" and should be consuming requirements.
    fn should_consume() -> bool;

    /// #Returns
    /// whether this block should play its active sound.
    fn should_active_sound() -> bool;

    /// #Returns
    /// whether this block should play its idle sound.
    fn should_ambient_sound() -> bool;

    /// #Returns
    /// whether this building is currently "burning" a trigger consumer (an item) - if true, valid() on those will return true.
    fn consume_trigger_valid() -> bool;

    /// #Returns
    /// whether this building is in a payload
    fn is_payload() -> bool;

    /// Any class that overrides this method and changes the value must call Vars.fogControl.forceUpdate(team).
    fn fog_radius() -> f32;

    /// Base efficiency. Takes the minimum value of all consumers.
    fn efficiency() -> f32;

    /// Base efficiency. Takes the minimum value of all consumers.
    fn efficiency_float_efficiency(efficiency: f32);

    /// Calculate your own efficiency multiplier. By default, this is applied in updateEfficiencyMultiplier.
    fn efficiency_scale() -> f32;

    /// Call when nothing is happening to the entity. This increments the internal sleep timer.
    fn sleep();

    /// Call when this entity is updating. This wakes it up.
    fn no_sleep();

    /// Called after the tile has been removed.
    fn after_destroyed();

    /// Called after efficiency is updated but before consumers are updated. Use to apply your own multiplier.
    fn update_efficiency_multiplier();

    /// Called after the block is placed by anyone.
    fn placed();

    /// Called after the block is placed by this client.
    fn player_placed(config: Object);

    /// Called after this building is created in the world. May be called multiple times, or when adjacent buildings change.
    fn on_proximity_added();

    /// Called clientside when the client taps a block to config.
    /// #Returns
    /// whether the configuration UI should be shown.
    fn config_tapped() -> bool;

    /// Called every frame a unit is on this
    fn unit_on(unit: Unit);

    /// Called right after this building is picked up.
    fn after_picked_up();

    /// Called right before this building is picked up.
    fn picked_up();

    /// Called shortly before this building is removed.
    fn on_proximity_removed();

    /// Called when a player control-selects this building - not called for ControlBlock subclasses.
    fn on_control_select(player: Unit);

    /// Called when a unit that spawned at this tile is removed.
    fn unit_removed(unit: Unit);

    /// Called when an unloader takes an item.
    fn item_taken(item: Item);

    /// Called when anything adjacent to this building is placed/removed, including itself.
    fn on_proximity_update();

    /// Called when arbitrary configuration is applied to a tile.
    fn configured(builder: Unit, value: Object);

    /// Called when the block is destroyed. The tile is still intact at this stage.
    fn on_destroyed();

    /// Called when the block is tapped by the local player.
    fn tapped();

    /// Called when this block is dropped as a payload.
    fn dropped();

    /// Called when this block is tapped to build a UI on the table.
    /// configurable must be true for this to be called.
    fn build_configuration(table: Table);

    /// Called when this building receives a position command. Requires a commandable block.
    fn on_command(target: Vec2);

    /// Changes this building's team in a safe manner.
    fn change_team(next: Team);

    /// Configure from a server.
    fn configure_any(value: Object);

    /// Configure with the current, local player.
    fn configure(value: Object);

    /// Deselect this tile from configuration.
    fn deselect();

    /// Draw the block overlay that is shown when a cursor is over the block.
    fn draw_select();

    /// Dumps any item with an accumulator. May dump multiple times per frame. Use with care.
    fn dump_accumulate() -> bool;

    /// Dumps any item with an accumulator. May dump multiple times per frame. Use with care.
    fn dump_accumulate_item_item(item: Item) -> bool;

    /// Efficiency  delta.
    fn edelta() -> f32;

    /// Handle a bullet collision.
    /// #Returns
    /// whether the bullet should be removed.
    fn collision(other: Bullet) -> bool;

    /// Handle a stack input.
    fn handle_stack(item: Item, amount: i32, source: Teamc);

    /// Handles splash damage with a bullet source.
    fn damage_bullet_bullet(bullet: Bullet, source: Team, damage: f32);

    /// Multiblock back.
    fn back() -> Building;

    /// Multiblock front.
    fn front() -> Building;

    /// Multiblock left.
    fn left() -> Building;

    /// Multiblock right.
    fn right() -> Building;

    /// Remove a stack from this inventory, and return the amount removed.
    fn remove_stack(item: Item, amount: i32) -> i32;

    /// Returns offset for stack placement.
    fn get_stack_offset(item: Item, trns: Vec2);

    /// Returns the amount of items this block can accept.
    fn accept_stack(item: Item, amount: i32, source: Teamc) -> i32;

    /// Returns the version of this Building IO code.
    fn version() -> u8;

    /// Returns whether a hand cursor should be shown over this block.
    fn get_cursor() -> Graphics.Cursor;

    /// Returns whether this config menu should show when the specified player taps it.
    fn should_show_configure(player: Player) -> bool;

    /// Same as efficiency, but for optional consumers only.
    fn optional_efficiency() -> f32;

    /// Same as efficiency, but for optional consumers only.
    fn optional_efficiency_float_optional_efficiency(optional_efficiency: f32);

    /// Scaled delta.
    fn delta() -> f32;

    /// Sets this tile entity data to this and adds it if necessary.
    fn init(tile: Tile, team: Team, should_add: bool, rotation: i32) -> Building;

    /// Sets up all the necessary variables, but does not add this entity anywhere.
    fn create(block: Block, team: Team) -> Building;

    /// TODO Each bit corresponds to a team ID. Only 64 are supported. Does not work on servers.
    fn visible_flags() -> i64;

    /// TODO Each bit corresponds to a team ID. Only 64 are supported. Does not work on servers.
    fn visible_flags_long_visible_flags(visible_flags: i64);

    /// The efficiency this block would have if shouldConsume() returned true.
    fn potential_efficiency() -> f32;

    /// The efficiency this block would have if shouldConsume() returned true.
    fn potential_efficiency_float_potential_efficiency(potential_efficiency: f32);

    /// This is for logic blocks.
    fn handle_string(value: Object);

    /// Tile configuration. Defaults to null. Used for block rebuilding.
    fn config() -> Object;

    /// Tries to take the payload. Returns null if no payload is present.
    fn take_payload() -> Payload;

    /// Try dumping any item near the building.
    fn dump() -> bool;

    /// Try offloading an item to a nearby container in its facing direction. Returns true if success.
    fn move_forward(item: Item) -> bool;

    /// Update table alignment after configuring.
    fn update_table_align(table: Table);

    /// Used for dumping items.
    fn can_dump(to: Building, item: Item) -> bool;

    /// Used to handle damage from splash damage for certain types of blocks.
    fn damage_team_source(source: Team, damage: f32);

    /// Whether this configuration should be hidden now. Called every frame the config is open.
    fn should_hide_configure(player: Player) -> bool;

    fn last_disabler() -> Option<Building>;

    fn items() -> Option<ItemModule>;

    fn liquids() -> Option<LiquidModule>;

    fn power() -> Option<PowerModule>;

    fn get_liquid_destination(from: Building, liquid: Liquid) -> Building;

    fn nearby_int_dx(dx: i32, dy: i32) -> Building;

    fn nearby_int_rotation(rotation: i32) -> Building;

    fn get_display_icon() -> TextureRegion;

    fn get_power_connections(out: HashSet<Building>) -> HashSet<Building>;

    fn proximity() -> HashSet<Building>;

    fn absorb_lasers() -> bool;

    fn accept_item(source: Building, item: Item) -> bool;

    fn accept_liquid(source: Building, liquid: Liquid) -> bool;

    fn accept_payload(source: Building, payload: Payload) -> bool;

    fn can_consume() -> bool;

    fn can_dump_liquid(to: Building, liquid: Liquid) -> bool;

    fn can_pickup() -> bool;

    fn can_resupply() -> bool;

    fn can_unload() -> bool;

    fn can_withdraw() -> bool;

    fn check_solid() -> bool;

    fn collide(other: Bullet) -> bool;

    fn conducts_to(other: Building) -> bool;

    fn enabled() -> bool;

    fn in_fog_to(viewer: Team) -> bool;

    fn interactable(team: Team) -> bool;

    fn is_discovered(viewer: Team) -> bool;

    fn is_heal_suppressed() -> bool;

    fn is_insulated() -> bool;

    fn is_valid() -> bool;

    fn payload_check(conveyor_rotation: i32) -> bool;

    fn production_valid() -> bool;

    fn was_damaged() -> bool;

    fn was_recently_damaged() -> bool;

    fn was_recently_healed(duration: f32) -> bool;

    fn was_visible() -> bool;

    fn relative_to_building_build(build: Building) -> u8;

    fn relative_to_int_cx(cx: i32, cy: i32) -> u8;

    fn relative_to_tile_tile(tile: Tile) -> u8;

    fn relative_to_edge(other: Tile) -> u8;

    fn sense_content_content(content: Content) -> f64;

    fn sense_l_access_sensor(sensor: LAccess) -> f64;

    fn calculate_heat_float[]_side_heat(side_heat: float[]) -> f32;

    fn calculate_heat_float[]_side_heat(side_heat: float[], came_from: IntSet) -> f32;

    fn get_display_efficiency() -> f32;

    fn get_power_production() -> f32;

    fn get_progress_increase(base_time: f32) -> f32;

    fn handle_damage(amount: f32) -> f32;

    fn heal_suppression_time() -> f32;

    fn hit_size() -> f32;

    fn last_heal_time() -> f32;

    fn move_liquid(next: Building, liquid: Liquid) -> f32;

    fn move_liquid_forward(leaks: bool, liquid: Liquid) -> f32;

    fn payload_rotation() -> f32;

    fn progress() -> f32;

    fn rotdeg() -> f32;

    fn time_scale() -> f32;

    fn visual_liquid() -> f32;

    fn cdump() -> i32;

    fn get_maximum_accepted(item: Item) -> i32;

    fn module_bitmask() -> i32;

    fn pos() -> i32;

    fn rotation() -> i32;

    fn tile_x() -> i32;

    fn tile_y() -> i32;

    fn sense_object(sensor: LAccess) -> Object;

    fn get_display_name() -> String;

    fn last_accessed() -> String;

    fn to_string() -> String;

    fn get_payloads() -> PayloadSeq;

    fn block() -> Block;

    fn find_closest_edge(to: Position, solid: Boolf<Tile>) -> Tile;

    fn tile() -> Tile;

    fn floor() -> Floor;

    fn get_payload() -> Payload;

    fn status() -> BlockStatus;

    fn add();

    fn add_plan_boolean_check_previous(check_previous: bool);

    fn add_plan_boolean_check_previous(check_previous: bool, ignore_conditions: bool);

    fn apply_boost(intensity: f32, duration: f32);

    fn apply_heal_suppression(amount: f32);

    fn apply_slowdown(intensity: f32, duration: f32);

    fn block_block_block(block: Block);

    fn cdump_int_cdump(cdump: i32);

    fn consume();

    fn control_l_access_type(type: LAccess, p_1: f64, p_2: f64, p_3: f64, p_4: f64);

    fn control_l_access_type(type: LAccess, p_1: Object, p_2: f64, p_3: f64, p_4: f64);

    fn created();

    fn damage_float_damage(damage: f32);

    fn display(table: Table);

    fn display_bars(table: Table);

    fn display_consumption(table: Table);

    fn draw();

    fn draw_configure();

    fn draw_cracks();

    fn draw_disabled();

    fn draw_light();

    fn draw_liquid_light(liquid: Liquid, amount: f32);

    fn draw_status();

    fn draw_team();

    fn draw_team_top();

    fn dump_liquid_liquid_liquid(liquid: Liquid);

    fn dump_liquid_liquid_liquid(liquid: Liquid, scaling: f32);

    fn enabled_boolean_enabled(enabled: bool);

    fn handle_item(source: Building, item: Item);

    fn handle_liquid(source: Building, liquid: Liquid, amount: f32);

    fn handle_payload(source: Building, payload: Payload);

    fn handle_unit_payload(unit: Unit, grabber: Cons<Payload>);

    fn heal();

    fn heal_float_amount(amount: f32);

    fn heal_suppression_time_float_heal_suppression_time(heal_suppression_time: f32);

    fn health_changed();

    fn hitbox(out: Rect);

    fn increment_dump(prox: i32);

    fn items_@nullable_item_module_items(items: Option<ItemModule>);

    fn kill();

    fn killed();

    fn last_accessed_string_last_accessed(last_accessed: String);

    fn last_disabler_@nullable_building_last_disabler(last_disabler: Option<Building>);

    fn last_heal_time_float_last_heal_time(last_heal_time: f32);

    fn liquids_@nullable_liquid_module_liquids(liquids: Option<LiquidModule>);

    fn on_removed();

    fn payload_draw();

    fn payload_rotation_float_payload_rotation(payload_rotation: f32);

    fn power_@nullable_power_module_power(power: Option<PowerModule>);

    fn power_graph_removed();

    fn produced_item_item(item: Item);

    fn produced_item_item(item: Item, amount: i32);

    fn proximity_seq<building>_proximity(proximity: HashSet<Building>);

    fn read(read: Reads, revision: u8);

    fn read_all(read: Reads, revision: u8);

    fn read_base(read: Reads);

    fn recently_healed();

    fn remove();

    fn remove_from_proximity();

    fn rotation_int_rotation(rotation: i32);

    fn tile_tile_tile(tile: Tile);

    fn transfer_liquid(next: Building, amount: f32, liquid: Liquid);

    fn update();

    fn update_consumption();

    fn update_payload(unit_holder: Unit, building_holder: Building);

    fn update_power_graph();

    fn update_proximity();

    fn update_tile();

    fn visual_liquid_float_visual_liquid(visual_liquid: f32);

    fn was_damaged_boolean_was_damaged(was_damaged: bool);

    fn was_visible_boolean_was_visible(was_visible: bool);

    fn write(write: Writes);

    fn write_all(write: Writes);

    fn write_base(write: Writes);
}
