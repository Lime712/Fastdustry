use std::collections::{HashMap, HashSet};

use crate::game::team::{ALL, CRUX, SHARDED, Team};
use crate::gen::building::Building;
use crate::vars::STATE;
use crate::world::block::Block;
use crate::world::blocks::storage::core_block::CoreBuild;

/// Class for various team-based utilities.
pub struct Teams {
    /// Maps team IDs to team data.
    map: [Option<TeamData>; 256],
    /// Active teams.
    pub active: Vec<&'static TeamData>,
    /// Teams with block or unit presence.
    pub present: Vec<&'static TeamData>,
    /// Current boss units.
    pub bosses: Vec<Unit>,
}

impl Teams {
    pub fn new() -> Teams {
        let mut s = Self {
            map: [None; 256],
            active: Vec::new(),
            present: Vec::new(),
            bosses: Vec::new(),
        };
        s.active.push(s.get(CRUX));
        s
    }

    /// Returns team data by type.
    pub fn get(&mut self, team: &Team) -> &mut TeamData {
        if self.map[team.id].is_none() {
            self.map[team.id] = Some(TeamData::new(team));
            self.map[team.id].as_mut().unwrap()
        } else {
            self.map[team.id].as_mut().unwrap()
        }
    }

    /// Do not modify
    pub fn get_active(&mut self) -> Vec<&TeamData> {
        // first check if all teams in `self.active` are still active
        self.active.retain(|team| team.active());
        active
    }

    pub fn register_core(&mut self, core: CoreBuild) {
        let mut data = self.get(core.team);
        // add core if not present
        if !data.cores.contains(&core) {
            data.cores.push(core);
        }

        // register in active list if needed
        if data.active() && !self.active.contains(&data) {
            self.active.push(data);
            self.update_enemies();
        }
    }

    pub fn unregister_core(&mut self, core: CoreBuild) {
        let mut data = self.get(core.team);
        // remove core if present
        if data.cores.contains(&core) {
            data.cores.retain(|c| c != &core);
        }

        // remove from active list if needed
        if !data.active() && self.active.contains(&data) {
            self.active.retain(|d| d != &data);
            self.update_enemies();
        }
    }

    fn count(&mut self, unit: Unit) {
        todo!("count")
    }

    pub fn update_team_stats(&mut self) {
        self.present.clear();
        self.bosses.clear();

        for team in ALL {
            let mut data = team.data();

            data.present_flag = data.buildings.len() > 0;
            data.unit_count = 0;
            data.units.clear();
            if data.cores.len() > 0 {
                data.last_core = Some(data.cores[0]);
            }
            // todo: check unitTree for units

            if !data.type_counts.is_none() {
                data.type_counts.unwrap().fill(0);
            }

            // clear old unit records
            if !data.units_by_type.is_none() {
                data.units_by_type.unwrap().map(|mut u| {
                    if !u.is_none() {
                        u.unwrap().clear();
                    }
                });
            }
        }

        // todo this is slow and dumb
        for
    }
}

pub struct TeamData {
    team: &'static Team,
    /// Handles RTS unit control.
    pub rts_ai: Option<RtsAi>,
    present_flag: bool,
    /// Enemies with cores or spawn points.
    pub core_enemies: Vec<&'static Team>,
    /// Planned blocks for drones. This is usually only blocks that have been broken.
    pub plans: Vec<BlockPlan>,
    /// List of live cores of this team.
    pub cores: Vec<CoreBuild>,
    /// Last known live core of this team.
    pub last_core: Option<CoreBuild>,
    /// Quadtree for all buildings of this team. Null if not active.
    // todo: implement those quadtree things

    /// Current unit cap. Do not modify externally.
    pub unit_cap: i32,
    /// Total unit count.
    pub unit_count: i32,
    /// Counts for each type of unit. Do not access directly.
    pub type_counts: Option<[i32; 256]>,
    /// Cached buildings by type.
    pub building_types: HashMap<&'static Block, Vec<Building>>,
    /// Units of this team. Updated every frame.
    pub units: HashSet<Unit>,
    /// Same as units but for players
    pub players: HashSet<Unit>,
    /// All buildings. Updated on team change / building addition or removal. Includes even buildings that do not update().
    pub buildings: HashSet<Building>,
    /// Units of this team by type. Updated each frame.
    pub units_by_type: Option<Vec<Option<Vec<Unit>>>>,
}

impl Default for TeamData {
    fn default() -> Self {
        Self {
            team: SHARDED,
            rts_ai: None,
            present_flag: false,
            core_enemies: Vec::new(),
            plans: Vec::new(),
            cores: Vec::new(),
            last_core: None,
            unit_cap: 0,
            unit_count: 0,
            unit_counts: None,
            building_types: HashMap::new(),
            units: HashSet::new(),
            players: HashSet::new(),
            buildings: HashSet::new(),
            units_by_type: None,
        }
    }
}

impl TeamData {
    pub fn new(team: &Team) -> Self {
        let mut s = Self::default();
        s.team = team;
        s
    }

    pub fn get_buildings(&mut self, block: &Block) -> &Vec<Building> {
        if !self.building_types.contains_key(block) {
            self.building_types.insert(block, Vec::new());
        }
        self.building_types.get(block).unwrap()
    }

    pub fn get_count(&self, block: &Block) -> i32 {
        let res = self.building_types.get(block);
        if res.is_none() {
            0
        } else {
            res.unwrap().len() as i32
        }
    }

    /// Destroys this team's presence on the map, killing part of its buildings and converting everything to 'derelict'.
    pub fn destroy_to_derelict(&mut self) {
        // grab all buildings from quadtree
        todo!("destroy_to_derelict")
    }

    pub fn active(&self) -> bool {
        (self.team == unsafe { STATE.unwrap().rules.wave_team })
    }
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


pub trait Entity {
    fn set_id(&mut self, id: i32);
    fn get_id(&self) -> i32;
    fn set_x(&mut self, x: f32);
    fn get_x(&self) -> f32;
    fn set_y(&mut self, y: f32);
    fn get_y(&self) -> f32;
}

pub struct Player {
    id: i32,
    x: f32,
    y: f32,
    pub name: String,
}

impl Entity for Player {
    fn set_id(&mut self, id: i32) { self.id = id; }
    fn get_id(&self) -> i32 { self.id }
    fn set_x(&mut self, x: f32) { self.x = x; }
    fn get_x(&self) -> f32 { self.x }
    fn set_y(&mut self, y: f32) { self.y = y; }
    fn get_y(&self) -> f32 { self.y }
}