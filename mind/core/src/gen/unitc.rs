use arc::arc_core::math::geom::position::Position;
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
use crate::gen::velc::Velc;
use crate::gen::weaponsc::Weaponsc;
use crate::gen::ranged::Ranged;
use crate::gen::senseable::Senseable;
use crate::gen::displayable::Displayable;
use arc::arc_core::math::geom::vec2::Vec2;
use crate::ctype::content::Content;
use crate::game::team::Team;
use crate::gen::player::Player;
use crate::logic::sensible::LAccess;

/// Interface for {@link mindustry.entities.comp.UnitComp}
pub trait Unitc : Boundedc + Builderc + Drawc + Entityc + Flyingc + Healthc + Hitboxc + Itemsc + Minerc + Physicsc + Posc + Rotc + Shieldc + Statusc + Syncc + Teamc + Velc + Weaponsc + Ranged + Senseable + Displayable {
    /// # Returns
    /// approx. square size of the physical hitbox for physics
    fn physic_size() -> f32;

    /// # Returns
    /// name of direct or indirect player controller.
    fn get_controller_name() -> String;

    /// # Returns
    /// pathfinder path type for calculating costs
    fn path_type() -> i32;

    /// # Returns
    /// speed with boost & floor multipliers factored in.
    fn speed() -> f32;

    /// # Returns
    /// where the unit wants to look at.
    fn pref_rotation() -> f32;

    /// # Returns
    /// whether there is solid, un-occupied ground under this unit.
    fn can_land() -> bool;

    /// Actually destroys the unit, removing it and creating explosions.
    fn destroy();

    /// Called when this unit was unloaded from a factory or spawn point.
    fn unloaded();

    /// Move based on preferred unit movement type.
    fn move_pref(movement: Vec2);

    fn trail() -> Option<Trail>;

    fn docked_type() -> Option<UnitType>;

    fn get_player() -> Player;

    fn can_drown() -> bool;

    fn can_shoot() -> bool;

    fn collides(other: Hitboxc) -> bool;

    fn displayable() -> bool;

    fn has_weapons() -> bool;

    fn hittable() -> bool;

    fn in_fog_to(viewer: Team) -> bool;

    fn in_range(other: Position) -> bool;

    fn is_ai() -> bool;

    fn is_commandable() -> bool;

    fn is_enemy() -> bool;

    fn is_immune(effect: StatusEffect) -> bool;

    fn is_path_impassable(tile_x: i32, tile_y: i32) -> bool;

    fn is_player() -> bool;

    fn is_sync_hidden(player: Player) -> bool;

    fn spawned_by_core() -> bool;

    fn targetable(targeter: Team) -> bool;

    fn flag() -> f64;

    fn sense(content: Content) -> f64;

    fn sense_sensor(sensor: LAccess) -> f64;

    fn bounds() -> f32;

    fn clip_size() -> f32;

    fn heal_time() -> f32;

    fn range() -> f32;

    fn shadow_alpha() -> f32;

    fn cap() -> i32;

    fn count() -> i32;

    fn item_capacity() -> i32;

    fn last_fog_pos() -> i32;

    fn sense_object(sensor: LAccess) -> Object;

    fn last_commanded() -> String;

    fn to_string() -> String;

    fn command() -> CommandAI;

    fn abilities() -> Ability[];

    fn controller() -> UnitController;

    fn type() -> UnitType;

    fn abilities_abilities(abilities: Ability[]);

    fn add();

    fn after_read();

    fn after_sync();

    fn aim_look(pos: Position);

    fn aim_look_x(x: f32, y: f32);

    fn approach(vector: Vec2);

    fn collision(other: Hitboxc, x: f32, y: f32);

    fn controller_next(next: UnitController);

    fn display(table: Table);

    fn docked_type_docked_type(docked_type: Option<UnitType>);

    fn draw();

    fn flag_flag(flag: f64);

    fn handle_sync_hidden();

    fn heal(amount: f32);

    fn heal_time_heal_time(heal_time: f32);

    fn kill();

    fn killed();

    fn landed();

    fn last_commanded_last_commanded(last_commanded: String);

    fn last_fog_pos_last_fog_pos(last_fog_pos: i32);

    fn look_at(pos: Position);

    fn look_at_angle(angle: f32);

    fn look_at_x(x: f32, y: f32);

    fn move_at(vector: Vec2);

    fn remove();

    fn reset_controller();

    fn rotate_move(vec: Vec2);

    fn set(def: UnitType, controller: UnitController);

    fn set_type(ty: UnitType);

    fn shadow_alpha_shadow_alpha(shadow_alpha: f32);

    fn spawned_by_core_spawned_by_core(spawned_by_core: bool);

    fn trail_trail(trail: Option<Trail>);

    fn type_type(ty: UnitType);

    fn update();

    fn update_boosting(boost: bool);
}
