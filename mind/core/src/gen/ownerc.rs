use crate::gen::entityc::Entityc;
/// Interface for {@link mindustry.entities.comp.OwnerComp}
pub trait Ownerc : Entityc {
    fn owner() -> Entityc;

    fn owner_owner(owner: Entityc);
}
