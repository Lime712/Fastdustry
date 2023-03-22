pub static BLOCKS: StatUnit = StatUnit::new("blocks");
pub static BLOCKS_SQUARED: StatUnit = StatUnit::new("blocksSquared");
pub static TILES_SECOND: StatUnit = StatUnit::new("tilesSecond");
pub static POWER_SECOND: StatUnit = StatUnit::new("powerSecond");
pub static LIQUID_SECOND: StatUnit = StatUnit::new("liquidSecond");
pub static ITEMS_SECOND: StatUnit = StatUnit::new("itemsSecond");
pub static LIQUID_UNITS: StatUnit = StatUnit::new("liquidUnits");
pub static POWER_UNITS: StatUnit = StatUnit::new("powerUnits");
pub static HEAT_UNITS: StatUnit = StatUnit::new("heatUnits");
pub static DEGREES: StatUnit = StatUnit::new("degrees");
pub static SECONDS: StatUnit = StatUnit::new("seconds");
pub static MINUTES: StatUnit = StatUnit::new("minutes");
pub static PER_SECOND: StatUnit = StatUnit::new_no_space("perSecond");
pub static PER_MINUTE: StatUnit = StatUnit::new_no_space("perMinute");
pub static PER_SHOT: StatUnit = StatUnit::new_no_space("perShot");
pub static TIMES_SPEED: StatUnit = StatUnit::new_no_space("timesSpeed");
pub static PERCENT: StatUnit = StatUnit::new_no_space("percent");
pub static SHIELD_HEALTH: StatUnit = StatUnit::new("shieldHealth");
pub static NONE: StatUnit = StatUnit::new("none");
pub static ITEMS: StatUnit = StatUnit::new("items");


/// Defines a unit of measurement for block stats.
pub struct StatUnit {
    pub space: bool,
    pub name: &'static str,
    pub icon: Option<&'static str>,
}

impl StatUnit {
    pub fn new(name: &'static str) -> Self {
        Self {
            space: true,
            name,
            icon: None,
        }
    }

    pub fn new_no_space(name: &'static str) -> Self {
        Self {
            space: false,
            name,
            icon: None,
        }
    }

    pub fn localized(&self) -> &'static str {
        if self == NONE {
            return "None";
        }
        return "stat.unit." + self.name;
    }
}

impl Clone for StatUnit {
    fn clone(&self) -> Self {
        Self {
            space: self.space,
            name: self.name,
            icon: self.icon,
        }
    }
}
