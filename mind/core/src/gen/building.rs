use std::collections::HashSet;
use arc::arc_core::math::geom::position::Position;

use arc::arc_core::math::geom::vec2::Vec2;

use crate::gen::buildingc::Buildingc;
use crate::gen::entityc::Entityc;
use crate::gen::healthc::Healthc;
use crate::gen::posc::Posc;
use crate::gen::teamc::Teamc;
use crate::gen::timerc::Timerc;
use crate::r#type::item::Item;
use crate::r#type::liquid::Liquid;
use crate::world::block::Block;

pub static BULLET_DAMAGE_EVENT: BuildDamageEvent = BuildDamageEvent::default();
pub static HIT_DURATION: f32 = 9.0;
pub static RECENT_DAMAGE_TIME: f32 = 60.0 * 5.0;
pub static SLEEPING_ENTITIES: i32 = 0;
pub static TEAM_CHANGE_EVENT: BuildTeamChangeEvent = BuildTeamChangeEvent::default();
pub static TEMP_BUILDS: HashSet<Building> = HashSet::new();
pub static TIME_TO_SLEEP: f32 = 60.0 * 1;
pub static TMP_TILES: HashSet<Building> = HashSet::new();

pub struct Building {
    added: bool,
    pub block: Block,
    pub cdump: i32,
    pub dead: bool,
    dump_accum: f32,
    pub efficiency: f32,
    pub enabled: bool,
    pub heal_suppression_time: f32,
    pub health: f32,
    pub hit_time: f32,
    pub id: i32,
    index_all: i32,
    index_build: i32,
    initialized: bool,
    pub items: Option<ItemModule>,
    pub last_accessed: String,
    last_damage_time: f32,
    pub last_disabler: Option<Building>,
    pub last_heal_time: f32,
    pub liquids: Option<LiquidModule>,
    pub max_health: f32,
    pub optional_efficiency: f32,
    pub payload_rotation: f32,
    pub potential_efficiency: f32,
    pub power: Option<PowerModule>,
    pub proximity: HashSet<Building>,
    pub rotation: i32,
    sleep_time: f32,
    sleeping: bool,
    pub team: Team,
    pub tile: Tile,
    time_scale: f32,
    time_scale_duration: f32,
    pub timer: Interval,
    pub visible_flags: i64,
    pub visual_liquid: f32,
    pub was_damaged: bool,
    pub was_visible: bool,
    pub x: f32,
    pub y: f32,
}

impl Default for Building {
    fn default() -> Building {
        Building {
            added: bool::default(),
            block: Block::default(),
            cdump: i32::default(),
            dead: bool::default(),
            dump_accum: f32::default(),
            efficiency: f32::default(),
            enabled: true,
            heal_suppression_time: -1.0,
            health: f32::default(),
            hit_time: f32::default(),
            id: EntityGroup.nextId(),
            index_all: -1,
            index_build: -1,
            initialized: bool::default(),
            items: None,
            last_accessed: String::default(),
            last_damage_time: -RECENT_DAMAGE_TIME.clone(),
            last_disabler: None,
            last_heal_time: -120.0 * 10.0,
            liquids: None,
            max_health: 1.0,
            optional_efficiency: f32::default(),
            payload_rotation: f32::default(),
            potential_efficiency: f32::default(),
            power: None,
            proximity: HashSet::new(),
            rotation: i32::default(),
            sleep_time: f32::default(),
            sleeping: bool::default(),
            team: Team.derelict,
            tile: Tile::default(),
            time_scale: 1.0,
            time_scale_duration: f32::default(),
            timer: new Interval(6),
            visible_flags: i64::default(),
            visual_liquid: f32::default(),
            was_damaged: bool::default(),
            was_visible: bool::default(),
            x: f32::default(),
            y: f32::default(),
        }
    }
}

impl Entityc for Building {
    fn is_added() -> bool {
        todo!()
    }

    fn is_local() -> bool {
        todo!()
    }

    fn is_null() -> bool {
        todo!()
    }

    fn is_remote() -> bool {
        todo!()
    }

    fn serialize() -> bool {
        todo!()
    }

