use std::cmp::min;
use bit_set::BitSet;

use arc::arc_core::math::windowed_mean::WindowedMean;
use arc::arc_core::util::interval::Interval;

use crate::r#type::item::Item;
use crate::r#type::item_stack::ItemStack;
use crate::vars;

pub static EMPTY: ItemModule = ItemModule::default();
static WINDOW_SIZE: usize = 6;
static mut CACHE_FLOW: Vec<WindowedMean> = vec![];
static mut CACHE_SUMS: Vec<f32> = vec![];
static mut DISPLAY_FLOW: Vec<f32> = vec![];
static mut CACHE_BITS: BitSet = BitSet::new();
static mut FLOW_TIMER: Interval = Interval::new(2);
static POLL_SCL: f32 = 20.0;

pub struct ItemModule {
    items: Vec<i32>,
    total: i32,
    take_rotation: i32,
    flow: Option<Vec<WindowedMean>>,
}

impl Default for ItemModule {
    fn default() -> Self {
        ItemModule {
            items: vec![],
            total: 0,
            take_rotation: 0,
            flow: None,
        }
    }
}

impl ItemModule {
    pub fn copy(&self) -> ItemModule {
        let mut i = ItemModule::default();
        i.set(self);
        i
    }

    pub fn set(&mut self, other: &ItemModule) {
        self.total = other.total;
        self.take_rotation = other.take_rotation;
        self.items = other.items.clone();
    }

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

    pub fn length(&self) -> usize {
        self.items.len()
    }

    /// * returns a specific item's flow rate in items/s; any value < 0 means not ready
    pub fn get_flow_rate(&self, item: Item) -> f32 {
        if let Some(flow) = &self.flow {
            unsafe { DISPLAY_FLOW[item.id() as usize] * 60.0 }
        } else {
            -1.0
        }
    }

    pub fn has_flow_itme(&self, item: Item) -> bool {
        if let Some(flow) = &self.flow {
            unsafe { CACHE_BITS.contains(item.id() as usize) }
        } else {
            false
        }
    }

    pub fn each(&self, mut f: impl FnMut(Item, i32)) {
        for i in 0..self.items.len() {
            if self.items[i] != 0 {
                f(vars::CONTENT.item(i as usize), self.items[i]);
            }
        }
    }

    pub fn sum(&self, mut f: impl FnMut(Item, i32) -> i32) -> i32 {
        let mut sum = 0;
        for i in 0..self.items.len() {
            if self.items[i] != 0 {
                sum += f(vars::CONTENT.item(i as usize), self.items[i]);
            }
        }
        sum
    }

    pub fn has(&self, id: usize) -> bool {
        self.items[id] > 0
    }

    pub fn has_item(&self, item: Item) -> bool {
        self.get(item) > 0
    }

    pub fn has_item_amount(&self, item: Item, amount: i32) -> bool {
        self.get(item) >= amount
    }

    pub fn has_item_stack(&self, stacks: Vec<ItemStack>) -> bool {
        for stack in stacks {
            if !self.has_item_amount(stack.item, stack.amount) {
                return false;
            }
        }
        true
    }

    pub fn empty(&self) -> bool {
        self.total == 0
    }

    pub fn total(&self) -> i32 {
        self.total
    }

    pub fn any(&self) -> bool {
        self.total > 0
    }

    pub fn first(&self) -> Option<Item> {
        for i in 0..self.items.len() {
            if self.items[i] > 0 {
                return Some(vars::CONTENT.item(i));
            }
        }
        None
    }

    pub fn take(&mut self) -> Option<Item> {
        for i in 0..self.items.len() {
            let mut index = (i + self.take_rotation as usize);
            if index >= self.items.len() {
                index -= self.items.len();
            }
            if self.items[index] > 0 {
                self.items[index] -= 1;
                self.total -= 1;
                self.take_rotation = index as i32 + 1;
                return Some(vars::CONTENT.item(index));
            }
        }
        None
    }

    pub fn get_id(&self, id: usize) -> i32 {
        self.items[id]
    }

    pub fn get_item(&self, item: Item) -> i32 {
        self.items[item.id() as usize]
    }

    pub fn set_item_amount(&mut self, item: Item, amount: i32) {
        self.total += (amount - self.items[item.id() as usize]);
        self.items[item.id() as usize] = amount;
    }

    pub fn add_id_amount(&mut self, id: usize, amount: i32) {
        self.items[id] += amount;
        self.total += amount;
        if !self.flow.is_none() {
            unsafe { CACHE_SUMS[id] += amount as f32; }
        }
    }

    pub fn add_item_amount(&mut self, item: Item, amount: i32) {
        self.add_id_amount(item.id() as usize, amount);
    }

    pub fn add_item_module(&mut self, other: &ItemModule) {
        for i in 0..self.items.len() {
            self.add_item_amount(vars::CONTENT.item(i), other.items[i]);
        }
    }

    pub fn handle_flow(&mut self, item: Item, amount: i32) {
        if !self.flow.is_none() {
            unsafe { CACHE_SUMS[item.id() as usize] += amount as f32; }
        }
    }

    pub fn undo_flow(&mut self, item: Item, amount: i32) {
        if !self.flow.is_none() {
            unsafe { CACHE_SUMS[item.id() as usize] -= amount as f32; }
        }
    }

    pub fn remove(&mut self, item: Item, mut amount: i32) {
        amount = min(amount, self.items[item.id() as usize]);

        self.items[item.id() as usize] -= amount;
        self.total -= amount;
    }

    pub fn remove_stack(&mut self, stack: ItemStack) {
        self.remove(stack.item, stack.amount);
    }

    pub fn clear(&mut self) {
        self.items = vec![0; self.items.len()];
        self.total = 0;
    }
}