use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::gen::syncc::Syncc;
use crate::r#type::liquid::Liquid;
/// Interface for {@link mindustry.entities.comp.PuddleComp}
pub trait Puddlec : Drawc + Entityc + Posc + Syncc {
    fn accepting() -> f32;

    fn amount() -> f32;

    fn clip_size() -> f32;

    fn effect_time() -> f32;

    fn get_flammability() -> f32;

    fn last_ripple() -> f32;

    fn update_time() -> f32;

    fn liquid() -> Liquid;

    fn tile() -> Tile;

    fn accepting_accepting(accepting: f32);

    fn after_read();

    fn after_sync();

    fn amount_amount(amount: f32);

    fn draw();

    fn effect_time_effect_time(effect_time: f32);

    fn last_ripple_last_ripple(last_ripple: f32);

    fn liquid_liquid(liquid: Liquid);

    fn remove();

    fn tile_tile(tile: Tile);

    fn update();

    fn update_time_update_time(update_time: f32);
}
