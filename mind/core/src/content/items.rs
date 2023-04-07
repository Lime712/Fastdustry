use std::sync::{Mutex};

use lazy_static::lazy_static;

use crate::r#type::item::Item;
// todo: make this const, so that it can be used in const functions

pub static mut SERPULO_ITEMS: Vec<Item> = Vec::new();
pub static mut EREKIR_ITEMS: Vec<Item> = Vec::new();
pub static mut EREKIR_ONLY_ITEMS: Vec<Item> = Vec::new();

lazy_static! {
    pub static ref SCRAP: Mutex<Item> = Mutex::new(Item::new("scrap"));
    pub static ref COPPER: Mutex<Item> = Mutex::new(Item::new("copper"));
    pub static ref LEAD: Mutex<Item> = Mutex::new(Item::new("lead"));
    pub static ref GRAPHITE: Mutex<Item> = Mutex::new(Item::new("graphite"));
    pub static ref COAL: Mutex<Item> = Mutex::new(Item::new("coal"));
    pub static ref TITANIUM: Mutex<Item> = Mutex::new(Item::new("titanium"));
    pub static ref THORIUM: Mutex<Item> = Mutex::new(Item::new("thorium"));
    pub static ref SILICON: Mutex<Item> = Mutex::new(Item::new("silicon"));
    pub static ref PLASTANIUM: Mutex<Item> = Mutex::new(Item::new("plastanium"));
    pub static ref PHASE_FABRIC: Mutex<Item> = Mutex::new(Item::new("phase-fabric"));
    pub static ref SURGE_ALLOY: Mutex<Item> = Mutex::new(Item::new("surge-alloy"));
    pub static ref SPORE_POD: Mutex<Item> = Mutex::new(Item::new("spore-pod"));
    pub static ref SAND: Mutex<Item> = Mutex::new(Item::new("sand"));
    pub static ref BLAST_COMPOUND: Mutex<Item> = Mutex::new(Item::new("blast-compound"));
    pub static ref PYRATITE: Mutex<Item> = Mutex::new(Item::new("pyratite"));
    pub static ref METAGLASS: Mutex<Item> = Mutex::new(Item::new("metaglass"));
    pub static ref BERYLLIUM: Mutex<Item> = Mutex::new(Item::new("beryllium"));
    pub static ref TUNGSTEN: Mutex<Item> = Mutex::new(Item::new("tungsten"));
    pub static ref OXIDE: Mutex<Item> = Mutex::new(Item::new("oxide"));
    pub static ref CARBIDE: Mutex<Item> = Mutex::new(Item::new("carbide"));
    pub static ref FISSILE_MATTER: Mutex<Item> = Mutex::new(Item::new("fissile-matter"));
    pub static ref DORMANT_CYST: Mutex<Item> = Mutex::new(Item::new("dormant-cyst"));
}

pub struct Items;

