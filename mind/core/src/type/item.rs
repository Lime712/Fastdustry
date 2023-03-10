
use crate::ctype::content_type::ContentType;
use crate::ctype::unlockable_content::UnlockableContent;
use crate::logic::sensible;
use crate::logic::sensible::{LAccess, Sensible};
use crate::world::meta::stat::*;

pub struct Item {
    // pub color: Color,
    pub super_struct: UnlockableContent,

    /// how explosive this item is.
    pub explosiveness: f64,
    /// flammability above 0.3 makes this eligible for item burners.
    pub flammability: f64,
    /// how radioactive this item is.
    pub radioactivity: f64,
    /// how electrically potent this item is.
    pub charge: f64,
    /// drill hardness of the item
    pub hardness: i32,
    /// base material cost of this item, used for calculating place times
    /// 1 cost = 1 tick added to build time
    pub cost: f64,
    /// When this item is present in the build cost, a block's <b>default</b> HEALTH is multiplied by 1 + scaling, where 'scaling' is summed together for ALL item requirement types.
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
            charge: 0.0,
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
    pub fn new(name: String) -> Self {
        Self {
            super_struct: UnlockableContent {
                localized_name: name,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn set_stats(&mut self) {
        self.super_struct.stats.add_percent(EXPLOSIVENESS.clone(), self.explosiveness);
        self.super_struct.stats.add_percent(FLAMMABILITY.clone(), self.flammability);
        self.super_struct.stats.add_percent(RADIOACTIVITY.clone(), self.radioactivity);
        self.super_struct.stats.add_percent(CHARGE.clone(), self.charge);
    }

    pub fn to_string(&self) -> String {
        format!("Item: {}", self.super_struct.localized_name)
    }

    pub fn get_content_type(&self) -> ContentType {
        ContentType::Item
    }
}

impl Sensible for Item {
    fn sense(&self, sensor: LAccess) -> f64 {
        if sensor == *sensible::COLOR {
            // self.color
            todo!()
        } else {
            0.0
        }
    }

    // todo: implement this
    // fn sense_object(&self, sensor: LAccess) -> Option<&dyn Content> {
    //     None
    // }
}