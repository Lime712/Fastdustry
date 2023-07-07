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
/// Interface for {@link mindustry.entities.comp.CrawlComp}
pub trait Crawlc : Boundedc + Builderc + Drawc + Entityc + Flyingc + Healthc + Hitboxc + Itemsc + Minerc + Physicsc + Posc + Rotc + Shieldc + Statusc + Syncc + Teamc + Unitc + Velc + Weaponsc {
    fn crawl_time() -> f32;

    fn floor_speed_multiplier() -> f32;

    fn last_crawl_slowdown() -> f32;

    fn segment_rot() -> f32;

    fn path_type() -> i32;

    fn solidity() -> SolidPred;

    fn drown_floor() -> Floor;

    fn last_deep_floor() -> Floor;

    fn add();

    fn crawl_time_crawl_time(crawl_time: f32);

    fn last_crawl_slowdown_last_crawl_slowdown(last_crawl_slowdown: f32);

    fn last_deep_floor_last_deep_floor(last_deep_floor: Floor);

    fn segment_rot_segment_rot(segment_rot: f32);

    fn update();
}
