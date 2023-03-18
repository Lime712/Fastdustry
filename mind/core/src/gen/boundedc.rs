use crate::gen::entityc::Entityc;
use crate::gen::flyingc::Flyingc;
use crate::gen::healthc::Healthc;
use crate::gen::hitboxc::Hitboxc;
use crate::gen::posc::Posc;
use crate::gen::velc::Velc;
/// Interface for {@link mindustry.entities.comp.BoundedComp}
pub trait Boundedc : Entityc + Flyingc + Healthc + Hitboxc + Posc + Velc {
    fn update();
}
