use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
/// Interface for {@link mindustry.entities.comp.DrawComp}
pub trait Drawc : Entityc + Posc {
    fn clip_size() -> f32;

    fn draw();
}
