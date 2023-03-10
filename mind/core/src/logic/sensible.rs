use lazy_static::lazy_static;

use crate::ctype::content::Content;

static mut COUNTER: u32 = 0;

pub trait Sensible {
    fn sense(&self, sensor: LAccess) -> f64;
    fn sense_content(&self, content: Content) -> f64 {
        0.0
    }
    fn sense_object<T: Default>(&self, sensor: LAccess) -> T {
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
            LAccess { params, is_obj, id: COUNTER }
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
pub static ref totalItems: LAccess = LAccess::new_without_params();
pub static ref firstItem: LAccess = LAccess::new_without_params();
pub static ref totalLiquids: LAccess = LAccess::new_without_params();
pub static ref totalPower: LAccess = LAccess::new_without_params();
pub static ref itemCapacity: LAccess = LAccess::new_without_params();
pub static ref liquidCapacity: LAccess = LAccess::new_without_params();
pub static ref powerCapacity: LAccess = LAccess::new_without_params();
pub static ref powerNetStored: LAccess = LAccess::new_without_params();
pub static ref powerNetCapacity: LAccess = LAccess::new_without_params();
pub static ref powerNetIn: LAccess = LAccess::new_without_params();
pub static ref powerNetOut: LAccess = LAccess::new_without_params();
pub static ref ammo: LAccess = LAccess::new_without_params();
pub static ref ammoCapacity: LAccess = LAccess::new_without_params();
pub static ref health: LAccess = LAccess::new_without_params();
pub static ref maxHealth: LAccess = LAccess::new_without_params();
pub static ref heat: LAccess = LAccess::new_without_params();
pub static ref efficiency: LAccess = LAccess::new_without_params();
pub static ref progress: LAccess = LAccess::new_without_params();
pub static ref timescale: LAccess = LAccess::new_without_params();
pub static ref rotation: LAccess = LAccess::new_without_params();
pub static ref x: LAccess = LAccess::new_without_params();
pub static ref y: LAccess = LAccess::new_without_params();
pub static ref shootX: LAccess = LAccess::new_without_params();
pub static ref shootY: LAccess = LAccess::new_without_params();
pub static ref size: LAccess = LAccess::new_without_params();
pub static ref dead: LAccess = LAccess::new_without_params();
pub static ref range: LAccess = LAccess::new_without_params();
pub static ref shooting: LAccess = LAccess::new_without_params();
pub static ref boosting: LAccess = LAccess::new_without_params();
pub static ref mineX: LAccess = LAccess::new_without_params();
pub static ref mineY: LAccess = LAccess::new_without_params();
pub static ref mining: LAccess = LAccess::new_without_params();
pub static ref speed: LAccess = LAccess::new_without_params();
pub static ref team: LAccess = LAccess::new_without_params();
pub static ref ty: LAccess = LAccess::new_without_params();
pub static ref flag: LAccess = LAccess::new_without_params();
pub static ref controlled: LAccess = LAccess::new_without_params();
pub static ref controller: LAccess = LAccess::new_without_params();
pub static ref name: LAccess = LAccess::new_without_params();
pub static ref payloadCount: LAccess = LAccess::new_without_params();
pub static ref payloadType: LAccess = LAccess::new_without_params();

/// values with parameters are considered controllable
pub static ref enabled: LAccess = LAccess::new_params(vec!["to"]);
/// "to" is standard for single parameter access
pub static ref shoot: LAccess = LAccess::new_params(vec!["x", "y", "shoot"]);
pub static ref shootp: LAccess = LAccess::new(true, vec!["unit", "shoot"]);
pub static ref config: LAccess = LAccess::new(true, vec!["to"]);
pub static ref color: LAccess = LAccess::new_params(vec!["to"]);
    }