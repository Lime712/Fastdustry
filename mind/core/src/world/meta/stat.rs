use std::hash::{Hash, Hasher};

use lazy_static::lazy_static;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum StatCat {
    GENERAL,
    POWER,
    LIQUIDS,
    ITEMS,
    CRAFTING,
    FUNCTION,
    OPTIONAL,
}

#[derive(PartialEq)]
pub struct StatValue {
    pub value: f64,
    pub display: String,
}

impl Eq for StatValue {}

impl Hash for StatValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.to_bits().hash(state);
    }
}

// pub static mut ALL: Vec<Stat> = Vec::new();
static mut COUNTER: u32 = 0;

#[derive(Eq, Hash, PartialEq)]
pub struct Stat {
    pub category: StatCat,
    pub name: String,
    pub id: u32,
}

impl Stat {
    pub fn new(category: StatCat, name: String) -> Stat {
        let s;
        unsafe {
            s = Stat {
                category,
                name,
                id: COUNTER,
            };
            COUNTER += 1;
            // ALL.push(s);
        };
        s
    }

    pub fn new_general(name: String) -> Stat {
        Stat::new(StatCat::GENERAL, name)
    }
}

impl Clone for Stat {
    fn clone(&self) -> Self {
        Stat {
            category: self.category,
            name: self.name.clone(),
            id: self.id,
        }
    }
}

