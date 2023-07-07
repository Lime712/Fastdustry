use crate::gen::building::Building;
use crate::gen::entityc::Entityc;
use crate::gen::flyingc::Flyingc;
use crate::gen::healthc::Healthc;
use crate::gen::hitboxc::Hitboxc;
use crate::gen::posc::Posc;
use crate::gen::rotc::Rotc;
use crate::gen::statusc::Statusc;
use crate::gen::teamc::Teamc;
use crate::gen::velc::Velc;
/// Interface for {@link mindustry.entities.comp.BuilderComp}
pub trait Builderc : Entityc + Flyingc + Healthc + Hitboxc + Posc + Rotc + Statusc + Teamc + Velc {
    /// # Returns
    /// the build plan currently active, or the one at the top of the queue.
    fn build_plan() -> BuildPlan;

    /// # Returns
    /// whether this plan should be skipped, in favor of the next one.
    fn should_skip(plan: BuildPlan, core: Building) -> bool;

    /// Add another build plans to the queue, if it doesn't exist there yet.
    fn add_build(place: BuildPlan, tail: bool);

    /// Add another build plans to the tail of the queue, if it doesn't exist there yet.
    fn add_build_place(place: BuildPlan);

    /// Clears the placement queue.
    fn clear_building();

    /// Draw all current build plans. Does not draw the beam effect, only the positions.
    fn draw_build_plans();

    /// Return whether this builder's place queue contains items.
    fn is_building() -> bool;

    fn plans() -> Queue<BuildPlan>;

    fn actively_building() -> bool;

    fn can_build() -> bool;

    fn update_building() -> bool;

    fn build_alpha() -> f32;

    fn build_alpha_build_alpha(build_alpha: f32);

    fn draw();

    fn draw_building();

    fn draw_building_beam(px: f32, py: f32);

    fn draw_plan(plan: BuildPlan, alpha: f32);

    fn draw_plan_top(plan: BuildPlan, alpha: f32);

    fn plans_plans(plans: Queue<BuildPlan>);

    fn remove_build(x: i32, y: i32, breaking: bool);

    fn update();

    fn update_build_logic();

    fn update_building_update_building(update_building: bool);

    fn validate_plans();
}