    fn class_id() -> i32 {
        todo!()
    }

    fn id() -> i32 {
        todo!()
    }

    fn add() {
        todo!()
    }

    fn after_read() {
        todo!()
    }

    fn id_id(id: i32) {
        todo!()
    }

    fn read(read: Reads) {
        todo!()
    }

    fn remove() {
        todo!()
    }

    fn update() {
        todo!()
    }

    fn write(write: Writes) {
        todo!()
    }
}

impl Healthc for Building {
    fn damage_pierce(amount: f32) {
        todo!()
    }

    fn damage_pierce_amount(amount: f32, with_effect: bool) {
        todo!()
    }

    fn heal_fract(amount: f32) {
        todo!()
    }

    fn heal_amount(amount: f32) {
        todo!()
    }

    fn damaged() -> bool {
        todo!()
    }

    fn dead() -> bool {
        todo!()
    }

    fn is_valid() -> bool {
        todo!()
    }

    fn health() -> f32 {
        todo!()
    }

    fn healthf() -> f32 {
        todo!()
    }

    fn hit_time() -> f32 {
        todo!()
    }

    fn max_health() -> f32 {
        todo!()
    }

    fn clamp_health() {
        todo!()
    }

    fn damage(amount: f32) {
        todo!()
    }

    fn damage_amount(amount: f32, with_effect: bool) {
        todo!()
    }

    fn damage_continuous(amount: f32) {
        todo!()
    }

    fn damage_continuous_pierce(amount: f32) {
        todo!()
    }

    fn dead_dead(dead: bool) {
        todo!()
    }

    fn heal() {
        todo!()
    }

    fn health_health(health: f32) {
        todo!()
    }

    fn hit_time_hit_time(hit_time: f32) {
        todo!()
    }

    fn kill() {
        todo!()
    }

    fn killed() {
        todo!()
    }

    fn max_health_max_health(max_health: f32) {
        todo!()
    }

    fn update() {
        todo!()
    }
}

impl Posc for Building {
    fn floor_on() -> Floor {
        todo!()
    }

    fn build_on() -> Building {
        todo!()
    }

    fn on_solid() -> bool {
        todo!()
    }

    fn get_x() -> f32 {
        todo!()
    }

    fn get_y() -> f32 {
        todo!()
    }

    fn x() -> f32 {
        todo!()
    }

    fn y() -> f32 {
        todo!()
    }

    fn tile_x() -> i32 {
        todo!()
    }

    fn tile_y() -> i32 {
        todo!()
    }

    fn block_on() -> Block {
        todo!()
    }

    fn tile_on() -> Tile {
        todo!()
    }

    fn set(pos: Position) {
        todo!()
    }

    fn set_x(x: f32, y: f32) {
        todo!()
    }

    fn trns(pos: Position) {
        todo!()
    }

    fn trns_x(x: f32, y: f32) {
        todo!()
    }

    fn x_x(x: f32) {
        todo!()
    }

    fn y_y(y: f32) {
        todo!()
    }
}

impl Teamc for Building {
    fn in_fog_to(viewer: Team) -> bool {
        todo!()
    }

    fn cheating() -> bool {
        todo!()
    }

    fn team() -> Team {
        todo!()
    }

    fn closest_core() -> CoreBuild {
        todo!()
    }

    fn closest_enemy_core() -> CoreBuild {
        todo!()
    }

    fn core() -> CoreBuild {
        todo!()
    }

    fn team_team(team: Team) {
        todo!()
    }
}

impl Timerc for Building {
    fn timer() -> Interval {
        todo!()
    }

    fn timer_index(index: i32, time: f32) -> bool {
        todo!()
    }

    fn timer_timer(timer: Interval) {
        todo!()
    }
}

impl Buildingc for Building {
    fn warmup() -> f32 {
        todo!()
    }

    fn overwrote(previous: HashSet<Building>) {
        todo!()
    }

    fn on_configure_tapped(x: f32, y: f32) -> bool {
        todo!()
    }

    fn on_configure_build_tapped(other: Building) -> bool {
        todo!()
    }

    fn on_configure_closed() {
        todo!()
    }

