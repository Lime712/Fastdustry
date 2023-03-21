use crate::arc_core::math::geom::mathf::modulo;

/// A simple class keeping track of the mean of a stream of values within a certain window. the WindowedMean will only return a
/// value in case enough data has been sampled. After enough data has been sampled the oldest sample will be replaced by the newest
/// in case a new sample is added.
/// @author badlogicgames@gmail.com
pub struct WindowedMean {
    values: Vec<f32>,
    added_values: usize,
    last_value: usize,
    mean: f32,
    dirty: bool,
}

impl Default for WindowedMean {
    fn default() -> Self {
        WindowedMean {
            values: vec![],
            added_values: 0,
            last_value: 0,
            mean: 0.0,
            dirty: true,
        }
    }
}

impl WindowedMean {
    /// constructor, windowSize specifies the number of samples we will continuously get the mean and variance from. the class
    /// will only return meaning full values if at least windowSize values have been added.
    /// # Parameters
    /// windowSize size of the sample window
    pub fn new(window_size: usize) -> WindowedMean {
        WindowedMean {
            values: vec![0.0; window_size],
            added_values: 0,
            last_value: 0,
            mean: 0.0,
            dirty: true,
        }
    }

    pub fn reset(&mut self) {
        self.added_values = 0;
        self.last_value = 0;
        self.mean = 0.0;
    }

    pub fn get(&self, index: i32) -> f32 {
        self.values[modulo(index + self.last_value as i32, self.values.len() as i32) as usize]
    }

    /// # Return
    /// whether the value returned will be meaningful
    pub fn has_enough_data(&self) -> bool {
        self.added_values >= self.values.len()
    }

    /// clears this WindowedMean. The class will only return meaningful values after enough data has been added again.
    pub fn clear(&mut self) {
        self.added_values = 0;
        self.last_value = 0;
        self.mean = 0.0;
        self.dirty = true;
        self.values = vec![0.0; self.values.len()];
    }

    pub fn fill(&mut self, value: f32) {
        self.dirty = true;
        self.values = vec![value; self.values.len()];
        self.added_values = self.values.len();
    }

    /// adds a new sample to this mean. In case the window is full the oldest value will be replaced by this new value.
    /// # Arguments
    /// * `value` - The value to add
    pub fn add(&mut self, value: f32) {
        if self.added_values < self.values.len() {
            self.added_values += 1;
        }
        self.values[self.last_value] = value;
        self.last_value += 1;
        if self.last_value > self.values.len() - 1 {
            self.last_value = 0;
        }
        self.dirty = true;
    }

    /// returns the mean of the samples added to this instance. Only returns meaningful results when at least window_size samples
    /// as specified in the constructor have been added.
    /// * returns the mean
    pub fn mean(&mut self) -> f32 {
        if self.has_enough_data() {
            if self.dirty {
                let mut mean = 0.0;
                for i in 0..self.values.len() {
                    mean += self.values[i];
                }
                self.mean = mean / self.values.len() as f32;
                self.dirty = false;
            }
            self.mean
        } else {
            0.0
        }
    }

    /// * returns raw mean; can be used before this window has enough data
    pub fn raw_mean(&mut self) -> f32 {
        if self.has_enough_data() {
            self.mean()
        } else if self.added_values == 0 {
            0.0
        } else {
            let mut sum = 0.0;
            for i in 0..self.added_values {
                sum += self.values[i];
            }
            sum / self.added_values as f32
        }
    }

    /// * returns the oldest value in the window
    pub fn oldest(&self) -> f32 {
        todo!()
    }

    pub fn highest(&self) -> f32 {
        // nice rust code to get the highest value in a vector
        self.values.iter().cloned().fold(0. / 0., f32::max)
    }

    pub fn get_count(&self) -> usize {
        self.added_values
    }

    pub fn get_window_size(&self) -> usize {
        self.values.len()
    }
}