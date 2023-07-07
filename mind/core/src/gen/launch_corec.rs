use arc::arc_core::util::interval::Interval;
use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::gen::timedc::Timedc;
use crate::world::block::Block;

/// Interface for {@link mindustry.entities.comp.LaunchCoreComp}
pub trait LaunchCorec : Drawc + Entityc + Posc + Timedc {
    fn input() -> Interval;

    fn cx() -> f32;

    fn cy() -> f32;

    fn block() -> Block;

    fn block_block(block: Block);

    fn draw();

    fn in_in(input: Interval);

    fn update();
}
