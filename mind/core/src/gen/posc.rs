use arc::arc_core::math::geom::position::Position;
use crate::gen::building::Building;
use crate::gen::position::Position;
use crate::gen::entityc::Entityc;
use crate::world::block::Block;

/// Interface for {@link mindustry.entities.comp.PosComp}
pub trait Posc : Position + Entityc {
    /// Returns air if this unit is on a non-air top block.
    fn floor_on() -> Floor;

    fn build_on() -> Building;

    fn on_solid() -> bool;

    fn get_x() -> f32;

    fn get_y() -> f32;

    fn x() -> f32;

    fn y() -> f32;

    fn tile_x() -> i32;

    fn tile_y() -> i32;

    fn block_on() -> Block;

    fn tile_on() -> Tile;

    fn set(pos: dyn Position);

    fn set_x(x: f32, y: f32);

    fn trns(pos: dyn Position);

    fn trns_x(x: f32, y: f32);

    fn x_x(x: f32);

    fn y_y(y: f32);
}
