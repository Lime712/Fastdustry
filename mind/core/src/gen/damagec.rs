use crate::gen::entityc::Entityc;
/// Interface for {@link mindustry.entities.comp.DamageComp}
pub trait Damagec : Entityc {
    fn damage() -> f32;

    fn damage_damage(damage: f32);
}
