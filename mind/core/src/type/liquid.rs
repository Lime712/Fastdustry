use crate::ctype::unlockable_content::UnlockableContent;

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
    pub explosiveness: i32,
    /// whether this fluid reacts in blocks at all (e.g. slag with water)
    pub block_reactive: bool,
    /// if false, this liquid cannot be a coolant
    pub coolant: bool,
    /// if true, this liquid can move through blocks as a puddle.
    pub move_through_blocks: bool,
    /// if true, this liquid can be incinerated in the incinerator block.
    pub incinerable: bool,
    /// The associated status effect.
    ///to be finished
    /// Effect shown in puddles.
    ///to be finished
    /// Particle effect rate spacing in ticks.
    pub particle_spacing: i32,
    /// Temperature at which this liquid vaporizes. This isn't just boiling.
    pub boil_point: f32,
    /// If true, puddle size is capped.
    pub cap_puddles: bool,
    /// Effect when this liquid vaporizes.
    ///to be finished
    /// If true, this liquid is hidden in most UI.
    pub hidden: bool,
}

impl Default for Liquid {
    fn default() -> Self {
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
            explosiveness: 0,
            block_reactive: true,
            coolant: true,
            move_through_blocks: false,
            incinerable: true,
            particle_spacing: 60,
            boil_point: 2,
            cap_puddles: true,
            hidden: false,
        }
    }
}

impl Liquid {
    pub fn new(name: String, color: Color) {

    }
}