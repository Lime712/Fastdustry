use std::collections::HashMap;
use crate::world::block::Block;

pub struct GameStats{
//     /** Enemy (red team) units destroyed. */
//     public int enemyUnitsDestroyed;
//     /** Total waves lasted. */
//     public int wavesLasted;
//     /** Friendly buildings fully built. */
//     public int buildingsBuilt;
//     /** Friendly buildings fully deconstructed. */
//     public int buildingsDeconstructed;
//     /** Friendly buildings destroyed. */
//     public int buildingsDestroyed;
//     /** Total units created by any means. */
//     public int unitsCreated;
//     /** Record of blocks that have been placed by count. Used for objectives only. */
//     public ObjectIntMap<Block> placedBlockCount = new ObjectIntMap<>();
//     /**
//      * Record of items that have entered the core through transport blocks. Used for objectives only.
//      * This can easily be ""spoofed"" with unloaders, so don't use it for anything remotely important.
//      * */
//     public ObjectIntMap<Item> coreItemCount = new ObjectIntMap<>();
    /// Enemy (red team) units destroyed.
    pub enemy_units_destroyed: i32,
    /// Total waves lasted.
    pub waves_lasted: i32,
    /// Friendly buildings fully built.
    pub buildings_built: i32,
    /// Friendly buildings fully deconstructed.
    pub buildings_deconstructed: i32,
    /// Friendly buildings destroyed.
    pub buildings_destroyed: i32,
    /// Total units created by any means.
    pub units_created: i32,
    /// Record of blocks that have been placed by count. Used for objectives only.
    pub placed_block_count: HashMap<Block, i32>,
    /// Record of items that have entered the core through transport blocks. Used for objectives only.
    /// This can easily be ""spoofed"" with unloaders, so don't use it for anything remotely important.
    pub core_item_count: HashMap<Block, i32>,
}

impl Default for GameStats{
    fn default() -> Self {
        Self{
            enemy_units_destroyed: 0,
            waves_lasted: 0,
            buildings_built: 0,
            buildings_deconstructed: 0,
            buildings_destroyed: 0,
            units_created: 0,
            placed_block_count: HashMap::new(),
            core_item_count: HashMap::new(),
        }
    }
}