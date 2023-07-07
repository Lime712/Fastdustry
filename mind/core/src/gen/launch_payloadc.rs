use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::gen::teamc::Teamc;
use crate::gen::timedc::Timedc;
use std::collections::HashSet;
use arc::arc_core::util::interval::Interval;
use crate::r#type::item_stack::ItemStack;

/// Interface for {@link mindustry.world.blocks.campaign.LaunchPad.LaunchPayloadComp}
pub trait LaunchPayloadc : Drawc + Entityc + Posc + Teamc + Timedc {
    fn stacks() -> HashSet<ItemStack>;

    fn input() -> Interval;

    fn cx() -> f32;

    fn cy() -> f32;

    fn draw();

    fn in_in(input: Interval);

    fn remove();

    fn stacks_stacks(stacks: HashSet<ItemStack>);

    fn update();
}
