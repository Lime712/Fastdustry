use crate::gen::building::Building;
use crate::world::block::Block;
use crate::world::consumer::Consume;
use crate::world::meta::stat;
use crate::world::meta::stat::{POWER_CAPACITY, POWER_USE, Stat};
use crate::world::meta::stat_unit::{NONE, POWER_SECOND, StatUnit};
use crate::world::meta::stats::Stats;

/// Consumer class for blocks which consume power while being connected to a power graph.
pub struct ConsumePower {
    /// The maximum amount of power which can be processed per tick. This might influence efficiency or load a buffer.
    pub usage: f32,
    /// The maximum power capacity in power units.
    pub capacity: f32,
    /// True if the module can store power.
    pub buffered: bool,

    // for the consume trait
    optional: bool,
    booster: bool,
    update: bool,
}

impl Default for ConsumePower {
    fn default() -> Self {
        Self {
            usage: 0.0,
            capacity: 0.0,
            buffered: false,
        }
    }
}

impl ConsumePower {
    pub fn new(usage: f32, capacity: f32, buffered: bool) -> Self {
        Self {
            usage,
            capacity,
            buffered,
        }
    }
}

impl Consume for ConsumePower {
    fn is_optional(&self) -> bool {
        self.optional
    }

    fn set_optional(&mut self, optional: bool) {
        self.optional = optional;
    }

    fn is_boost(&self) -> bool {
        self.booster
    }

    fn set_boost(&mut self, boost: bool) {
        self.booster = boost;
    }

    fn is_update(&self) -> bool {
        self.update
    }

    fn set_update(&mut self, update: bool) {
        self.update = update;
    }

    fn apply(&mut self, block: &mut Block) {
        todo!()
    }

    fn optional(&self, optional: bool, boost: bool, update: bool) -> Self {
        todo!()
    }

    fn ignore(&self) -> bool {
        self.buffered
    }

    fn efficiency(&self, build: Building) -> f32 {
        build.power.unwrap().status
    }

    fn display(&self, mut stats: Stats) {
        if self.buffered {
            stats.add_stat_unit(POWER_CAPACITY.clone(), self.capacity, NONE.clone());
        } else {
            stats.add_stat_unit(POWER_USE.clone(), self.usage * 60.0, POWER_SECOND.clone());
        }
    }
}

impl ConsumePower {
    /// Retrieves the amount of power which is requested for the given block and entity.
    /// # Arguments
    /// * `entity` - The entity which contains the power module.
    /// * returns The amount of power which is requested per tick.
    fn request_power(&self, entity: Building) -> f32 {
        return if self.buffered {
            1.0 - entity.power.unwrap().status * self.capacity
        } else {
            self.usage * (if entity.should_consume() { 1.0 } else { 0.0 })
        };
    }
}