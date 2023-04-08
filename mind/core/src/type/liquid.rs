use std::collections::HashSet;
use crate::ctype::unlockable_content::UnlockableContent;
use crate::logic::sensible::{COLOR, LAccess, Sensible};
use crate::world::meta::stat::{EXPLOSIVENESS, FLAMMABILITY, HEAT_CAPACITY, RADIOACTIVITY, TEMPERATURE, VISCOSITY};

#[derive(Debug, Clone)]
pub struct Liquid {
    pub animation_frames: i32,
    pub animation_scale_gas: i32,
    pub animation_scale_liquid: i32,
    pub super_struct: UnlockableContent,

    /// If true, this fluid is treated as a gas (and does not create puddles)
    pub gas: bool,
    /// Color used in pipes and on the ground.
    ///to be finished
    /// Color of this liquid in gas form.
    ///to be finished
    /// Color used in bars.
    ///to be finished
    /// Color used to draw lights. Note that the alpha channel is used to dictate brightness.
    ///to be finished
    /// 0-1, 0 is completely not flammable, anything above that may catch fire when exposed to heat, 0.5+ is very flammable.
    pub flammability: f32,
    /// temperature: 0.5 is 'room' temperature, 0 is very cold, 1 is molten hot
    pub temperature: f32,
    /// how much heat this liquid can store. 0.4=water (decent), anything lower is probably less dense and bad at cooling.
    pub heat_capacity: f32,
    /// how thick this liquid is. 0.5=water (relatively viscous), 1 would be something like tar (very slow).
    pub viscosity: f32,
    /// how prone to exploding this liquid is, when heated. 0 = nothing, 1 = nuke
    pub explosiveness: f32,
    /// whether this fluid reacts in blocks at all (e.g. slag with water)
    pub block_reactive: bool,
    /// if false, this liquid cannot be a coolant
    pub coolant: bool,
    /// if true, this liquid can move through blocks as a puddle.
    pub move_through_blocks: bool,
    /// if true, this liquid can be incinerated in the incinerator block.
    pub incinerable: bool,
    /// The associated status effect.
    /// todo
    /// Effect shown in puddles.
    /// todo
    /// Particle effect rate spacing in ticks.
    pub particle_spacing: i32,
    /// Temperature at which this liquid vaporizes. This isn't just boiling.
    pub boil_point: f32,
    /// If true, puddle size is capped.
    pub cap_puddles: bool,
    /// Effect when this liquid vaporizes.
    /// todo
    /// If true, this liquid is hidden in most UI.
    pub hidden: bool,
    pub can_stay_on: HashSet<&'static Liquid>,
}

impl Default for Liquid {
    const fn default() -> Self {
        Self {
            animation_frames: 50,
            animation_scale_gas: 190,
            animation_scale_liquid: 230,
            super_struct: UnlockableContent::default(),
            gas: false,
            flammability: 0.0,
            temperature: 0.5,
            heat_capacity: 0.5,
            viscosity: 0.5,
            explosiveness: 0.0,
            block_reactive: true,
            coolant: true,
            move_through_blocks: false,
            incinerable: true,
            particle_spacing: 60,
            boil_point: 2.0,
            cap_puddles: true,
            hidden: false,
            can_stay_on: HashSet::new(),
        }
    }
}

impl Liquid {
    pub const fn new(name: &'static str) -> Self {
        Self {
            super_struct: UnlockableContent::new(name),
            ..Default::default()
        }
    }

    pub fn set_stats(&mut self) {
        self.super_struct
            .stats
            .add_percent(EXPLOSIVENESS.clone(), self.explosiveness);
        self.super_struct
            .stats
            .add_percent(FLAMMABILITY.clone(), self.flammability);
        self.super_struct
            .stats
            .add_percent(TEMPERATURE.clone(), self.temperature);
        self.super_struct
            .stats
            .add_percent(HEAT_CAPACITY.clone(), self.heat_capacity);
        self.super_struct
            .stats
            .add_percent(VISCOSITY.clone(), self.viscosity);
    }
}

impl Sensible for Liquid {
    // TODO: implement rest of the functions
    fn sense(&self, sensor: LAccess) -> f32 {
        if sensor == *COLOR {
            // return color;
            0.0
        } else {
            0.0
        }
    }
}

impl PartialEq for Liquid {
    fn eq(&self, other: &Self) -> bool {
        self.super_struct.localized_name == other.super_struct.localized_name
    }
}

impl Eq for Liquid {}

impl std::hash::Hash for Liquid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.super_struct.localized_name.hash(state);
    }
}