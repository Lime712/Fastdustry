use crate::arc_core::util::time;

pub struct Interval {
    times: Vec<f32>,
}

impl Default for Interval {
    fn default() -> Self {
        Self::new(1)
    }
}

impl Interval {
    pub fn new(times: usize) -> Self {
        Self {
            times: vec![0.0; times],
        }
    }

    pub fn get(&mut self, id: usize, time: f32) -> bool {
        if id >= self.times.len() {
            panic!("Interval ID out of bounds: {} >= {}", id, self.times.len());
        }

        let got = self.check(id, time);
        if got {
            self.times[id] = unsafe { time::TIME };
        }
        return got;
    }

    pub fn check(&self, id: usize, time: f32) -> bool {
        unsafe { time::TIME - self.times[id] >= time || time::TIME < self.times[id] }
    }

    pub fn reset(&mut self, id: usize, time: f32) {
        self.times[id] = unsafe { time::TIME } - time;
    }

    pub fn clear(&mut self) {
        for i in 0..self.times.len() {
            self.times[i] = 0.0;
        }
    }

    pub fn get_times(&self) -> &Vec<f32> {
        &self.times
    }
}
