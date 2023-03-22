use crate::gen::building::Building;
use crate::world::block::Block;
use crate::world::meta::stats::Stats;

pub mod consume_power;

/// An abstract class that defines a type of resource that a block can consume.
pub trait Consume {
    /// If true, this consumer will not influence consumer validity.
    fn is_optional(&self) -> bool;
    fn set_optional(&mut self, optional: bool);
    /// If true, this consumer will be displayed as a boost input.
    fn is_boost(&self) -> bool;
    fn set_boost(&mut self, boost: bool);
    /// If false, this consumer will still be checked, but it will need to updated manually.
    fn is_update(&self) -> bool;
    fn set_update(&mut self, update: bool);
    /// Multiplier for costs. Does not work for power consumers.
    fn get_multiplier(&self) -> f32 {
        return 1.0;
    }
    fn apply(&mut self, block: &mut Block);
    fn optional(&self, optional: bool, boost: bool, update: bool) -> Self;
    fn boost(&self) {
        self.optional(true, true)
    }
    fn update(&mut self, update: bool) -> &mut Self {
        self.set_update(update);
        self
    }
    /// * returns if true, this consumer will be ignored in the consumer list (no updates or valid() checks)
    fn ignore(&self) -> bool { return false; }
    /// Called when a consumption is triggered manually.
    fn trigger(&mut self, build: Building) {}
    fn update_build(&mut self, build: Building) {}
    /// * returns [0, 1] efficiency multiplier based on input. Returns 0 if not valid in subclasses. Should return fraction if needs are partially met.
    fn efficiency(&self, build: Building) -> f32 { return 1.0; }
    /// * returns multiplier for efficiency - this can be above 1. Will not influence a building's base efficiency value.
    fn efficiency_multiplier(&self, build: Building) -> f32 {
        return 1.0;
    }
    fn display(&self, stats: Stats) {}
}