use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
/// Interface for {@link mindustry.entities.comp.TeamComp}
pub trait Teamc : Entityc + Posc {
    /// # Returns
    /// whether the center of this entity is visible to the viewing team.
    fn in_fog_to(viewer: Team) -> bool;

    fn cheating() -> bool;

    fn team() -> Team;

    fn closest_core() -> CoreBlock.CoreBuild;

    fn closest_enemy_core() -> CoreBlock.CoreBuild;

    fn core() -> CoreBlock.CoreBuild;

    fn team_team(team: Team);
}
