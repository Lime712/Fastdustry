use arc::arc_core::util::interval::Interval;
use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;

pub const DEATH_DELAY: f32 = 60.0;

#[readonly::make]
pub struct Player {
    pub added: bool,
    pub admin: bool,
    // sync local
    pub boosting: bool,
    // color: Color,
    pub con: Option<NetConnection>,
    pub death_timer: f32,
    pub id: i32,
    // entity_group::next_id(),
    // some more stuff
    index__all: i32,
    index__draw: i32,
    index__player: i32,
    index__sync: i32,
    pub just_switch_from: Option<Unit>,
    pub just_switch_to: Option<Unit>,
    pub last_read_unit: Unit,
    pub last_text: String,
    pub last_updated: i64,
    // sync local
    pub mouse_x: f32,
    // sync local
    pub mouse_y: f32,
    pub name: String,
    // sync local
    pub shooting: bool,
    #[readonly]
    pub team: Team,
    pub text_fade_time: f32,
    pub timer: Interval,
    // sync local
    pub typing: bool,
    #[readonly]
    pub unit: Option<Unit>,
    pub update_spacing: i64,
    pub wrong_read_units: i64,
    // some sync stuff i dont get yet
    pub x: f32,
    x_last_: f32,
    x_target_: f32,
    pub y: f32,
    y_last_: f32,
    y_target_: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            added: false,
            admin: false,
            boosting: false,
            con: None,
            death_timer: 0.0,
            id: 0,
            index__all: 0,
            index__draw: 0,
            index__player: 0,
            index__sync: 0,
            just_switch_from: None,
            just_switch_to: None,
            last_read_unit: Unit::default(),
            last_text: String::new(),
            last_updated: 0,
            mouse_x: 0.0,
            mouse_y: 0.0,
            name: String::new(),
            shooting: false,
            team: Team::default(),
            text_fade_time: 0.0,
            timer: Interval::new(),
            typing: false,
            unit: Unit::default(),
            update_spacing: 0,
            wrong_read_units: 0,
            x: 0.0,
            x_last_: 0.0,
            x_target_: 0.0,
            y: 0.0,
            y_last_: 0.0,
            y_target_: 0.0,
        }
    }
}

impl Entityc for Player {
    fn is_added(&self) -> bool {
        self.added
    }

    fn is_local(&self) -> bool {
        todo!()
    }

    fn is_null(&self) -> bool {
        false
    }

    fn is_remote(&self) -> bool {
        todo!()
    }

    fn serialize(&self) -> bool {
        todo!()
    }

    fn class_id(&self) -> i32 {
        todo!()
    }

    fn id(&self) -> i32 {
        todo!()
    }

    fn add(&mut self) {
        todo!()
    }

    fn after_read(&self) {
        todo!()
    }

    fn id_id(id: i32) {
        todo!()
    }

    fn remove(&mut self) {
        todo!()
    }

    fn update(&self) {
        todo!()
    }
}

impl Posc for Player {
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

impl Drawc for Player {
    fn clip_size(&self) -> f32 {
        if self.unit.is_none() {
            20.0
        } else {
            self.unit.unwrap().ty.hit_size * 2.0
        }
    }

    fn draw(&self) {
        unimplemented!()
    }
}

