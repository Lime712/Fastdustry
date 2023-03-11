use crate::world::meta::attribute::{ALL_ATTRIBUTES, Attribute};

pub struct Attributes {
    arr: Vec<f32>,
}

impl Default for Attributes {
    fn default() -> Self {
        Self { arr: Vec::new() }
    }
}

impl Attributes {
    pub fn clear(&mut self) {
        self.arr.clear();
    }

    pub fn get(&mut self, atr: &Attribute) -> f32 {
        self.check();
        if (atr.id as usize) < self.arr.len() {
            return self.arr[atr.id as usize];
        }
        0.0
    }

    pub fn set(&mut self, atr: &Attribute, val: f32) {
        self.check();
        if (atr.id as usize) < self.arr.len() {
            self.arr[atr.id as usize] = val;
        }
    }

    pub fn add(&mut self, mut atrs: &Attributes) {
        self.check();
        atrs.check();
        for i in 0..self.arr.len() {
            self.arr[i] += atrs.arr[i];
        }
    }

    fn check(&mut self) {
        if self.arr.len() != unsafe { ALL_ATTRIBUTES.len() } {
            self.arr.clear();
            for a in unsafe { ALL_ATTRIBUTES.iter() } {
                self.arr.push(0.0);
            }
        }
    }
}