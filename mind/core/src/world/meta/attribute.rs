use std::collections::HashMap;
use std::fmt::Display;
use lazy_static::lazy_static;
use crate::vars;


#[derive(Clone, Debug, PartialEq)]
pub struct Attribute {
    pub name: &'static str,
    pub id: f32,
}

pub static mut ALL_ATTRIBUTES: Vec<Attribute> = Vec::new();
pub static mut ALL_ATTRIBUTES_MAP: HashMap<&'static str, Attribute> = HashMap::new();

impl Attribute {
    pub fn new(name: &'static str) -> Attribute {
        let id = unsafe {
            ALL_ATTRIBUTES.len() as f32
        };
        let a = Attribute {
            name,
            id,
        };
        unsafe {
            ALL_ATTRIBUTES.push(a.clone());
            ALL_ATTRIBUTES_MAP.insert(name, a.clone());
        }
        a
    }
    /// # Return
    /// the environmental value for this attribute.
    pub fn env(&self) -> f32 {
        return if vars::state == None {
            0.0
        } else {
            vars::state::env_attributes.get(self).unwrap_or(0.0)
        }
    }
}

lazy_static! {
    /// Heat content. Used for thermal generator yield.
    pub static ref HEAT: Attribute = Attribute::new("heat");
    /// Spore content. Used for cultivator yield.
    pub static ref SPORES: Attribute = Attribute::new("spores");
    /// Water content. Used for water extractor yield.
    pub static ref WATER: Attribute = Attribute::new("water");
    /// Oil content. Used for oil extractor yield.
    pub static ref OIL: Attribute = Attribute::new("oil");
    /// Light coverage. Negative values decrease solar panel efficiency.
    pub static ref LIGHT: Attribute = Attribute::new("light");
    /// Used for sand extraction.
    pub static ref SAND: Attribute = Attribute::new("sand");
    /// Used for erekir vents only.
    pub static ref STEAM: Attribute = Attribute::new("steam");
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Never returns null, may throw an exception if not found.
pub fn get_attribute(name: &str) -> Attribute {
    unsafe {
        ALL_ATTRIBUTES_MAP.get(name).unwrap().clone()
    }
}


