use std::collections::{HashMap, HashSet};

use crate::world::meta::stat::{Stat, StatCat, StatValue};

pub struct Stats {
    /// Whether to display stats with categories. If false, categories are completely ignored during display.
    pub use_categories: bool,
    /// Whether these stats are initialized yet.
    pub initialized: bool,
    /// Production time period in ticks. Used for crafters.
    pub time_period: f64,
    pub map: HashMap<StatCat, HashMap<Stat, HashSet<StatValue>>>,
    pub dirty: bool,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            use_categories: true,
            initialized: false,
            time_period: 0.0,
            map: HashMap::new(),
            dirty: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    pub fn add(&mut self, stat: Stat, value: StatValue) {
        self.add_cat(stat, StatCat::GENERAL, value);
    }

    pub fn add_cat(&mut self, stat: Stat, category: StatCat, value: StatValue) {
        if !self.initialized {
            self.init();
        }
        let mut map = self.map.entry(category).or_insert(HashMap::new());
        let mut set = map.entry(stat).or_insert(HashSet::new());
        set.insert(value);
        self.dirty = true;
    }

    pub fn add_percent(&mut self, stat: Stat, percent: f64) {
        self.add_percent_cat(stat, StatCat::GENERAL, percent);
    }

    pub fn add_percent_cat(&mut self, stat: Stat, category: StatCat, percent: f64) {
        if !self.initialized {
            self.init();
        }
        let mut map = self.map.entry(category).or_insert(HashMap::new());
        let mut set = map.entry(stat).or_insert(HashSet::new());
        set.insert(StatValue {
            value: percent,
            display: format!("{}%", percent),
        });
        self.dirty = true;
    }

    pub fn get(&self, stat: Stat) -> Option<&HashSet<StatValue>> {
        self.get_cat(stat, StatCat::GENERAL)
    }

    pub fn get_cat(&self, stat: Stat, category: StatCat) -> Option<&HashSet<StatValue>> {
        if !self.initialized {
            return None;
        }
        let map = self.map.get(&category)?;
        let set = map.get(&stat)?;
        Some(set)
    }

    pub fn get_all(&self) -> &HashMap<StatCat, HashMap<Stat, HashSet<StatValue>>> {
        &self.map
    }

    pub fn get_all_mut(&mut self) -> &mut HashMap<StatCat, HashMap<Stat, HashSet<StatValue>>> {
        &mut self.map
    }

    pub fn remove(&mut self, stat: Stat) {
        self.remove_cat(stat, StatCat::GENERAL);
    }

    pub fn remove_cat(&mut self, stat: Stat, category: StatCat) {
        if !self.initialized {
            return;
        }
        let map = self.map.get_mut(&category).unwrap();
        map.remove(&stat);
        self.dirty = true;
    }
}