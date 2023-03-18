use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::gen::syncc::Syncc;
use crate::gen::timedc::Timedc;
/// Interface for {@link mindustry.entities.comp.FireComp}
pub trait Firec : Drawc + Entityc + Posc + Syncc + Timedc {
    fn clip_size() -> f32;

    fn tile() -> Tile;

    fn after_read();

    fn after_sync();

    fn draw();

    fn remove();

    fn tile_tile(tile: Tile);

    fn update();
}
