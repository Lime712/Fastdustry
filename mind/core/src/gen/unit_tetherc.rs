use crate::gen::boundedc::Boundedc;
use crate::gen::builderc::Builderc;
use crate::gen::drawc::Drawc;
use crate::gen::entityc::Entityc;
use crate::gen::flyingc::Flyingc;
use crate::gen::healthc::Healthc;
use crate::gen::hitboxc::Hitboxc;
use crate::gen::itemsc::Itemsc;
use crate::gen::minerc::Minerc;
use crate::gen::physicsc::Physicsc;
use crate::gen::posc::Posc;
use crate::gen::rotc::Rotc;
use crate::gen::shieldc::Shieldc;
use crate::gen::statusc::Statusc;
use crate::gen::syncc::Syncc;
use crate::gen::teamc::Teamc;
use crate::gen::unitc::Unitc;
use crate::gen::velc::Velc;
use crate::gen::weaponsc::Weaponsc;
/// Interface for {@link mindustry.entities.comp.UnitTetherComp}
pub trait UnitTetherc : Boundedc + Builderc + Drawc + Entityc + Flyingc + Healthc + Hitboxc + Itemsc + Minerc + Physicsc + Posc + Rotc + Shieldc + Statusc + Syncc + Teamc + Unitc + Velc + Weaponsc {
    fn spawner() -> Option<Unit>;

    fn spawner_unit_id() -> i32;

    fn after_read();

    fn after_sync();

    fn spawner_spawner(spawner: Option<Unit>);

    fn spawner_unit_id_spawner_unit_id(spawner_unit_id: i32);

    fn update();
}
