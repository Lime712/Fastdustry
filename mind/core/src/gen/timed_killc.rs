use crate::gen::scaled::Scaled;
use crate::gen::entityc::Entityc;
use crate::gen::healthc::Healthc;
use crate::gen::posc::Posc;
/// Interface for {@link mindustry.entities.comp.TimedKillComp}
pub trait TimedKillc : Scaled + Entityc + Healthc + Posc {
    fn fin() -> f32;

    fn lifetime() -> f32;

    fn time() -> f32;

    fn lifetime_lifetime(lifetime: f32);

    fn time_time(time: f32);

    fn update();
}
