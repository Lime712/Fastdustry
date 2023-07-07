use crate::gen::boundedc::Boundedc;
use crate::gen::builderc::Builderc;
use crate::gen::drawc::Drawc;
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
/// Interface for {@link mindustry.entities.comp.LegsComp}
pub trait Legsc : Boundedc + Builderc + Drawc + Entityc + Flyingc + Healthc + Hitboxc + Itemsc + Minerc + Physicsc + Posc + Rotc + Shieldc + Statusc + Syncc + Teamc + Unitc + Velc + Weaponsc {
    /// # Returns
    /// outwards facing angle of leg at the specified index.
    fn leg_angle(index: i32) -> f32;

    fn cur_move_offset() -> Vec2;

    fn leg_offset(out: Vec2, index: i32) -> Vec2;

    fn base_rotation() -> f32;

    fn default_leg_angle(index: i32) -> f32;

    fn move_space() -> f32;

    fn total_length() -> f32;

    fn path_type() -> i32;

    fn solidity() -> SolidPred;

    fn legs() -> Vec<Leg>;

    fn drown_floor() -> Floor;

    fn last_deep_floor() -> Floor;

    fn add();

    fn base_rotation_base_rotation(base_rotation: f32);

    fn cur_move_offset_cur_move_offset(cur_move_offset: Vec2);

    fn destroy();

    fn last_deep_floor_last_deep_floor(last_deep_floor: Floor);

    fn legs_legs(legs: Vec<Leg>);

    fn move_space_move_space(move_space: f32);

    fn reset_legs();

    fn reset_legs_leg_length(leg_length: f32);

    fn total_length_total_length(total_length: f32);

    fn unloaded();

    fn update();
}
