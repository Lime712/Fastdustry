use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::gen::timedc::Timedc;
/// Interface for {@link mindustry.entities.comp.LaunchCoreComp}
pub trait LaunchCorec : Drawc + Entityc + Posc + Timedc {
    fn in() -> Interval;

    fn cx() -> f32;

    fn cy() -> f32;

    fn block() -> Block;

    fn block_block(block: Block);

    fn draw();

    fn in_in(in: Interval);

    fn update();
}
