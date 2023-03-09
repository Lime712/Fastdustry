use crate::ctype::unlockable_content::UnlockableContent;

pub struct Item {
    // pub color: Color,
    pub super_struct: UnlockableContent,

    /// how explosive this item is.
    pub explosiveness: f32,
    /// flammability above 0.3 makes this eligible for item burners.
    pub flammability: f32,
    /// how radioactive this item is.
    pub radioactivity: f32,
    /// drill hardness of the item
    pub hardness: i32,
    /// base material cost of this item, used for calculating place times
    /// 1 cost = 1 tick added to build time
    pub cost: f64,
    /// When this item is present in the build cost, a block's <b>default</b> health is multiplied by 1 + scaling, where 'scaling' is summed together for ALL item requirement types.
    pub health_scaling: f64,
    /// if true, this item is of the lowest priority to drills.
    pub low_priority: bool,
    /// If >0, this item is animated.
    pub frames: i32,
    /// Number of generated transition frames between each frame.
    pub transition_frames: i32,
    /// Ticks in-between animation frames.
    pub frame_time: i32,
    /// If true, this material is used by buildings. If false, this material will be incinerated in certain cores.
    pub buildable: bool,
    pub hidden: bool,
    // For mods. Adds this item to the listed planets' hidden items Seq.
    // not implemented
}

impl Default for Item {
    fn default() -> Self {
        Self {
            super_struct: UnlockableContent::default(),
            // color: Color::default(),
            explosiveness: 0.0,
            flammability: 0.0,
            radioactivity: 0.0,
            hardness: 0,
            cost: 1.0,
            health_scaling: 0.0,
            low_priority: false,
            frames: 0,
            transition_frames: 0,
            frame_time: 5,
            buildable: true,
            hidden: false,
        }
    }
}

impl Item {
    pub fn new(name: String) {

    }
}