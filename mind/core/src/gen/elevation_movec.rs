use crate::gen::entityc::Entityc;
use crate::gen::flyingc::Flyingc;
use crate::gen::healthc::Healthc;
use crate::gen::hitboxc::Hitboxc;
use crate::gen::posc::Posc;
use crate::gen::velc::Velc;
/// Interface for {@link mindustry.entities.comp.ElevationMoveComp}
pub trait ElevationMovec : Entityc + Flyingc + Healthc + Hitboxc + Posc + Velc {
    fn solidity() -> SolidPred;
}