    fn dump_payload(todump: Payload) -> bool {
        todo!()
    }

    fn move_payload(todump: Payload) -> bool {
        todo!()
    }

    fn offload(item: Item) {
        todo!()
    }

    fn put(item: Item) -> bool {
        todo!()
    }

    fn dump_liquid(liquid: Liquid, scaling: f32, output_dir: i32) {
        todo!()
    }

    fn ambient_volume() -> f32 {
        todo!()
    }

    fn drawrot() -> f32 {
        todo!()
    }

    fn explosion_item_cap() -> i32 {
        todo!()
    }

    fn flow_items() -> ItemModule {
        todo!()
    }

    fn get_command_position() -> Vec2 {
        todo!()
    }

    fn total_progress() -> f32 {
        todo!()
    }

    fn active_sound_volume() -> f32 {
        todo!()
    }

    fn check_suppression() -> bool {
        todo!()
    }

    fn can_control_select(player: Unit) -> bool {
        todo!()
    }

    fn allow_update() -> bool {
        todo!()
    }

    fn should_consume() -> bool {
        todo!()
    }

    fn should_active_sound() -> bool {
        todo!()
    }

    fn should_ambient_sound() -> bool {
        todo!()
    }

    fn consume_trigger_valid() -> bool {
        todo!()
    }

    fn is_payload() -> bool {
        todo!()
    }

    fn fog_radius() -> f32 {
        todo!()
    }

    fn efficiency() -> f32 {
        todo!()
    }

    fn efficiency_efficiency(efficiency: f32) {
        todo!()
    }

    fn efficiency_scale() -> f32 {
        todo!()
    }

    fn sleep() {
        todo!()
    }

    fn no_sleep() {
        todo!()
    }

    fn after_destroyed() {
        todo!()
    }

    fn update_efficiency_multiplier() {
        todo!()
    }

    fn placed() {
        todo!()
    }

    fn player_placed(config: Object) {
        todo!()
    }

    fn on_proximity_added() {
        todo!()
    }

    fn config_tapped() -> bool {
        todo!()
    }

    fn unit_on(unit: Unit) {
        todo!()
    }

    fn after_picked_up() {
        todo!()
    }

    fn picked_up() {
        todo!()
    }

    fn on_proximity_removed() {
        todo!()
    }

    fn on_control_select(player: Unit) {
        todo!()
    }

    fn unit_removed(unit: Unit) {
        todo!()
    }

    fn item_taken(item: Item) {
        todo!()
    }

    fn on_proximity_update() {
        todo!()
    }

    fn configured(builder: Unit, value: Object) {
        todo!()
    }

    fn on_destroyed() {
        todo!()
    }

    fn tapped() {
        todo!()
    }

    fn dropped() {
        todo!()
    }

    fn build_configuration(table: Table) {
        todo!()
    }

    fn on_command(target: Vec2) {
        todo!()
    }

    fn change_team(next: Team) {
        todo!()
    }

    fn configure_any(value: Object) {
        todo!()
    }

    fn configure(value: Object) {
        todo!()
    }

    fn deselect() {
        todo!()
    }

    fn draw_select() {
        todo!()
    }

    fn dump_accumulate() -> bool {
        todo!()
    }

    fn dump_accumulate_item(item: Item) -> bool {
        todo!()
    }

    fn edelta() -> f32 {
        todo!()
    }

    fn collision(other: Bullet) -> bool {
        todo!()
    }

    fn handle_stack(item: Item, amount: i32, source: Teamc) {
        todo!()
    }

    fn damage(bullet: Bullet, source: Team, damage: f32) {
        todo!()
    }

    fn back() -> Building {
        todo!()
    }

    fn front() -> Building {
        todo!()
    }

    fn left() -> Building {
        todo!()
    }

    fn right() -> Building {
        todo!()
    }

    fn remove_stack(item: Item, amount: i32) -> i32 {
        todo!()
    }

    fn get_stack_offset(item: Item, trns: Vec2) {
        todo!()
    }

    fn accept_stack(item: Item, amount: i32, source: Teamc) -> i32 {
        todo!()
    }

