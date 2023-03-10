use lazy_static::lazy_static;

use crate::ctype::content::Content;

static mut COUNTER: u32 = 0;

pub trait Sensible {
    fn sense(&self, sensor: LAccess) -> f64;
    fn sense_content(&self, _content: Content) -> f64 {
        0.0
    }
    fn sense_object<T: Default>(&self, _sensor: LAccess) -> T {
        T::default()
    }
}

pub struct LAccess {
    params: Vec<&'static str>,
    is_obj: bool,
    id: u32,
}

impl LAccess {
    pub fn new(is_obj: bool, params: Vec<&'static str>) -> LAccess {
        unsafe {
            COUNTER += 1;
            LAccess {
                params,
                is_obj,
                id: COUNTER,
            }
        }
    }

    pub fn new_params(params: Vec<&'static str>) -> LAccess {
        LAccess::new(false, params)
    }

    pub fn new_without_params() -> LAccess {
        LAccess::new_params(Vec::new())
    }
}

impl Eq for LAccess {}

impl PartialEq for LAccess {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params && self.is_obj == other.is_obj && self.id == other.id
    }
}

lazy_static! {
pub static ref TOTAL_ITEMS: LAccess = LAccess::new_without_params();
pub static ref FIRST_ITEM: LAccess = LAccess::new_without_params();
pub static ref TOTAL_LIQUIDS: LAccess = LAccess::new_without_params();
pub static ref TOTAL_POWER: LAccess = LAccess::new_without_params();
pub static ref ITEM_CAPACITY: LAccess = LAccess::new_without_params();
pub static ref LIQUID_CAPACITY: LAccess = LAccess::new_without_params();
pub static ref POWER_CAPACITY: LAccess = LAccess::new_without_params();
pub static ref POWER_NET_STORED: LAccess = LAccess::new_without_params();
pub static ref POWER_NET_CAPACITY: LAccess = LAccess::new_without_params();
pub static ref POWER_NET_IN: LAccess = LAccess::new_without_params();
pub static ref POWER_NET_OUT: LAccess = LAccess::new_without_params();
pub static ref AMMO: LAccess = LAccess::new_without_params();
pub static ref AMMO_CAPACITY: LAccess = LAccess::new_without_params();
pub static ref HEALTH: LAccess = LAccess::new_without_params();
pub static ref MAX_HEALTH: LAccess = LAccess::new_without_params();
pub static ref HEAT: LAccess = LAccess::new_without_params();
pub static ref EFFICIENCY: LAccess = LAccess::new_without_params();
pub static ref PROGRESS: LAccess = LAccess::new_without_params();
pub static ref TIMESCALE: LAccess = LAccess::new_without_params();
pub static ref ROTATION: LAccess = LAccess::new_without_params();
pub static ref X: LAccess = LAccess::new_without_params();
pub static ref Y: LAccess = LAccess::new_without_params();
pub static ref SHOOT_X: LAccess = LAccess::new_without_params();
pub static ref SHOOT_Y: LAccess = LAccess::new_without_params();
pub static ref SIZE: LAccess = LAccess::new_without_params();
pub static ref DEAD: LAccess = LAccess::new_without_params();
pub static ref RANGE: LAccess = LAccess::new_without_params();
pub static ref SHOOTING: LAccess = LAccess::new_without_params();
pub static ref BOOSTING: LAccess = LAccess::new_without_params();
pub static ref MINE_X: LAccess = LAccess::new_without_params();
pub static ref MINE_Y: LAccess = LAccess::new_without_params();
pub static ref MINING: LAccess = LAccess::new_without_params();
pub static ref SPEED: LAccess = LAccess::new_without_params();
pub static ref TEAM: LAccess = LAccess::new_without_params();
pub static ref TYPE: LAccess = LAccess::new_without_params();
pub static ref FLAG: LAccess = LAccess::new_without_params();
pub static ref CONTROLLED: LAccess = LAccess::new_without_params();
pub static ref CONTROLLER: LAccess = LAccess::new_without_params();
pub static ref NAME: LAccess = LAccess::new_without_params();
pub static ref PAYLOAD_COUNT: LAccess = LAccess::new_without_params();
pub static ref PAYLOAD_TYPE: LAccess = LAccess::new_without_params();

/// values with parameters are considered controllable
pub static ref ENABLED: LAccess = LAccess::new_params(vec!["to"]);
/// "to" is standard for single parameter access
pub static ref SHOOT: LAccess = LAccess::new_params(vec!["x", "y", "shoot"]);
pub static ref SHOOTP: LAccess = LAccess::new(true, vec!["unit", "shoot"]);
pub static ref CONFIG: LAccess = LAccess::new(true, vec!["to"]);
pub static ref COLOR: LAccess = LAccess::new_params(vec!["to"]);
    }
