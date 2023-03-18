use crate::gen::entityc::Entityc;
/// Interface for {@link mindustry.entities.comp.RotComp}
pub trait Rotc : Entityc {
    fn rotation() -> f32;

    fn rotation_rotation(rotation: f32);
}
