
use crate::r#type::liquid::Liquid;

pub struct CellLiquid {
    pub liquid: Liquid,
    pub cells: u8,
    pub spread_target: Option<Liquid>,
    pub max_spread:f64,
    pub spread_conversion: f64,
    pub spread_damage: f64,
    pub remove_scaling: f64,
}

impl Default for CellLiquid {
    fn default() -> Self {
        Self {
            liquid: Liquid::default(),
            cells: 8,
            spread_target: None,
            max_spread: 0.75,
            spread_conversion: 1.2,
            spread_damage: 0.11,
            remove_scaling: 0.25,
        }
    }
}

impl CellLiquid {
    pub fn new(name: &'static str) -> Self {
        Self {
            liquid: Liquid::new(name),
            ..Default::default()
        }
    }
}

// todo: implement all the logic stuff