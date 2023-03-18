use crate::gen::boundedc::Boundedc;
use crate::gen::builderc::Builderc;
use crate::gen::drawc::Drawc;
use crate::gen::elevation_movec::ElevationMovec;
use crate::gen::entityc::Entityc;
use crate::gen::flyingc::Flyingc;
use crate::gen::healthc::Healthc;
use crate::gen::hitboxc::Hitboxc;
use crate::gen::itemsc::Itemsc;
use crate::gen::minerc::Minerc;
use crate::gen::physicsc::Physicsc;
use crate::gen::posc::Posc;
use crate::gen::rotc::Rotc;
use crate::gen::shieldc::Shieldc;
use crate::gen::statusc::Statusc;
use crate::gen::syncc::Syncc;
use crate::gen::teamc::Teamc;
use crate::gen::unitc::Unitc;
use crate::gen::velc::Velc;
use crate::gen::weaponsc::Weaponsc;
use arc::arc_core::math::geom::vec2::Vec2;
/// Interface for {@link mindustry.entities.comp.TankComp}
pub trait Tankc : Boundedc + Builderc + Drawc + ElevationMovec + Entityc + Flyingc + Healthc + Hitboxc + Itemsc + Minerc + Physicsc + Posc + Rotc + Shieldc + Statusc + Syncc + Teamc + Unitc + Velc + Weaponsc {
    fn walked() -> bool;

    fn floor_speed_multiplier() -> f32;

    fn tread_time() -> f32;

    fn drown_floor() -> Floor;

    fn approach(vector: Vec2);

    fn move_at(vector: Vec2, acceleration: f32);

    fn tread_time_tread_time(tread_time: f32);

    fn update();

    fn walked_walked(walked: bool);
}
