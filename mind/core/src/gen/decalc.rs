use arc::arc_core::graphics::color::Color;
use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::gen::rotc::Rotc;
use crate::gen::timedc::Timedc;
/// Interface for {@link mindustry.entities.comp.DecalComp}
pub trait Decalc : Drawc + Entityc + Posc + Rotc + Timedc {
    fn color() -> Color;

    fn region() -> TextureRegion;

    fn clip_size() -> f32;

    fn color_color(color: Color);

    fn draw();

    fn region_region(region: TextureRegion);
}
