use crate::gen::quad_tree_object::QuadTreeObject;
use crate::gen::sized::Sized;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
/// Interface for {@link mindustry.entities.comp.HitboxComp}
pub trait Hitboxc : QuadTreeObject + Sized + Entityc + Posc {
    fn collides(other: Hitboxc) -> bool;

    fn delta_angle() -> f32;

    fn delta_len() -> f32;

    fn delta_x() -> f32;

    fn delta_y() -> f32;

    fn hit_size() -> f32;

    fn last_x() -> f32;

    fn last_y() -> f32;

    fn add();

    fn after_read();

    fn collision(other: Hitboxc, x: f32, y: f32);

    fn delta_x_delta_x(delta_x: f32);

    fn delta_y_delta_y(delta_y: f32);

    fn get_collisions(consumer: Cons<QuadTree>);

    fn hit_size_hit_size(hit_size: f32);

    fn hitbox(rect: Rect);

    fn hitbox_tile(rect: Rect);

    fn last_x_last_x(last_x: f32);

    fn last_y_last_y(last_y: f32);

    fn update();

    fn update_last_position();
}