lazy_static! {
// todo fix this
pub static ref health: Stat = Stat::new_general("health".to_string());
pub static ref armor: Stat = Stat::new_general("armor".to_string());
pub static ref size: Stat = Stat::new_general("size".to_string());
pub static ref displaySize: Stat = Stat::new_general("displaySize".to_string());
pub static ref buildTime: Stat = Stat::new_general("buildTime".to_string());
pub static ref buildCost: Stat = Stat::new_general("buildCost".to_string());
pub static ref memoryCapacity: Stat = Stat::new_general("memoryCapacity".to_string());
pub static ref explosiveness: Stat = Stat::new_general("explosiveness".to_string());
pub static ref flammability: Stat = Stat::new_general("flammability".to_string());
pub static ref radioactivity: Stat = Stat::new_general("radioactivity".to_string());
pub static ref charge: Stat = Stat::new_general("charge".to_string());
pub static ref heatCapacity: Stat = Stat::new_general("heatCapacity".to_string());
pub static ref viscosity: Stat = Stat::new_general("viscosity".to_string());
pub static ref temperature: Stat = Stat::new_general("temperature".to_string());
pub static ref flying: Stat = Stat::new_general("flying".to_string());
pub static ref speed: Stat = Stat::new_general("speed".to_string());
pub static ref buildSpeed: Stat = Stat::new_general("buildSpeed".to_string());
pub static ref mineSpeed: Stat = Stat::new_general("mineSpeed".to_string());
pub static ref mineTier: Stat = Stat::new_general("mineTier".to_string());
pub static ref payloadCapacity: Stat = Stat::new_general("payloadCapacity".to_string());
pub static ref baseDeflectChance: Stat = Stat::new_general("baseDeflectChance".to_string());
pub static ref lightningChance: Stat = Stat::new_general("lightningChance".to_string());
pub static ref lightningDamage: Stat = Stat::new_general("lightningDamage".to_string());
pub static ref abilities: Stat = Stat::new_general("abilities".to_string());
pub static ref canBoost: Stat = Stat::new_general("canBoost".to_string());
pub static ref maxUnits: Stat = Stat::new_general("maxUnits".to_string());
pub static ref damageMultiplier: Stat = Stat::new_general("damageMultiplier".to_string());
}
lazy_static! {
    pub static ref healthMultiplier: Stat = Stat::new_general("healthMultiplier".to_string());
pub static ref speedMultiplier: Stat = Stat::new_general("speedMultiplier".to_string());
pub static ref reloadMultiplier: Stat = Stat::new_general("reloadMultiplier".to_string());
pub static ref buildSpeedMultiplier: Stat = Stat::new_general("buildSpeedMultiplier".to_string());
pub static ref reactive: Stat = Stat::new_general("reactive".to_string());
pub static ref healing: Stat = Stat::new_general("healing".to_string());
pub static ref immunities: Stat = Stat::new_general("immunities".to_string());
pub static ref itemCapacity: Stat = Stat::new(StatCat::ITEMS, "itemCapacity".to_string());
pub static ref itemsMoved: Stat = Stat::new(StatCat::ITEMS, "itemsMoved".to_string());
pub static ref launchTime: Stat = Stat::new(StatCat::ITEMS, "launchTime".to_string());
pub static ref maxConsecutive: Stat = Stat::new(StatCat::ITEMS, "maxConsecutive".to_string());
pub static ref liquidCapacity: Stat = Stat::new(StatCat::LIQUIDS, "liquidCapacity".to_string());
pub static ref powerCapacity: Stat = Stat::new(StatCat::POWER, "powerCapacity".to_string());
pub static ref powerUse: Stat = Stat::new(StatCat::POWER, "powerUse".to_string());
pub static ref powerDamage: Stat = Stat::new(StatCat::POWER, "powerDamage".to_string());
pub static ref powerRange: Stat = Stat::new(StatCat::POWER, "powerRange".to_string());
pub static ref powerConnections: Stat = Stat::new(StatCat::POWER, "powerConnections".to_string());
pub static ref basePowerGeneration: Stat = Stat::new(StatCat::POWER, "basePowerGeneration".to_string());
pub static ref tiles: Stat = Stat::new(StatCat::CRAFTING, "tiles".to_string());
pub static ref input: Stat = Stat::new(StatCat::CRAFTING, "input".to_string());
pub static ref output: Stat = Stat::new(StatCat::CRAFTING, "output".to_string());
pub static ref productionTime: Stat = Stat::new(StatCat::CRAFTING, "productionTime".to_string());
pub static ref maxEfficiency: Stat = Stat::new(StatCat::CRAFTING, "maxEfficiency".to_string());
pub static ref drillTier: Stat = Stat::new(StatCat::CRAFTING, "drillTier".to_string());
pub static ref drillSpeed: Stat = Stat::new(StatCat::CRAFTING, "drillSpeed".to_string());
pub static ref linkRange: Stat = Stat::new(StatCat::CRAFTING, "linkRange".to_string());
pub static ref instructions: Stat = Stat::new(StatCat::CRAFTING, "instructions".to_string());
pub static ref weapons: Stat = Stat::new(StatCat::FUNCTION, "weapons".to_string());
pub static ref bullet: Stat = Stat::new(StatCat::FUNCTION, "bullet".to_string());
pub static ref speedIncrease: Stat = Stat::new(StatCat::FUNCTION, "speedIncrease".to_string());
pub static ref repairTime: Stat = Stat::new(StatCat::FUNCTION, "repairTime".to_string());
pub static ref repairSpeed: Stat = Stat::new(StatCat::FUNCTION, "repairSpeed".to_string());
pub static ref range: Stat = Stat::new(StatCat::FUNCTION, "range".to_string());
pub static ref shootRange: Stat = Stat::new(StatCat::FUNCTION, "shootRange".to_string());
pub static ref inaccuracy: Stat = Stat::new(StatCat::FUNCTION, "inaccuracy".to_string());
pub static ref shots: Stat = Stat::new(StatCat::FUNCTION, "shots".to_string());
pub static ref reload: Stat = Stat::new(StatCat::FUNCTION, "reload".to_string());
pub static ref targetsAir: Stat = Stat::new(StatCat::FUNCTION, "targetsAir".to_string());
pub static ref targetsGround: Stat = Stat::new(StatCat::FUNCTION, "targetsGround".to_string());
pub static ref damage: Stat = Stat::new(StatCat::FUNCTION, "damage".to_string());
pub static ref ammo: Stat = Stat::new(StatCat::FUNCTION, "ammo".to_string());
pub static ref ammoUse: Stat = Stat::new(StatCat::FUNCTION, "ammoUse".to_string());
pub static ref shieldHealth: Stat = Stat::new(StatCat::FUNCTION, "shieldHealth".to_string());
pub static ref cooldownTime: Stat = Stat::new(StatCat::FUNCTION, "cooldownTime".to_string());
pub static ref moduleTier: Stat = Stat::new(StatCat::FUNCTION, "moduleTier".to_string());
pub static ref booster: Stat = Stat::new(StatCat::OPTIONAL, "booster".to_string());
pub static ref boostEffect: Stat = Stat::new(StatCat::OPTIONAL, "boostEffect".to_string());
pub static ref affinities: Stat = Stat::new(StatCat::OPTIONAL, "affinities".to_string());
pub static ref opposites: Stat = Stat::new(StatCat::OPTIONAL, "opposites".to_string());
}