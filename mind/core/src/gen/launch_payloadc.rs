use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::gen::teamc::Teamc;
use crate::gen::timedc::Timedc;
use std::collections::HashSet;
/// Interface for {@link mindustry.world.blocks.campaign.LaunchPad.LaunchPayloadComp}
pub trait LaunchPayloadc : Drawc + Entityc + Posc + Teamc + Timedc {
    fn stacks() -> HashSet<ItemStack>;

    fn in() -> Interval;

    fn cx() -> f32;

    fn cy() -> f32;

    fn draw();

    fn in_in(in: Interval);

    fn remove();

    fn stacks_stacks(stacks: HashSet<ItemStack>);

    fn update();
}
