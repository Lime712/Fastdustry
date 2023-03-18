use crate::gen::damagec::Damagec;
use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::gen::teamc::Teamc;
/// Interface for {@link mindustry.entities.comp.ShielderComp}
pub trait Shielderc : Damagec + Entityc + Posc + Teamc {
    fn absorb();
}
