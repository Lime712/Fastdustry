use crate::gen::entityc::Entityc;
use crate::gen::flyingc::Flyingc;
use crate::gen::healthc::Healthc;
use crate::gen::hitboxc::Hitboxc;
use crate::gen::posc::Posc;
use crate::gen::rotc::Rotc;
use crate::gen::statusc::Statusc;
use crate::gen::teamc::Teamc;
use crate::gen::velc::Velc;
/// Interface for {@link mindustry.entities.comp.WeaponsComp}
pub trait Weaponsc : Entityc + Flyingc + Healthc + Hitboxc + Posc + Rotc + Statusc + Teamc + Velc {
    /// Aim at something. This will make all mounts point at it.
    fn aim(x: f32, y: f32);

    /// Update shooting and rotation for this unit.
    fn update();

    /// weapon mount array, never null
    fn mounts() -> WeaponMount[];

    /// weapon mount array, never null
    fn mounts_mounts(mounts: WeaponMount[]);

    fn can_shoot() -> bool;

    fn is_rotate() -> bool;

    fn is_shooting() -> bool;

    fn aim_x() -> f32;

    fn aim_y() -> f32;

    fn ammo() -> f32;

    fn ammof() -> f32;

    fn aim_pos(pos: Position);

    fn aim_x_aim_x(aim_x: f32);

    fn aim_y_aim_y(aim_y: f32);

    fn ammo_ammo(ammo: f32);

    fn control_weapons(rotate: bool, shoot: bool);

    fn control_weapons_rotate_shoot(rotate_shoot: bool);

    fn is_shooting_is_shooting(is_shooting: bool);

    fn remove();

    fn set_weapon_rotation(rotation: f32);

    fn setup_weapons(def: UnitType);
}
