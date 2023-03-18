use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::gen::syncc::Syncc;
use arc::arc_core::math::geom::vec2::Vec2;
/// Interface for {@link mindustry.type.Weather.WeatherStateComp}
pub trait WeatherStatec : Drawc + Entityc + Posc + Syncc {
    fn wind_vector() -> Vec2;

    fn effect_timer() -> f32;

    fn intensity() -> f32;

    fn life() -> f32;

    fn opacity() -> f32;

    fn weather() -> Weather;

    fn draw();

    fn effect_timer_effect_timer(effect_timer: f32);

    fn init(weather: Weather);

    fn intensity_intensity(intensity: f32);

    fn life_life(life: f32);

    fn opacity_opacity(opacity: f32);

    fn update();

    fn weather_weather(weather: Weather);

    fn wind_vector_wind_vector(wind_vector: Vec2);
}
