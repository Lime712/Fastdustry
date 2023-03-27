use crate::game::team::Team;

/// Class for various team-based utilities.
pub struct Teams {
    /// Maps team IDs to team data.
    map: [TeamData; 256],

}


pub struct TeamData {
    team: Team,
    /// Handles RTS unit control.
    pub rts_ai: Option<RtsAi>,
    present_flag: bool,
    /// Enemies with cores or spawn points.
    pub core_enemies: Vec<Team>,
    /// Planned blocks for drones. This is usually only blocks that have been broken.
    pub plans: Vec<BlockPlan>,
    /// List of live cores of this team.
    pub cores: Vec<CoreBuild>,
    /// Last known live core of this team.
    pub last_core: Option<CoreBuild>,
    /// Quadtree for all buildings of this team. Null if not active.

}

/// Represents a block made by this team that was destroyed somewhere on the map.
/// This does not include deconstructed blocks.
pub struct BlockPlan {
    pub x: i16,
    pub y: i16,
    pub rotation: i16,
    pub block: i16,
    pub config: Vec<u8>,
    pub removed: bool,
}