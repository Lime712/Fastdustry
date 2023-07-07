use arc::arc_core::func::Cons;
use crate::entities::bullet::bullet_type::BulletType;
use crate::gen::building::Building;
use crate::gen::damagec::Damagec;
use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::hitboxc::Hitboxc;
use crate::gen::ownerc::Ownerc;
use crate::gen::posc::Posc;
use crate::gen::shielderc::Shielderc;
use crate::gen::teamc::Teamc;
use crate::gen::timedc::Timedc;
use crate::gen::timerc::Timerc;
use crate::gen::velc::Velc;
/// Interface for {@link mindustry.entities.comp.BulletComp}
pub trait Bulletc : Damagec + Drawc + Entityc + Hitboxc + Ownerc + Posc + Shielderc + Teamc + Timedc + Timerc + Velc {
    /// # Returns
    /// the bullet's rotation.
    fn rotation() -> f32;

    /// Sets the bullet's rotation in degrees.
    fn rotation_angle(angle: f32);

    fn mover() -> Option<Mover>;

    fn trail() -> Option<Trail>;

    fn aim_tile() -> Option<Tile>;

    fn collided() -> IntSeq;

    fn absorbed() -> bool;

    fn check_under_build(build: Building, x: f32, y: f32) -> bool;

    fn collides(other: dyn Hitboxc) -> bool;

    fn has_collided(id: i32) -> bool;

    fn hit() -> bool;

    fn is_local() -> bool;

    fn keep_alive() -> bool;

    fn aim_x() -> f32;

    fn aim_y() -> f32;

    fn clip_size() -> f32;

    fn damage_multiplier() -> f32;

    fn fdata() -> f32;

    fn origin_x() -> f32;

    fn origin_y() -> f32;

    fn data() -> Object;

    fn ty() -> BulletType;

    fn absorb();

    fn absorbed_absorbed(absorbed: bool);

    fn add();

    fn aim_tile_aim_tile(aim_tile: Option<Tile>);

    fn aim_x_aim_x(aim_x: f32);

    fn aim_y_aim_y(aim_y: f32);

    fn collided_collided(collided: IntSeq);

    fn collision(other: Hitboxc, x: f32, y: f32);

    fn data_data(data: Object);

    fn draw();

    fn fdata_fdata(fdata: f32);

    fn get_collisions(consumer: Cons<QuadTree>);

    fn hit_hit(hit: bool);

    fn init_vel(angle: f32, amount: f32);

    fn keep_alive_keep_alive(keep_alive: bool);

    fn move_relative(x: f32, y: f32);

    fn mover_mover(mover: Option<Mover>);

    fn origin_x_origin_x(origin_x: f32);

    fn origin_y_origin_y(origin_y: f32);

    fn remove();

    fn tile_raycast(x_1: i32, y_1: i32, x_2: i32, y_2: i32);

    fn trail_trail(trail: Option<Trail>);

    fn turn(x: f32, y: f32);

    fn type_type(ty: BulletType);

    fn update();
}
