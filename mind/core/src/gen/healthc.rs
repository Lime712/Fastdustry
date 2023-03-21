use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
/// Interface for {@link mindustry.entities.comp.HealthComp}
pub trait Healthc : Entityc + Posc {
    /// Damage and pierce armor.
    fn damage_pierce(amount: f32);

    /// Damage and pierce armor.
    fn damage_pierce_amount(amount: f32, with_effect: bool);

    /// Heals by a 0-1 fraction of max health.
    fn heal_fract(amount: f32);

    /// Heals by a flat amount.
    fn heal_amount(amount: f32);

    fn damaged() -> bool;

    fn dead() -> bool;

    fn is_valid() -> bool;

    fn health() -> f32;

    fn healthf() -> f32;

    fn hit_time() -> f32;

    fn max_health() -> f32;

    fn clamp_health();

    fn damage(amount: f32);

    fn damage_amount(amount: f32, with_effect: bool);

    fn damage_continuous(amount: f32);

    fn damage_continuous_pierce(amount: f32);

    fn dead_dead(dead: bool);

    fn heal();

    fn health_health(health: f32);

    fn hit_time_hit_time(hit_time: f32);

    fn kill();

    fn killed();

    fn max_health_max_health(max_health: f32);

    fn update();
}
