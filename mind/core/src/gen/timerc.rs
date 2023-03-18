use crate::gen::entityc::Entityc;
/// Interface for {@link mindustry.entities.comp.TimerComp}
pub trait Timerc : Entityc {
    fn timer() -> Interval;

    fn timer_index(index: i32, time: f32) -> bool;

    fn timer_timer(timer: Interval);
}
