use crate::r#type::cell_liquid::CellLiquid;
use crate::r#type::liquid::Liquid;

pub const WATER: Liquid = {
    let mut t = Liquid::new("water");
    // water
    t.heat_capacity = 0.4;
    t.boil_point = 0.5;
    t.super_struct.always_unlocked = true;
    t
};
pub const SLAG: Liquid = {
    let mut t = Liquid::new("slag");
    // slag
    t.temperature = 1.0;
    t.viscosity = 0.7;
    t
};
pub const OIL: Liquid = {
    let mut t = Liquid::new("oil");
    // oil
    t.viscosity = 0.75;
    t.flammability = 1.2;
    t.explosiveness = 1.2;
    t.heat_capacity = 0.7;
    t.boil_point = 0.65;
    t.can_stay_on.insert(&WATER);
    t
};
pub const CRYOFLUID: Liquid = {
    let mut t = Liquid::new("cryofluid");
    // cryofluid
    t.heat_capacity = 0.9;
    t.temperature = 0.25;
    t.boil_point = 0.55;
    t
};
pub const NEOPLASM: CellLiquid = {
    let mut t = CellLiquid::new("neoplasm");
    // neoplasm
    t.liquid.heat_capacity = 0.4;
    t.liquid.temperature = 0.54;
    t.liquid.viscosity = 0.85;
    t.liquid.flammability = 0.0;
    t.liquid.cap_puddles = false;
    t.spread_target = Some(WATER.clone());
    t.liquid.move_through_blocks = true;
    t.liquid.incinerable = true;
    t.liquid.block_reactive = false;
    t.liquid.can_stay_on.insert(&WATER);
    t.liquid.can_stay_on.insert(&OIL);
    t.liquid.can_stay_on.insert(&CRYOFLUID);
    t
};
pub const ARKYCITE: Liquid = {
    let mut t = Liquid::new("arkycite");
    // arkycite
    t.flammability = 0.4;
    t.viscosity = 0.7;
    t
};
pub const GALLIUM: Liquid = {
    let mut t = Liquid::new("gallium");
    // gallium
    t.coolant = false;
    t.hidden = true;
    t
};
pub const OZONE: Liquid = {
    let mut t = Liquid::new("ozone");
    // ozone
    t.gas = true;
    t.flammability = 1.0;
    t.explosiveness = 1.0;
    t
};
pub const HYDROGEN: Liquid = {
    let mut t = Liquid::new("hydrogen");
    // hydrogen
    t.gas = true;
    t.flammability = 1.0;
    t
};
pub const NITROGEN: Liquid = {
    let mut t = Liquid::new("nitrogen");
    // nitrogen
    t.gas = true;
    t
};
pub const CYANOGEN: Liquid = {
    let mut t = Liquid::new("cyanogen");
    // cyanogen
    t.gas = true;
    t.flammability = 2.0;
    t
};