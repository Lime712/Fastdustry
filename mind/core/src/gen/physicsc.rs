use crate::gen::entityc::Entityc;
use crate::gen::flyingc::Flyingc;
use crate::gen::healthc::Healthc;
use crate::gen::hitboxc::Hitboxc;
use crate::gen::posc::Posc;
use crate::gen::velc::Velc;
use arc::arc_core::math::geom::vec2::Vec2;
/// Interface for {@link mindustry.entities.comp.PhysicsComp}
pub trait Physicsc : Entityc + Flyingc + Healthc + Hitboxc + Posc + Velc {
    fn mass() -> f32;

    fn physref() -> PhysicsProcess.PhysicRef;

    fn impulse(v: Vec2);

    fn impulse_x(x: f32, y: f32);

    fn impulse_net(v: Vec2);

    fn physref_physref(physref: PhysicsProcess.PhysicRef);
}
