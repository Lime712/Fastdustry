use std::cmp::max;

use bit_set::BitSet;

use arc::arc_core::math::windowed_mean::WindowedMean;
use arc::arc_core::util::interval::Interval;

use crate::r#type::liquid::Liquid;

static WINDOW_SIZE: usize = 3;
static mut FLOW_TIMER: Interval = Interval::new(2);
static POLL_SCL: f32 = 20.0;

static mut CACHE_FLOW: Vec<WindowedMean> = vec![];
static mut CACHE_SUMS: Vec<f32> = vec![];
static mut DISPLAY_FLOW: Vec<f32> = vec![];
static mut CACHE_BITS: BitSet = BitSet::new();

pub struct LiquidModule {
    liquids: Vec<f32>,
    current: Liquid,
    flow: Option<Vec<WindowedMean>>,
}

impl LiquidModule {
    pub fn update_flow(&mut self) {
        if unsafe { FLOW_TIMER.get(1, POLL_SCL) } {
            unsafe {
                if self.flow.is_none() {
                    if CACHE_FLOW.is_empty() || CACHE_FLOW.len() != self.items.len() {
                        CACHE_FLOW = vec![WindowedMean::new(WINDOW_SIZE); self.items.len()];
                        CACHE_SUMS = vec![0.0; self.items.len()];
                        DISPLAY_FLOW = vec![0.0; self.items.len()];
                    } else {
                        for i in 0..self.items.len() {
                            CACHE_FLOW[i].reset();
                        }
                        CACHE_SUMS = vec![0.0; self.items.len()];
                        CACHE_BITS.clear();
                    }
                    DISPLAY_FLOW = vec![-1.0; self.items.len()];
                    self.flow = Some(CACHE_FLOW.clone());
                }

                let update_flow = FLOW_TIMER.get(0, 30.0);

                for i in 0..self.items.len() {
                    self.flow[i].add(CACHE_SUMS[i]);
                    if CACHE_SUMS[i] > 0.0 {
                        CACHE_BITS.insert(i);
                    }
                    CACHE_SUMS[i] = 0.0;

                    if update_flow {
                        DISPLAY_FLOW[i] = if self.flow[i].has_enough_data() {
                            self.flow[i].mean() / POLL_SCL
                        } else {
                            -1.0
                        };
                    }
                }
            }
        }
    }

    pub fn stop_flow(&mut self) {
        self.flow = None;
    }

    /// * returns current liquid's flow rate in u/s; any value < 0 means 'not ready'.
    pub fn get_flow_rate(&self, liquid: Liquid) -> f32 {
        if let Some(flow) = &self.flow {
            unsafe { DISPLAY_FLOW[liquid.id() as usize] * 60.0 }
        } else {
            -1.0
        }
    }

    pub fn has_flow_liquid(&self, liquid: Liquid) -> bool {
        if let Some(flow) = &self.flow {
            unsafe { CACHE_BITS.contains(liquid.id() as usize) }
        } else {
            false
        }
    }

    /// Last received or loaded liquid. Only valid for liquid modules with 1 type of liquid.
    pub fn current(&self) -> &Liquid {
        &self.current
    }

    pub fn reset(&mut self, liquid: Liquid, amount: f32) {
        self.current = liquid;
        self.liquids = vec![0.0; self.items.len()];
        self.liquids[liquid.id() as usize] = amount;
    }

    pub fn current_amount(&self) -> f32 {
        self.liquids[self.current.id() as usize]
    }

    pub fn get(&self, liquid: Liquid) -> f32 {
        self.liquids[liquid.id() as usize]
    }

    pub fn clear(&mut self) {
        self.liquids = vec![0.0; self.items.len()];
    }

    pub fn add(&mut self, liquid: Liquid, amount: f32) {
        self.liquids[liquid.id() as usize] += amount;
        self.current = liquid;

        if let Some(_flow) = &mut self.flow {
            unsafe { CACHE_SUMS[liquid.id() as usize] += amount; }
        }
    }

    pub fn handle_flow(&mut self, liquid: Liquid, amount: i32) {
        if !self.flow.is_none() {
            unsafe { CACHE_SUMS[liquid.id() as usize] += max(amount, 0) as f32; }
        }
    }

    pub fn remove(&mut self, liquid: Liquid, amount: f32) {
        //cap to prevent negative removal
        self.add(liquid, max(-amount, -self.liquids[liquid.id() as usize]));
    }

    pub fn each(&self, mut f: impl FnMut(Liquid, f32)) {
        for i in 0..self.items.len() {
            if self.liquids[i] > 0.0 { f(self.items[i], self.liquids[i]); }
        }
    }

    pub fn sum(&self, mut f: impl FnMut(Liquid, f32) -> f32) -> f32 {
        let mut sum = 0.0;
        for i in 0..self.items.len() {
            if self.liquids[i] > 0.0 {
                sum += f(self.items[i], self.liquids[i]);
            }
        }
        sum
    }
}