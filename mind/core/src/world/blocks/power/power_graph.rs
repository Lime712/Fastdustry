use std::cmp::min;
use std::collections::{HashSet, VecDeque};

use arc::arc_core::math::geom::mathf;
use arc::arc_core::math::geom::mathf::zero;
use arc::arc_core::math::windowed_mean::WindowedMean;

use crate::gen::building::Building;
use crate::gen::power_graph_updater::PowerGraphUpdater;

static mut QUEUE: VecDeque<Building> = VecDeque::new();
static mut OUT_ARRAY1: HashSet<Building> = HashSet::new();
static mut OUT_ARRAY2: HashSet<Building> = HashSet::new();
static mut CLOSED_SET: HashSet<i32> = HashSet::new();
static mut LAST_GRAPH_ID: u32 = 0;

pub struct PowerGraph {
    pub producers: HashSet<Building>,
    pub consumers: HashSet<Building>,
    pub batteries: HashSet<Building>,
    pub all: HashSet<Building>,
    entity: Option<PowerGraphUpdater>,
    power_balance: WindowedMean,
    last_power_produced: f32,
    last_power_needed: f32,
    last_power_stored: f32,
    last_scaled_power_in: f32,
    last_scaled_power_out: f32,
    last_capacity: f32,
    /// diodes workaround for correct energy production info
    energy_delta: f32,
    graph_id: u32,
}

impl PowerGraph {
    fn new() -> PowerGraph {
        let mut s = Self {
            producers: Default::default(),
            consumers: Default::default(),
            batteries: Default::default(),
            all: Default::default(),
            entity: Some(PowerGraphUpdater::create()),
            power_balance: WindowedMean::new(60),
            last_power_produced: 0.0,
            last_power_needed: 0.0,
            last_power_stored: 0.0,
            last_scaled_power_in: 0.0,
            last_scaled_power_out: 0.0,
            last_capacity: 0.0,
            energy_delta: 0.0,
            graph_id: 0,
        };
        s.entity.graph = Some(&s);
        unsafe { LAST_GRAPH_ID += 1; }
        s.graph_id = unsafe { LAST_GRAPH_ID };
        s
    }

    pub fn get_id(&self) -> u32 {
        self.graph_id
    }

    pub fn get_last_scaled_power_in(&self) -> f32 {
        self.last_scaled_power_in
    }

    pub fn get_last_scaled_power_out(&self) -> f32 {
        self.last_scaled_power_out
    }

    pub fn get_last_capacity(&self) -> f32 {
        self.last_capacity
    }

    pub fn get_power_balance(&mut self) -> f32 {
        self.power_balance.raw_mean()
    }

    pub fn has_power_balance_samples(&self) -> bool {
        self.power_balance.has_enough_data()
    }

    pub fn get_last_power_needed(&self) -> f32 {
        self.last_power_needed
    }

    pub fn get_last_power_produced(&self) -> f32 {
        self.last_power_produced
    }

    pub fn get_last_power_stored(&self) -> f32 {
        self.last_power_stored
    }

    pub fn transfer_power(&mut self, amount: f32) {
        if amount > 0.0 {
            self.charge_batteries(amount);
        } else {
            self.use_batteries(-amount);
        }
        self.energy_delta += amount;
    }

    pub fn get_satisfaction(&self) -> f32 {
        if mathf::zero(self.last_power_produced, None) {
            return 0.0;
        } else if mathf::zero(self.last_power_needed, None) {
            return 1.0;
        }
        mathf::clamp(self.last_power_produced / self.last_power_needed, 0.0, 1.0)
    }

    pub fn get_power_produced(&self) -> f32 {
        let mut power_produced = 0.0;
        for producer in &self.producers {
            power_produced += producer.get_power_production() * producer.delta();
        }
        power_produced
    }

    pub fn get_power_needed(&self) -> f32 {
        let mut power_needed = 0.0;
        for consumer in &self.consumers {
            let consume_power = consumer.block.cons_power.unwrap();
            if self.other_consumers_are_valid(consumer, consume_power) {
                power_needed += consume_power.requested_power(consumer) * consumer.delta();
            }
        }
        power_needed
    }

    pub fn get_battery_stored(&self) -> f32 {
        let mut total_accumulator = 0.0;
        for battery in &self.batteries {
            if battery.enabled { total_accumulator += battery.get_accumulator(); }
        }
        total_accumulator
    }

    pub fn get_battery_capacity(&self) -> f32 {
        let mut total_capacity: f32 = 0.0;
        for battery in &self.batteries {
            if battery.enabled {
                total_capacity += (1.0 - battery.power.status) * battery.block.cons_power.unwrap().capacity;
            }
        }
        total_capacity
    }

    pub fn get_total_battery_capacity(&self) -> f32 {
        let mut total_capacity: f32 = 0.0;
        for battery in &self.batteries {
            if battery.enabled {
                total_capacity += battery.block.cons_power.unwrap().capacity;
            }
        }
        total_capacity
    }

    pub fn use_batteries(&mut self, needed: f32) -> f32 {
        let mut stored = self.get_battery_stored();
        if mathf::equal(stored, 0.0, None) { return 0.0; }

        let used = min(stored, needed);
        let consumed_power_percentage = min(1.0, needed / stored);
        for battery in &self.batteries {
            if battery.enabled {
                battery.power.unwrap().status *= 1.0 - consumed_power_percentage;
            }
        }
        used
    }

    pub fn charge_batteries(&mut self, excess: f32) -> f32 {
        let mut capacity = self.get_battery_capacity();
        // how much of the missing in each battery % is charged
        let charge_power_percentage = min(1.0, excess / capacity);
        if mathf::equal(capacity, 0.0, None) { return 0.0; }

        for battery in &self.batteries {
            if battery.enabled {
                // TODO why would it be 0
                if battery.enabled && battery.block.cons_power.unwrap().capacity > 0.0 {
                    battery.power.unwrap().status += charge_power_percentage * (1.0 - battery.power.unwrap().status);
                }
            }
        }
        min(excess, capacity)
    }

    pub fn distribute_power(&mut self, needed: f32, produced: f32, charged: bool) {
        // distribute even if not needed. this is because some might be requiring power but not using it; it updates consumers
        let coverage = if zero(needed, None) && zero(produced, None) && !charged && zero(self.last_power_stored, None) {
            0.0
        } else {
            if zero(needed, None) {
                1.0
            } else {
                min(1.0, produced / needed)
            }
        };
        for consumer in &self.consumers {
            // TODO how would it even be null
            let cons = consumer.block.cons_power.unwrap();
            if cons.buffered
        }
    }
}