    fn version() -> u8 {
        todo!()
    }

    fn get_cursor() -> Graphics {
        todo!()
    }

    fn should_show_configure(player: Player) -> bool {
        todo!()
    }

    fn optional_efficiency() -> f32 {
        todo!()
    }

    fn optional_efficiency_optional_efficiency(optional_efficiency: f32) {
        todo!()
    }

    fn delta() -> f32 {
        todo!()
    }

    fn init(tile: Tile, team: Team, should_add: bool, rotation: i32) -> Building {
        todo!()
    }

    fn create(block: Block, team: Team) -> Building {
        todo!()
    }

    fn visible_flags() -> i64 {
        todo!()
    }

    fn visible_flags_visible_flags(visible_flags: i64) {
        todo!()
    }

    fn potential_efficiency() -> f32 {
        todo!()
    }

    fn potential_efficiency_potential_efficiency(potential_efficiency: f32) {
        todo!()
    }

    fn handle_string(value: Object) {
        todo!()
    }

    fn config() -> Object {
        todo!()
    }

    fn take_payload() -> Payload {
        todo!()
    }

    fn dump() -> bool {
        todo!()
    }

    fn dump_todump(todump: Item) -> bool {
        todo!()
    }

    fn move_forward(item: Item) -> bool {
        todo!()
    }

    fn update_table_align(table: Table) {
        todo!()
    }

    fn can_dump(to: Building, item: Item) -> bool {
        todo!()
    }

    fn damage_source(source: Team, damage: f32) {
        todo!()
    }

    fn should_hide_configure(player: Player) -> bool {
        todo!()
    }

    fn last_disabler() -> Option<Building> {
        todo!()
    }

    fn items() -> Option<ItemModule> {
        todo!()
    }

    fn liquids() -> Option<LiquidModule> {
        todo!()
    }

    fn power() -> Option<PowerModule> {
        todo!()
    }

    fn get_liquid_destination(from: Building, liquid: Liquid) -> Building {
        todo!()
    }

    fn nearby(dx: i32, dy: i32) -> Building {
        todo!()
    }

    fn nearby_rotation(rotation: i32) -> Building {
        todo!()
    }

    fn get_display_icon() -> TextureRegion {
        todo!()
    }

    fn get_power_connections(out: HashSet<Building>) -> HashSet<Building> {
        todo!()
    }

    fn proximity() -> HashSet<Building> {
        todo!()
    }

    fn absorb_lasers() -> bool {
        todo!()
    }

    fn accept_item(source: Building, item: Item) -> bool {
        todo!()
    }

    fn accept_liquid(source: Building, liquid: Liquid) -> bool {
        todo!()
    }

    fn accept_payload(source: Building, payload: Payload) -> bool {
        todo!()
    }

    fn can_consume() -> bool {
        todo!()
    }

    fn can_dump_liquid(to: Building, liquid: Liquid) -> bool {
        todo!()
    }

    fn can_pickup() -> bool {
        todo!()
    }

    fn can_resupply() -> bool {
        todo!()
    }

    fn can_unload() -> bool {
        todo!()
    }

    fn can_withdraw() -> bool {
        todo!()
    }

    fn check_solid() -> bool {
        todo!()
    }

    fn collide(other: Bullet) -> bool {
        todo!()
    }

    fn conducts_to(other: Building) -> bool {
        todo!()
    }

    fn enabled() -> bool {
        todo!()
    }

    fn in_fog_to(viewer: Team) -> bool {
        todo!()
    }

    fn interactable(team: Team) -> bool {
        todo!()
    }

    fn is_discovered(viewer: Team) -> bool {
        todo!()
    }

    fn is_heal_suppressed() -> bool {
        todo!()
    }

    fn is_insulated() -> bool {
        todo!()
    }

    fn is_valid() -> bool {
        todo!()
    }

    fn payload_check(conveyor_rotation: i32) -> bool {
        todo!()
    }

    fn production_valid() -> bool {
        todo!()
    }

    fn was_damaged() -> bool {
        todo!()
    }

    fn was_recently_damaged() -> bool {
        todo!()
    }

