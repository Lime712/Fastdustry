use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use arc::arc_core::math::geom::vec2::Vec2;
/// Interface for {@link mindustry.entities.comp.VelComp}
pub trait Velc : Entityc + Posc {
    /// # Returns
    /// function to use for check solid state. if null, no checking is done.
    fn solidity() -> EntityCollisions.SolidPred;

    /// # Returns
    /// whether this entity can exist on its current location
    fn can_pass_on() -> bool;

    /// # Returns
    /// whether this entity can move through a location
    fn can_pass(tile_x: i32, tile_y: i32) -> bool;

    fn vel() -> Vec2;

    fn moving() -> bool;

    fn drag() -> f32;

    fn drag_drag(drag: f32);

    fn move(v: Vec2);

    fn move_cx(cx: f32, cy: f32);

    fn update();

    fn vel_vel(vel: Vec2);

    fn vel_add_net(v: Vec2);

    fn vel_add_net_vx(vx: f32, vy: f32);
}
