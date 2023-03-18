use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::gen::rotc::Rotc;
/// Interface for {@link mindustry.entities.comp.ChildComp}
pub trait Childc : Entityc + Posc + Rotc {
    fn parent() -> Option<Posc>;

    fn rot_with_parent() -> bool;

    fn offset_pos() -> f32;

    fn offset_rot() -> f32;

    fn offset_x() -> f32;

    fn offset_y() -> f32;

    fn add();

    fn offset_pos_offset_pos(offset_pos: f32);

    fn offset_rot_offset_rot(offset_rot: f32);

    fn offset_x_offset_x(offset_x: f32);

    fn offset_y_offset_y(offset_y: f32);

    fn parent_parent(parent: Option<Posc>);

    fn rot_with_parent_rot_with_parent(rot_with_parent: bool);

    fn update();
}