    fn was_recently_healed(duration: f32) -> bool {
        todo!()
    }

    fn was_visible() -> bool {
        todo!()
    }

    fn relative_to(build: Building) -> u8 {
        todo!()
    }

    fn relative_to_cx(cx: i32, cy: i32) -> u8 {
        todo!()
    }

    fn relative_to_tile(tile: Tile) -> u8 {
        todo!()
    }

    fn relative_to_edge(other: Tile) -> u8 {
        todo!()
    }

    fn sense(content: Content) -> f64 {
        todo!()
    }

    fn sense_sensor(sensor: LAccess) -> f64 {
        todo!()
    }

    fn calculate_heat(side_heat: float, _: []) -> f32 {
        todo!()
    }

    fn calculate_heat_side_heat(side_heat: float, _: [], came_from: IntSet) -> f32 {
        todo!()
    }

    fn get_display_efficiency() -> f32 {
        todo!()
    }

    fn get_power_production() -> f32 {
        todo!()
    }

    fn get_progress_increase(base_time: f32) -> f32 {
        todo!()
    }

    fn handle_damage(amount: f32) -> f32 {
        todo!()
    }

    fn heal_suppression_time() -> f32 {
        todo!()
    }

    fn hit_size() -> f32 {
        todo!()
    }

    fn last_heal_time() -> f32 {
        todo!()
    }

    fn move_liquid(next: Building, liquid: Liquid) -> f32 {
        todo!()
    }

    fn move_liquid_forward(leaks: bool, liquid: Liquid) -> f32 {
        todo!()
    }

    fn payload_rotation() -> f32 {
        todo!()
    }

    fn progress() -> f32 {
        todo!()
    }

    fn rotdeg() -> f32 {
        todo!()
    }

    fn time_scale() -> f32 {
        todo!()
    }

    fn visual_liquid() -> f32 {
        todo!()
    }

    fn cdump() -> i32 {
        todo!()
    }

    fn get_maximum_accepted(item: Item) -> i32 {
        todo!()
    }

    fn module_bitmask() -> i32 {
        todo!()
    }

    fn pos() -> i32 {
        todo!()
    }

    fn rotation() -> i32 {
        todo!()
    }

    fn tile_x() -> i32 {
        todo!()
    }

    fn tile_y() -> i32 {
        todo!()
    }

    fn sense_object(sensor: LAccess) -> Object {
        todo!()
    }

    fn get_display_name() -> String {
        todo!()
    }

    fn last_accessed() -> String {
        todo!()
    }

    fn to_string() -> String {
        todo!()
    }

    fn get_payloads() -> PayloadSeq {
        todo!()
    }

    fn block() -> Block {
        todo!()
    }

    fn find_closest_edge(to: Position, solid: Boolf<Tile>) -> Tile {
        todo!()
    }

    fn tile() -> Tile {
        todo!()
    }

    fn floor() -> Floor {
        todo!()
    }

    fn get_payload() -> Payload {
        todo!()
    }

    fn status() -> BlockStatus {
        todo!()
    }

    fn add() {
        todo!()
    }

    fn add_plan(check_previous: bool) {
        todo!()
    }

    fn add_plan_check_previous(check_previous: bool, ignore_conditions: bool) {
        todo!()
    }

    fn apply_boost(intensity: f32, duration: f32) {
        todo!()
    }

    fn apply_heal_suppression(amount: f32) {
        todo!()
    }

    fn apply_slowdown(intensity: f32, duration: f32) {
        todo!()
    }

    fn block_block(block: Block) {
        todo!()
    }

    fn cdump_cdump(cdump: i32) {
        todo!()
    }

    fn consume() {
        todo!()
    }

    fn control(_: LAccess, p_1: f64, p_2: f64, p_3: f64, p_4: f64) {
        todo!()
    }

    fn control_type(_: LAccess, p_1: Object, p_2: f64, p_3: f64, p_4: f64) {
        todo!()
    }

    fn created() {
        todo!()
    }

    fn damage_damage(damage: f32) {
        todo!()
    }

    fn display(table: Table) {
        todo!()
    }

