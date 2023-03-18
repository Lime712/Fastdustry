use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::itemsc::Itemsc;
use crate::gen::posc::Posc;
use crate::gen::rotc::Rotc;
use crate::gen::teamc::Teamc;
use crate::r#type::item::Item;
/// Interface for {@link mindustry.entities.comp.MinerComp}
pub trait Minerc : Drawc + Entityc + Itemsc + Posc + Rotc + Teamc {
    fn mine_tile() -> Option<Tile>;

    fn can_mine() -> bool;

    fn can_mine_item(item: Item) -> bool;

    fn mining() -> bool;

    fn offload_immediately() -> bool;

    fn valid_mine(tile: Tile) -> bool;

    fn valid_mine_tile(tile: Tile, check_dst: bool) -> bool;

    fn mine_timer() -> f32;

    fn get_mine_result(tile: Tile) -> Item;

    fn draw();

    fn mine_tile_mine_tile(mine_tile: Option<Tile>);

    fn mine_timer_mine_timer(mine_timer: f32);

    fn update();
}
