use crate::game::team::Team;
use crate::gen::building::Building;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CoreBuild {
    pub building: Building,
    pub storage_capacity: i32,
    pub no_effect: bool,
    pub last_damage: &'static Team,
    pub iframes: f32,
    pub thruster_time: f32,
}


impl CoreBuild {
    // todo: implement all the methods
}