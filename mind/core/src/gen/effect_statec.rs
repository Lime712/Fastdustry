use arc::arc_core::graphics::color::Color;
use crate::gen::childc::Childc;
use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::gen::rotc::Rotc;
use crate::gen::timedc::Timedc;
/// Interface for {@link mindustry.entities.comp.EffectStateComp}
pub trait EffectStatec : Childc + Drawc + Entityc + Posc + Rotc + Timedc {
    fn color() -> Color;

    fn clip_size() -> f32;

    fn data() -> Object;

    fn effect() -> Effect;

    fn color_color(color: Color);

    fn data_data(data: Object);

    fn draw();

    fn effect_effect(effect: Effect);
}
