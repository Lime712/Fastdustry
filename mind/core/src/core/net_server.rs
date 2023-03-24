use std::collections::HashSet;
use arc::arc_core::math::geom::vec2::Vec2;
use crate::vars::TILESIZE;
use crate::net::Administration;

/// note that snapshots are compressed, so the max snapshot size here is above the typical UDP safe limit
const MAX_SNAPSHOT_SIZE: usize = 800;
const TIMER_BLOCK_SYNC: usize = 0;
const TIMER_HEALTH_SYNC: usize = 1;
const BLOCK_SYNC_TIME: f32 = 60.0 * 6.0;
const HEALTH_SYNC_TIME: f32 = 30.0;
static mut HIDDEN_IDS: HashSet<i32> = HashSet::new();
static mut HEALTH_HASH_SET: HashSet<i32> = HashSet::new();
static mut VECTOR: Vec2 = Vec2::new(0.0, 0.0);
/// If a player goes away of their server-side coordinates by this distance, they get teleported back.
const CORRECT_DIST: f32 = TILESIZE * 14.0;


pub struct NetServer {
    pub admins: Administration,
    pub client_commands: CommandHandler,
}