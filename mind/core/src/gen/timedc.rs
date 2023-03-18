use crate::gen::scaled::Scaled;
use crate::gen::entityc::Entityc;
/// Interface for {@link mindustry.entities.comp.TimedComp}
pub trait Timedc : Scaled + Entityc {
    fn fin() -> f32;

    fn lifetime() -> f32;

    fn time() -> f32;

    fn lifetime_lifetime(lifetime: f32);

    fn time_time(time: f32);

    fn update();
}
