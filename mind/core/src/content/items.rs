use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::r#type::item::Item;

pub const COPPER: Item = {
    let mut t = Item::new("copper");
    // copper
    t.hardness = 1;
    t.cost = 0.5;
    t.super_struct.always_unlocked = true;
    t
};
pub const LEAD: Item = {
    let mut t = Item::new("lead");
    // lead
    t.hardness = 2;
    t.cost = 0.6;
    t.super_struct.always_unlocked = true;
    t
};
pub const GRAPHITE: Item = {
    let mut t = Item::new("graphite");
    // graphite
    t.cost = 0.6;
    t.super_struct.always_unlocked = true;
    t
};
pub const COAL: Item = {
    let mut t = Item::new("coal");
    // coal
    t.flammability = 0.3;
    t.explosiveness = 0.1;
    t.super_struct.always_unlocked = true;
    t
};
pub const TITANIUM: Item = {
    let mut t = Item::new("titanium");
    // titanium
    t.hardness = 3;
    t.cost = 0.8;
    t.super_struct.always_unlocked = true;
    t
};
pub const THORIUM: Item = {
    let mut t = Item::new("thorium");
    // thorium
    t.radioactivity = 0.4;
    t.cost = 0.8;
    t.super_struct.always_unlocked = true;
    t
};
pub const SILICON: Item = {
    let mut t = Item::new("silicon");
    // silicon
    t.cost = 0.5;
    t.super_struct.always_unlocked = true;
    t
};

pub const PLASTANIUM: Item = {
    let mut t = Item::new("plastanium");
    // plastanium
    t.flammability = 0.1;
    t.explosiveness = 0.2;
    t.cost = 1.3;
    t.health_scaling = 0.1;
    t.super_struct.always_unlocked = true;
    t
};
pub const PHASE_FABRIC: Item = {
    let mut t = Item::new("phase-fabric");
    // phase fabric
    t.cost = 1.3;
    t.radioactivity = 0.6;
    t.health_scaling = 0.25;
    t.super_struct.always_unlocked = true;
    t
};
pub const SURGE_ALLOY: Item = {
    let mut t = Item::new("surge-alloy");
    // surge alloy
    t.cost = 1.2;
    t.charge = 0.75;
    t.health_scaling = 0.25;
    t
};
pub const SPORE_POD: Item = {
    let mut t = Item::new("spore-pod");
    // spore pod
    t.flammability = 0.3;
    t.explosiveness = 0.2;
    t.health_scaling = 0.25;
    t.super_struct.always_unlocked = true;
    t
};
pub const SAND: Item = {
    let mut t = Item::new("sand");
    // sand
    t.cost = 0.4;
    t.super_struct.always_unlocked = true;
    t
};
pub const BLAST_COMPOUND: Item = {
    let mut t = Item::new("blast-compound");
    // blast compound
    t.flammability = 0.3;
    t.explosiveness = 0.5;
    t.super_struct.always_unlocked = true;
    t
};
pub const PYRATITE: Item = {
    let mut t = Item::new("pyratite");
    // pyratite
    t.flammability = 0.6;
    t.explosiveness = 0.3;
    t.radioactivity = 0.1;
    t.cost = 0.7;
    t.super_struct.always_unlocked = true;
    t
};
pub const METAGLASS: Item = {
    let mut t = Item::new("metaglass");
    // metaglass
    t.health_scaling = 0.25;
    t.super_struct.always_unlocked = true;
    t
};
pub const BERYLLIUM: Item = {
    let mut t = Item::new("beryllium");
    // beryllium
    t.hardness = 2;
    t.cost = 1.2;
    t.super_struct.always_unlocked = true;
    t
};
pub const TUNGSTEN: Item = {
    let mut t = Item::new("tungsten");
    // tungsten
    t.hardness = 4;
    t.cost = 1.1;
    t.super_struct.always_unlocked = true;
    t
};
pub const OXIDE: Item = {
    let mut t = Item::new("oxide");
    // oxide
    t.health_scaling = 0.25;
    t.super_struct.always_unlocked = true;
    t
};
pub const CARBIDE: Item = {
    let mut t = Item::new("carbide");
    // carbide
    t.health_scaling = 0.25;
    t.super_struct.always_unlocked = true;
    t
};
pub const FISSILE_MATTER: Item = {
    let mut t = Item::new("fissile-matter");
    // fissile matter
    t.cost = 1.2;
    t.charge = 0.5;
    t.health_scaling = 0.25;
    t.super_struct.always_unlocked = true;
    t
};
pub const DORMANT_CYST: Item = {
    let mut t = Item::new("dormant-cyst");
    // dormant cyst
    t.cost = 1.2;
    t.health_scaling = 0.25;
    t.super_struct.always_unlocked = true;
    t
};


pub const SERPULO_ITEMS: Vec<&'static Item> = {
    let mut items = Vec::new();
    // add:
    // scrap, copper, lead, graphite, coal, titanium, thorium, silicon, plastanium,
    // phaseFabric, surgeAlloy, sporePod, sand, blastCompound, pyratite, metaglass
    items.push(&COPPER);
    items.push(&LEAD);
    items.push(&GRAPHITE);
    items.push(&COAL);
    items.push(&TITANIUM);
    items.push(&THORIUM);
    items.push(&SILICON);
    items.push(&PLASTANIUM);
    items.push(&PHASE_FABRIC);
    items.push(&SURGE_ALLOY);
    items.push(&SPORE_POD);
    items.push(&SAND);
    items.push(&BLAST_COMPOUND);
    items.push(&PYRATITE);
    items.push(&METAGLASS);
    items
};
pub const EREKIR_ITEMS: Vec<&'static Item> = {
    let mut items = Vec::new();
    // add:
    // graphite, thorium, silicon, phaseFabric, surgeAlloy, sand,
    // beryllium, tungsten, oxide, carbide, fissileMatter, dormantCyst
    // to erekir items
    items.push(&GRAPHITE);
    items.push(&THORIUM);
    items.push(&SILICON);
    items.push(&PHASE_FABRIC);
    items.push(&SURGE_ALLOY);
    items.push(&SAND);
    items.push(&BERYLLIUM);
    items.push(&TUNGSTEN);
    items.push(&OXIDE);
    items.push(&CARBIDE);
    items.push(&FISSILE_MATTER);
    items.push(&DORMANT_CYST);
    items
};
pub const EREKIR_ONLY_ITEMS: Vec<&'static Item> = {
    let mut items = Vec::new();
    // add all erekir items and remove all serpulo items to the erekirOnlyItems
    let mut i = 0;
    while i < EREKIR_ITEMS.len() {
        items.push(EREKIR_ITEMS[i]);
        i += 1;
    }
    let mut i = 0;
    while i < SERPULO_ITEMS.len() {
        if let Some(index) = items
            .iter()
            .position(|x| x.super_struct.localized_name == item.super_struct.localized_name)
        {
            items.remove(index);
        }
    }
    items
};




pub struct Items;