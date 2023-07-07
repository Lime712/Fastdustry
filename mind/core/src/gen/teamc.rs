use crate::game::team::Team;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::world::blocks::storage::core_block::CoreBuild;

/// Interface for {@link mindustry.entities.comp.TeamComp}
pub trait Teamc : Entityc + Posc {
    /// # Returns
    /// whether the center of this entity is visible to the viewing team.
    fn in_fog_to(viewer: Team) -> bool;

    fn cheating() -> bool;

    fn team() -> Team;

    fn closest_core() -> CoreBuild;

    fn closest_enemy_core() -> CoreBuild;

    fn core() -> CoreBuild;

    fn team_team(team: Team);
}
