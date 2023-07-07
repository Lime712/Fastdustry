use arc::arc_core::graphics::color::Color;
use crate::gen::entityc::Entityc;
use crate::gen::flyingc::Flyingc;
use crate::gen::healthc::Healthc;
use crate::gen::hitboxc::Hitboxc;
use crate::gen::posc::Posc;
use crate::gen::velc::Velc;
/// Interface for {@link mindustry.entities.comp.StatusComp}
pub trait Statusc : Entityc + Flyingc + Healthc + Hitboxc + Posc + Velc {
    /// Adds a status effect to this unit.
    fn apply(effect: StatusEffect, duration: f32);

    /// Apply a status effect for 1 tick (for permanent effects)
    fn apply_effect(effect: StatusEffect);

    /// Removes a status effect.
    fn unapply(effect: StatusEffect);

    fn status_color() -> Color;

    fn status_bits() -> Bits;

    fn disarmed() -> bool;

    fn has_effect(effect: StatusEffect) -> bool;

    fn is_boss() -> bool;

    fn is_immune(effect: StatusEffect) -> bool;

    fn build_speed_multiplier() -> f32;

    fn damage_multiplier() -> f32;

    fn drag_multiplier() -> f32;

    fn get_duration(effect: StatusEffect) -> f32;

    fn health_multiplier() -> f32;

    fn reload_multiplier() -> f32;

    fn speed_multiplier() -> f32;

    fn build_speed_multiplier_build_speed_multiplier(build_speed_multiplier: f32);

    fn clear_statuses();

    fn damage_multiplier_damage_multiplier(damage_multiplier: f32);

    fn disarmed_disarmed(disarmed: bool);

    fn drag_multiplier_drag_multiplier(drag_multiplier: f32);

    fn draw();

    fn health_multiplier_health_multiplier(health_multiplier: f32);

    fn reload_multiplier_reload_multiplier(reload_multiplier: f32);

    fn speed_multiplier_speed_multiplier(speed_multiplier: f32);

    fn update();
}
