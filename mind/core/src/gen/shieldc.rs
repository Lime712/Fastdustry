use crate::gen::entityc::Entityc;
use crate::gen::healthc::Healthc;
use crate::gen::posc::Posc;
/// Interface for {@link mindustry.entities.comp.ShieldComp}
pub trait Shieldc : Entityc + Healthc + Posc {
    /// Absorbs health damage.
    fn shield() -> f32;

    /// Absorbs health damage.
    fn shield_shield(shield: f32);

    /// Shield opacity.
    fn shield_alpha() -> f32;

    /// Shield opacity.
    fn shield_alpha_shield_alpha(shield_alpha: f32);

    /// Subtracts an amount from damage. No need to save.
    fn armor() -> f32;

    /// Subtracts an amount from damage. No need to save.
    fn armor_armor(armor: f32);

    fn damage(amount: f32);

    fn damage_pierce(amount: f32, with_effect: bool);

    fn update();
}