    fn display_bars(table: Table) {
        todo!()
    }

    fn display_consumption(table: Table) {
        todo!()
    }

    fn draw() {
        todo!()
    }

    fn draw_configure() {
        todo!()
    }

    fn draw_cracks() {
        todo!()
    }

    fn draw_disabled() {
        todo!()
    }

    fn draw_light() {
        todo!()
    }

    fn draw_liquid_light(liquid: Liquid, amount: f32) {
        todo!()
    }

    fn draw_status() {
        todo!()
    }

    fn draw_team() {
        todo!()
    }

    fn draw_team_top() {
        todo!()
    }

    fn dump_liquid_liquid(liquid: Liquid) {
        todo!()
    }

    fn dump_liquid_liquid_scaling(liquid: Liquid, scaling: f32) {
        todo!()
    }

    fn enabled_enabled(enabled: bool) {
        todo!()
    }

    fn handle_item(source: Building, item: Item) {
        todo!()
    }

    fn handle_liquid(source: Building, liquid: Liquid, amount: f32) {
        todo!()
    }

    fn handle_payload(source: Building, payload: Payload) {
        todo!()
    }

    fn handle_unit_payload(unit: Unit, grabber: Cons<Payload>) {
        todo!()
    }

    fn heal() {
        todo!()
    }

    fn heal_amount(amount: f32) {
        todo!()
    }

    fn heal_suppression_time_heal_suppression_time(heal_suppression_time: f32) {
        todo!()
    }

    fn health_changed() {
        todo!()
    }

    fn hitbox(out: Rect) {
        todo!()
    }

    fn increment_dump(prox: i32) {
        todo!()
    }

    fn items_items(items: Option<ItemModule>) {
        todo!()
    }

    fn kill() {
        todo!()
    }

    fn killed() {
        todo!()
    }

    fn last_accessed_last_accessed(last_accessed: String) {
        todo!()
    }

    fn last_disabler_last_disabler(last_disabler: Option<Building>) {
        todo!()
    }

    fn last_heal_time_last_heal_time(last_heal_time: f32) {
        todo!()
    }

    fn liquids_liquids(liquids: Option<LiquidModule>) {
        todo!()
    }

    fn on_removed() {
        todo!()
    }

    fn payload_draw() {
        todo!()
    }

    fn payload_rotation_payload_rotation(payload_rotation: f32) {
        todo!()
    }

    fn power_power(power: Option<PowerModule>) {
        todo!()
    }

    fn power_graph_removed() {
        todo!()
    }

    fn produced(item: Item) {
        todo!()
    }

    fn produced_item(item: Item, amount: i32) {
        todo!()
    }

    fn proximity_proximity(proximity: HashSet<Building>) {
        todo!()
    }

    fn read(read: Reads, revision: u8) {
        todo!()
    }

    fn read_all(read: Reads, revision: u8) {
        todo!()
    }

    fn read_base(read: Reads) {
        todo!()
    }

    fn recently_healed() {
        todo!()
    }

    fn remove() {
        todo!()
    }

    fn remove_from_proximity() {
        todo!()
    }

    fn rotation_rotation(rotation: i32) {
        todo!()
    }

    fn tile_tile(tile: Tile) {
        todo!()
    }

    fn transfer_liquid(next: Building, amount: f32, liquid: Liquid) {
        todo!()
    }

    fn update() {
        todo!()
    }

    fn update_consumption() {
        todo!()
    }

    fn update_payload(unit_holder: Unit, building_holder: Building) {
        todo!()
    }

    fn update_power_graph() {
        todo!()
    }

    fn update_proximity() {
        todo!()
    }

    fn update_tile() {
        todo!()
    }

    fn visual_liquid_visual_liquid(visual_liquid: f32) {
        todo!()
    }

    fn was_damaged_was_damaged(was_damaged: bool) {
        todo!()
    }

    fn was_visible_was_visible(was_visible: bool) {
        todo!()
    }

    fn write(write: Writes) {
        todo!()
    }

    fn write_all(write: Writes) {
        todo!()
    }

    fn write_base(write: Writes) {
        todo!()
    }
}