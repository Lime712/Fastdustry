use std::ops::BitOr;

pub enum BlockGroup {
    None,
    Walls,
    Projectors,
    Turrets,
    Transportation,
    Power,
    Liquids,
    Drills,
    Units,
    Logic,
    Payloads,
}

impl BlockGroup {
    pub fn any_replace(&self) -> bool {
        match self {
            BlockGroup::None => false,
            BlockGroup::Walls => true,
            BlockGroup::Projectors => true,
            BlockGroup::Turrets => true,
            BlockGroup::Transportation => true,
            BlockGroup::Power => false,
            BlockGroup::Liquids => true,
            BlockGroup::Drills => false,
            BlockGroup::Units => false,
            BlockGroup::Logic => true,
            BlockGroup::Payloads => true,
        }
    }
}

/// Stores special flags of blocks for easy querying.
pub enum BlockFlag {
    /** Enemy core; primary target for all units. */
    Core,
    /** Vault/container/etc */
    Storage,
    /** Something that generates power. */
    Generator,
    /** Any turret. */
    Turret,
    /** A block that transforms resources. */
    Factory,
    /** Repair point. */
    Repair,
    /** Block that stored power for resupply. */
    Battery,
    /** Any reactor block. */
    Reactor,
    /** Blocks that extinguishes fires. */
    Extinguisher,
    /** Is a drill. */
    Drill,

    /// special, internal identifiers
    LaunchPad,
    UnitCargoUnloadPoint,
    UnitAssembler,
    HasFogRadius,
}

/// Values for logic only. Filters out some internal flags.
pub static LOGIC_BLOCK_FLAGS: [BlockFlag; 8] = [
    BlockFlag::Core,
    BlockFlag::Storage,
    BlockFlag::Generator,
    BlockFlag::Turret,
    BlockFlag::Factory,
    BlockFlag::Repair,
    BlockFlag::Battery,
    BlockFlag::Reactor
];

/** Environmental flags for different types of locations. */
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Env {
    /// is on a planet
    Terrestrial = 1,
    /// is in space, no atmosphere
    Space = 1 << 1,
    /// is underwater, on a planet
    Underwater = 1 << 2,
    /// has a spores
    Spores = 1 << 3,
    /// has a scorching env effect
    Scorching = 1 << 4,
    /// has oil reservoirs
    GroundOil = 1 << 5,
    /// has water reservoirs
    GroundWater = 1 << 6,
    /// has oxygen in the atmosphere
    Oxygen = 1 << 7,
    /// all attributes combined, only used for bitmasking purposes
    Any = 0xffffffff,
    /// no attributes (0)
    None = 0
}