use lazy_static::lazy_static;
use crate::r#type::liquid::Liquid;
use std::sync::Mutex;
use crate::r#type::cell_liquid::CellLiquid;

// original code
/*package mindustry.content;

import arc.graphics.*;
import mindustry.type.*;

public class Liquids{
    public static Liquid water, slag, oil, cryofluid,
    arkycite, gallium, neoplasm,
    ozone, hydrogen, nitrogen, cyanogen;

    public static void load(){

        water = new Liquid("water", Color.valueOf("596ab8")){{
            heatCapacity = 0.4f;
            effect = StatusEffects.wet;
            boilPoint = 0.5f;
            gasColor = Color.grays(0.9f);
            alwaysUnlocked = true;
        }};

        slag = new Liquid("slag", Color.valueOf("ffa166")){{
            temperature = 1f;
            viscosity = 0.7f;
            effect = StatusEffects.melting;
            lightColor = Color.valueOf("f0511d").a(0.4f);
        }};

        oil = new Liquid("oil", Color.valueOf("313131")){{
            viscosity = 0.75f;
            flammability = 1.2f;
            explosiveness = 1.2f;
            heatCapacity = 0.7f;
            barColor = Color.valueOf("6b675f");
            effect = StatusEffects.tarred;
            boilPoint = 0.65f;
            gasColor = Color.grays(0.4f);
            canStayOn.add(water);
        }};

        cryofluid = new Liquid("cryofluid", Color.valueOf("6ecdec")){{
            heatCapacity = 0.9f;
            temperature = 0.25f;
            effect = StatusEffects.freezing;
            lightColor = Color.valueOf("0097f5").a(0.2f);
            boilPoint = 0.55f;
            gasColor = Color.valueOf("c1e8f5");
        }};

        neoplasm = new CellLiquid("neoplasm", Color.valueOf("c33e2b")){{
            heatCapacity = 0.4f;
            temperature = 0.54f;
            viscosity = 0.85f;
            flammability = 0f;
            capPuddles = false;
            spreadTarget = Liquids.water;
            moveThroughBlocks = true;
            incinerable = true;
            blockReactive = false;
            canStayOn.addAll(water, oil, cryofluid);

            colorFrom = Color.valueOf("e8803f");
            colorTo = Color.valueOf("8c1225");
        }};

        arkycite = new Liquid("arkycite", Color.valueOf("84a94b")){{
            flammability = 0.4f;
            viscosity = 0.7f;
            neoplasm.canStayOn.add(this);
        }};

        gallium = new Liquid("gallium", Color.valueOf("9a9dbf")){{
            coolant = false;
            hidden = true;
        }};

        ozone = new Liquid("ozone", Color.valueOf("fc81dd")){{
            gas = true;
            barColor = Color.valueOf("d699f0");
            explosiveness = 1f;
            flammability = 1f;
        }};

        hydrogen = new Liquid("hydrogen", Color.valueOf("9eabf7")){{
            gas = true;
            flammability = 1f;
        }};

        nitrogen = new Liquid("nitrogen", Color.valueOf("efe3ff")){{
            gas = true;
        }};

        cyanogen = new Liquid("cyanogen", Color.valueOf("89e8b6")){{
            gas = true;
            flammability = 2f;
        }};
    }
}

 */


lazy_static! {
    pub static ref WATER: Mutex<Liquid> = Mutex::new(Liquid::new("water"));
    pub static ref SLAG: Mutex<Liquid> = Mutex::new(Liquid::new("slag"));
    pub static ref OIL: Mutex<Liquid> = Mutex::new(Liquid::new("oil"));
    pub static ref CRYOFLUID: Mutex<Liquid> = Mutex::new(Liquid::new("cryofluid"));
    pub static ref ARKYCITE: Mutex<Liquid> = Mutex::new(Liquid::new("arkycite"));
    pub static ref GALLIUM: Mutex<Liquid> = Mutex::new(Liquid::new("gallium"));
    pub static ref NEOPLASM: Mutex<CellLiquid> = Mutex::new(CellLiquid::new("neoplasm"));
    pub static ref OZONE: Mutex<Liquid> = Mutex::new(Liquid::new("ozone"));
    pub static ref HYDROGEN: Mutex<Liquid> = Mutex::new(Liquid::new("hydrogen"));
    pub static ref NITROGEN: Mutex<Liquid> = Mutex::new(Liquid::new("nitrogen"));
    pub static ref CYANOGEN: Mutex<Liquid> = Mutex::new(Liquid::new("cyanogen"));
}

pub struct Liquids;

impl Liquids {
    pub fn load() {
        // water
        WATER.lock().unwrap().heat_capacity = 0.4;
        WATER.lock().unwrap().boil_point = 0.5;
        WATER.lock().unwrap().super_struct.always_unlocked = true;

        // slag
        SLAG.lock().unwrap().temperature = 1.0;
        SLAG.lock().unwrap().viscosity = 0.7;

        // oil
        OIL.lock().unwrap().viscosity = 0.75;
        OIL.lock().unwrap().flammability = 1.2;
        OIL.lock().unwrap().explosiveness = 1.2;
        OIL.lock().unwrap().heat_capacity = 0.7;
        OIL.lock().unwrap().boil_point = 0.65;
        OIL.lock().unwrap().can_stay_on.insert(WATER.lock().unwrap().clone());

        // cryofluid
        CRYOFLUID.lock().unwrap().heat_capacity = 0.9;
        CRYOFLUID.lock().unwrap().temperature = 0.25;
        CRYOFLUID.lock().unwrap().boil_point = 0.55;

        // neoplasm
        NEOPLASM.lock().unwrap().liquid.heat_capacity = 0.4;
        NEOPLASM.lock().unwrap().liquid.temperature = 0.54;
        NEOPLASM.lock().unwrap().liquid.viscosity = 0.85;
        NEOPLASM.lock().unwrap().liquid.flammability = 0.0;
        NEOPLASM.lock().unwrap().liquid.cap_puddles = false;
        NEOPLASM.lock().unwrap().spread_target = Some(WATER.lock().unwrap().clone());
        NEOPLASM.lock().unwrap().liquid.move_through_blocks = true;
        NEOPLASM.lock().unwrap().liquid.incinerable = true;
        NEOPLASM.lock().unwrap().liquid.block_reactive = false;
        NEOPLASM.lock().unwrap().liquid.can_stay_on.insert(WATER.lock().unwrap().clone());
        NEOPLASM.lock().unwrap().liquid.can_stay_on.insert(OIL.lock().unwrap().clone());
        NEOPLASM.lock().unwrap().liquid.can_stay_on.insert(CRYOFLUID.lock().unwrap().clone());

        // arkycite
        ARKYCITE.lock().unwrap().flammability = 0.4;
        ARKYCITE.lock().unwrap().viscosity = 0.7;

        // gallium
        GALLIUM.lock().unwrap().coolant = false;
        GALLIUM.lock().unwrap().hidden = true;

        // ozone
        OZONE.lock().unwrap().gas = true;
        OZONE.lock().unwrap().flammability = 1.0;
        OZONE.lock().unwrap().explosiveness = 1.0;

        // hydrogen
        HYDROGEN.lock().unwrap().gas = true;
        HYDROGEN.lock().unwrap().flammability = 1.0;

        // nitrogen
        NITROGEN.lock().unwrap().gas = true;

        // cyanogen
        CYANOGEN.lock().unwrap().gas = true;
        CYANOGEN.lock().unwrap().flammability = 2.0;
    }
}