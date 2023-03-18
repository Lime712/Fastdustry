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
use std::collections::HashSet;
/// Interface for {@link mindustry.entities.comp.PayloadComp}
pub trait Payloadc : Boundedc + Builderc + Drawc + Entityc + Flyingc + Healthc + Hitboxc + Itemsc + Minerc + Physicsc + Posc + Rotc + Shieldc + Statusc + Syncc + Teamc + Unitc + Velc + Weaponsc {
    /// # Returns
    /// whether the tile has been successfully placed.
    fn drop_block(payload: BuildPayload) -> bool;

    fn payloads() -> HashSet<Payload>;

    fn can_pickup(build: Building) -> bool;

    fn can_pickup_unit(unit: Unit) -> bool;

    fn can_pickup_payload(pay: Payload) -> bool;

    fn drop_last_payload() -> bool;

    fn drop_unit(payload: UnitPayload) -> bool;

    fn has_payload() -> bool;

    fn try_drop_payload(payload: Payload) -> bool;

    fn payload_used() -> f32;

    fn add_payload(load: Payload);

    fn content_info(table: Table, item_size: f32, width: f32);

    fn payloads_payloads(payloads: HashSet<Payload>);

    fn pickup(tile: Building);

    fn pickup_unit(unit: Unit);

    fn update();
}
