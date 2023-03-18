use crate::gen::entityc::Entityc;
use crate::gen::healthc::Healthc;
use crate::gen::hitboxc::Hitboxc;
use crate::gen::posc::Posc;
use crate::gen::velc::Velc;
use arc::arc_core::math::geom::vec2::Vec2;
/// Interface for {@link mindustry.entities.comp.FlyingComp}
pub trait Flyingc : Entityc + Healthc + Hitboxc + Posc + Velc {
    fn last_drown_floor() -> Option<Floor>;

    fn can_drown() -> bool;

    fn check_target(target_air: bool, target_ground: bool) -> bool;

    fn emit_walk_sound() -> bool;

    fn hovering() -> bool;

    fn is_flying() -> bool;

    fn is_grounded() -> bool;

    fn drown_time() -> f32;

    fn elevation() -> f32;

    fn floor_speed_multiplier() -> f32;

    fn splash_timer() -> f32;

    fn drown_floor() -> Floor;

    fn drown_time_drown_time(drown_time: f32);

    fn elevation_elevation(elevation: f32);

    fn hovering_hovering(hovering: bool);

    fn landed();

    fn last_drown_floor_last_drown_floor(last_drown_floor: Option<Floor>);

    fn move_at(vector: Vec2, acceleration: f32);

    fn splash_timer_splash_timer(splash_timer: f32);

    fn update();

    fn update_drowning();

    fn wobble();
}
