use crate::gen::unit_controller::UnitController;
use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::gen::syncc::Syncc;
use crate::gen::timerc::Timerc;
/// Interface for {@link mindustry.entities.comp.PlayerComp}
pub trait Playerc : UnitController + Drawc + Entityc + Posc + Syncc + Timerc {
    /// # Returns
    /// largest/closest core, with largest cores getting priority
    fn best_core() -> CoreBlock.CoreBuild;

    /// # Returns
    /// name with a markup color prefix
    fn colored_name() -> String;

    fn just_switch_from() -> Option<Unit>;

    fn just_switch_to() -> Option<Unit>;

    fn con() -> Option<NetConnection>;

    fn unit() -> Unit;

    fn color() -> Color;

    fn icon() -> TextureRegion;

    fn admin() -> bool;

    fn boosting() -> bool;

    fn dead() -> bool;

    fn display_ammo() -> bool;

    fn is_builder() -> bool;

    fn is_valid_controller() -> bool;

    fn shooting() -> bool;

    fn typing() -> bool;

    fn clip_size() -> f32;

    fn death_timer() -> f32;

    fn mouse_x() -> f32;

    fn mouse_y() -> f32;

    fn text_fade_time() -> f32;

    fn ip() -> String;

    fn last_text() -> String;

    fn locale() -> String;

    fn name() -> String;

    fn plain_name() -> String;

    fn usid() -> String;

    fn uuid() -> String;

    fn team() -> Team;

    fn get_info() -> Administration.PlayerInfo;

    fn closest_core() -> CoreBlock.CoreBuild;

    fn core() -> CoreBlock.CoreBuild;

    fn admin_admin(admin: bool);

    fn after_sync();

    fn boosting_boosting(boosting: bool);

    fn check_spawn();

    fn clear_unit();

    fn color_color(color: Color);

    fn con_con(con: Option<NetConnection>);

    fn death_timer_death_timer(death_timer: f32);

    fn draw();

    fn just_switch_from_just_switch_from(just_switch_from: Option<Unit>);

    fn just_switch_to_just_switch_to(just_switch_to: Option<Unit>);

    fn kick(reason: String);

    fn kick_reason(reason: String, duration: i64);

    fn kick_reason(reason: Packets.KickReason);

    fn kick_reason_duration(reason: Packets.KickReason, duration: i64);

    fn last_text_last_text(last_text: String);

    fn locale_locale(locale: String);

    fn mouse_x_mouse_x(mouse_x: f32);

    fn mouse_y_mouse_y(mouse_y: f32);

    fn name_name(name: String);

    fn remove();

    fn reset();

    fn send_message(text: String);

    fn send_message_text(text: String, from: Player);

    fn send_message_text_from(text: String, from: Player, unformatted: String);

    fn shooting_shooting(shooting: bool);

    fn team_team(team: Team);

    fn text_fade_time_text_fade_time(text_fade_time: f32);

    fn typing_typing(typing: bool);

    fn unit_unit(unit: Unit);

    fn update();
}
