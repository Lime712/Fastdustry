use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::gen::syncc::Syncc;
use std::u8;
/// Interface for {@link mindustry.entities.comp.WorldLabelComp}
pub trait WorldLabelc : Drawc + Entityc + Posc + Syncc {
    /// Flags are packed into a byte for sync efficiency; see the flag static values.
    fn flags() -> u8;

    /// Flags are packed into a byte for sync efficiency; see the flag static values.
    fn flags_flags(flags: u8);

    /// This MUST be called instead of remove()!
    fn hide();

    fn clip_size() -> f32;

    fn font_size() -> f32;

    fn z() -> f32;

    fn text() -> String;

    fn draw();

    fn font_size_font_size(font_size: f32);

    fn text_text(text: String);

    fn z_z(z: f32);
}