impl Items {
    pub fn load() {
        // copper
        COPPER.lock().unwrap().hardness = 1;
        COPPER.lock().unwrap().cost = 0.5;
        COPPER.lock().unwrap().super_struct.always_unlocked = true;

        // lead
        LEAD.lock().unwrap().hardness = 1;
        LEAD.lock().unwrap().cost = 0.6;
        LEAD.lock().unwrap().super_struct.always_unlocked = true;

        // graphite
        GRAPHITE.lock().unwrap().cost = 0.6;
        GRAPHITE.lock().unwrap().super_struct.always_unlocked = true;

        // coal
        COAL.lock().unwrap().flammability = 0.3;
        COAL.lock().unwrap().explosiveness = 0.1;
        COAL.lock().unwrap().super_struct.always_unlocked = true;

        // titanium
        TITANIUM.lock().unwrap().hardness = 3;
        TITANIUM.lock().unwrap().cost = 0.8;
        TITANIUM.lock().unwrap().super_struct.always_unlocked = true;

        // thorium
        THORIUM.lock().unwrap().radioactivity = 0.4;
        THORIUM.lock().unwrap().cost = 0.8;
        THORIUM.lock().unwrap().super_struct.always_unlocked = true;

        // silicon
        SILICON.lock().unwrap().cost = 0.5;
        SILICON.lock().unwrap().super_struct.always_unlocked = true;

        // plastanium
        PLASTANIUM.lock().unwrap().flammability = 0.1;
        PLASTANIUM.lock().unwrap().explosiveness = 0.2;
        PLASTANIUM.lock().unwrap().cost = 1.3;
        PLASTANIUM.lock().unwrap().health_scaling = 0.1;
        PLASTANIUM.lock().unwrap().super_struct.always_unlocked = true;

        // phase fabric
        PHASE_FABRIC.lock().unwrap().cost = 1.3;
        PHASE_FABRIC.lock().unwrap().radioactivity = 0.6;
        PHASE_FABRIC.lock().unwrap().health_scaling = 0.25;
        PHASE_FABRIC.lock().unwrap().super_struct.always_unlocked = true;

        // surge alloy
        SURGE_ALLOY.lock().unwrap().cost = 1.2;
        SURGE_ALLOY.lock().unwrap().charge = 0.75;
        SURGE_ALLOY.lock().unwrap().health_scaling = 0.25;

        // spore pod
        SPORE_POD.lock().unwrap().flammability = 0.3;
        SPORE_POD.lock().unwrap().explosiveness = 0.2;
        SPORE_POD.lock().unwrap().health_scaling = 0.25;
        SPORE_POD.lock().unwrap().super_struct.always_unlocked = true;

        // sand
        SAND.lock().unwrap().cost = 0.4;
        SAND.lock().unwrap().super_struct.always_unlocked = true;

        // blast compound
        BLAST_COMPOUND.lock().unwrap().flammability = 0.3;
        BLAST_COMPOUND.lock().unwrap().explosiveness = 0.5;
        BLAST_COMPOUND.lock().unwrap().super_struct.always_unlocked = true;

        // pyratite
        PYRATITE.lock().unwrap().flammability = 0.6;
        PYRATITE.lock().unwrap().explosiveness = 0.3;
        PYRATITE.lock().unwrap().radioactivity = 0.1;
        PYRATITE.lock().unwrap().cost = 0.7;
        PYRATITE.lock().unwrap().super_struct.always_unlocked = true;

        // metaglass
        METAGLASS.lock().unwrap().health_scaling = 0.25;
        METAGLASS.lock().unwrap().super_struct.always_unlocked = true;

        // beryllium
        BERYLLIUM.lock().unwrap().hardness = 2;
        BERYLLIUM.lock().unwrap().cost = 1.2;
        BERYLLIUM.lock().unwrap().super_struct.always_unlocked = true;

        // tungsten
        TUNGSTEN.lock().unwrap().hardness = 4;
        TUNGSTEN.lock().unwrap().cost = 1.1;
        TUNGSTEN.lock().unwrap().super_struct.always_unlocked = true;

        // oxide
        OXIDE.lock().unwrap().health_scaling = 0.25;
        OXIDE.lock().unwrap().super_struct.always_unlocked = true;

        // carbide
        CARBIDE.lock().unwrap().health_scaling = 0.25;
        CARBIDE.lock().unwrap().super_struct.always_unlocked = true;

        // fissile matter
        FISSILE_MATTER.lock().unwrap().cost = 1.2;
        FISSILE_MATTER.lock().unwrap().charge = 0.5;
        FISSILE_MATTER.lock().unwrap().health_scaling = 0.25;
        FISSILE_MATTER.lock().unwrap().super_struct.always_unlocked = true;

        // dormant cyst
        DORMANT_CYST.lock().unwrap().cost = 1.2;
        DORMANT_CYST.lock().unwrap().health_scaling = 0.25;
        DORMANT_CYST.lock().unwrap().super_struct.always_unlocked = true;

        unsafe {
            // add:
            // scrap, copper, lead, graphite, coal, titanium, thorium, silicon, plastanium,
            // phaseFabric, surgeAlloy, sporePod, sand, blastCompound, pyratite, metaglass
            SERPULO_ITEMS.push(COPPER.lock().unwrap().clone());
            SERPULO_ITEMS.push(LEAD.lock().unwrap().clone());
            SERPULO_ITEMS.push(GRAPHITE.lock().unwrap().clone());
            SERPULO_ITEMS.push(COAL.lock().unwrap().clone());
            SERPULO_ITEMS.push(TITANIUM.lock().unwrap().clone());
            SERPULO_ITEMS.push(THORIUM.lock().unwrap().clone());
            SERPULO_ITEMS.push(SILICON.lock().unwrap().clone());
            SERPULO_ITEMS.push(PLASTANIUM.lock().unwrap().clone());
            SERPULO_ITEMS.push(PHASE_FABRIC.lock().unwrap().clone());
            SERPULO_ITEMS.push(SURGE_ALLOY.lock().unwrap().clone());
            SERPULO_ITEMS.push(SPORE_POD.lock().unwrap().clone());
            SERPULO_ITEMS.push(SAND.lock().unwrap().clone());
            SERPULO_ITEMS.push(BLAST_COMPOUND.lock().unwrap().clone());
            SERPULO_ITEMS.push(PYRATITE.lock().unwrap().clone());
            SERPULO_ITEMS.push(METAGLASS.lock().unwrap().clone());

            // add:
            // graphite, thorium, silicon, phaseFabric, surgeAlloy, sand,
            // beryllium, tungsten, oxide, carbide, fissileMatter, dormantCyst
            // to erekir items
            SERPULO_ITEMS.push(GRAPHITE.lock().unwrap().clone());
            SERPULO_ITEMS.push(THORIUM.lock().unwrap().clone());
            SERPULO_ITEMS.push(SILICON.lock().unwrap().clone());
            SERPULO_ITEMS.push(PHASE_FABRIC.lock().unwrap().clone());
            SERPULO_ITEMS.push(SURGE_ALLOY.lock().unwrap().clone());
            SERPULO_ITEMS.push(SAND.lock().unwrap().clone());
            SERPULO_ITEMS.push(BERYLLIUM.lock().unwrap().clone());
            SERPULO_ITEMS.push(TUNGSTEN.lock().unwrap().clone());
            SERPULO_ITEMS.push(OXIDE.lock().unwrap().clone());
            SERPULO_ITEMS.push(CARBIDE.lock().unwrap().clone());
            SERPULO_ITEMS.push(FISSILE_MATTER.lock().unwrap().clone());
            SERPULO_ITEMS.push(DORMANT_CYST.lock().unwrap().clone());

            // add all erekir items and remove all serpulo items to the erekirOnlyItems
            for item in &*EREKIR_ITEMS {
                EREKIR_ONLY_ITEMS.push(item.clone());
            }
            for item in &*SERPULO_ITEMS {
                if let Some(index) = EREKIR_ONLY_ITEMS
                    .iter()
                    .position(|x| x.super_struct.localized_name == item.super_struct.localized_name)
                {
                    EREKIR_ONLY_ITEMS.remove(index);
                }
            }
        }
    }
}
