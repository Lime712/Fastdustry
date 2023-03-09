use std::hash::{Hash, Hasher};

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

#[derive(Eq, Hash, PartialEq)]
pub struct Stat {
    pub category: StatCat,
    pub name: String,
    pub id: u32,
}

impl Stat {
    pub fn new(category: StatCat, name: String) -> Stat {
        unsafe {
            let s = Stat {
                category,
                name,
                id: ALL.len() as u32,
            };
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

// todo fix this
pub static health: Stat = Stat::new_general("health".to_string());
pub static armor: Stat = Stat::new_general("armor".to_string());
pub static size: Stat = Stat::new_general("size".to_string());
pub static displaySize: Stat = Stat::new_general("displaySize".to_string());
pub static buildTime: Stat = Stat::new_general("buildTime".to_string());
pub static buildCost: Stat = Stat::new_general("buildCost".to_string());
pub static memoryCapacity: Stat = Stat::new_general("memoryCapacity".to_string());
pub static explosiveness: Stat = Stat::new_general("explosiveness".to_string());
pub static flammability: Stat = Stat::new_general("flammability".to_string());
pub static radioactivity: Stat = Stat::new_general("radioactivity".to_string());
pub static charge: Stat = Stat::new_general("charge".to_string());
pub static heatCapacity: Stat = Stat::new_general("heatCapacity".to_string());
pub static viscosity: Stat = Stat::new_general("viscosity".to_string());
pub static temperature: Stat = Stat::new_general("temperature".to_string());
pub static flying: Stat = Stat::new_general("flying".to_string());
pub static speed: Stat = Stat::new_general("speed".to_string());
pub static buildSpeed: Stat = Stat::new_general("buildSpeed".to_string());
pub static mineSpeed: Stat = Stat::new_general("mineSpeed".to_string());
pub static mineTier: Stat = Stat::new_general("mineTier".to_string());
pub static payloadCapacity: Stat = Stat::new_general("payloadCapacity".to_string());
pub static baseDeflectChance: Stat = Stat::new_general("baseDeflectChance".to_string());
pub static lightningChance: Stat = Stat::new_general("lightningChance".to_string());
pub static lightningDamage: Stat = Stat::new_general("lightningDamage".to_string());
pub static abilities: Stat = Stat::new_general("abilities".to_string());
pub static canBoost: Stat = Stat::new_general("canBoost".to_string());
pub static maxUnits: Stat = Stat::new_general("maxUnits".to_string());
pub static damageMultiplier: Stat = Stat::new_general("damageMultiplier".to_string());
pub static healthMultiplier: Stat = Stat::new_general("healthMultiplier".to_string());
pub static speedMultiplier: Stat = Stat::new_general("speedMultiplier".to_string());
pub static reloadMultiplier: Stat = Stat::new_general("reloadMultiplier".to_string());
pub static buildSpeedMultiplier: Stat = Stat::new_general("buildSpeedMultiplier".to_string());
pub static reactive: Stat = Stat::new_general("reactive".to_string());
pub static healing: Stat = Stat::new_general("healing".to_string());
pub static immunities: Stat = Stat::new_general("immunities".to_string());
pub static itemCapacity: Stat = Stat::new(StatCat::ITEMS, "itemCapacity".to_string());
pub static itemsMoved: Stat = Stat::new(StatCat::ITEMS, "itemsMoved".to_string());
pub static launchTime: Stat = Stat::new(StatCat::ITEMS, "launchTime".to_string());
pub static maxConsecutive: Stat = Stat::new(StatCat::ITEMS, "maxConsecutive".to_string());
pub static liquidCapacity: Stat = Stat::new(StatCat::LIQUIDS, "liquidCapacity".to_string());
pub static powerCapacity: Stat = Stat::new(StatCat::POWER, "powerCapacity".to_string());
pub static powerUse: Stat = Stat::new(StatCat::POWER, "powerUse".to_string());
pub static powerDamage: Stat = Stat::new(StatCat::POWER, "powerDamage".to_string());
pub static powerRange: Stat = Stat::new(StatCat::POWER, "powerRange".to_string());
pub static powerConnections: Stat = Stat::new(StatCat::POWER, "powerConnections".to_string());
pub static basePowerGeneration: Stat = Stat::new(StatCat::POWER, "basePowerGeneration".to_string());
pub static tiles: Stat = Stat::new(StatCat::CRAFTING, "tiles".to_string());
pub static input: Stat = Stat::new(StatCat::CRAFTING, "input".to_string());
pub static output: Stat = Stat::new(StatCat::CRAFTING, "output".to_string());
pub static productionTime: Stat = Stat::new(StatCat::CRAFTING, "productionTime".to_string());
pub static maxEfficiency: Stat = Stat::new(StatCat::CRAFTING, "maxEfficiency".to_string());
pub static drillTier: Stat = Stat::new(StatCat::CRAFTING, "drillTier".to_string());
pub static drillSpeed: Stat = Stat::new(StatCat::CRAFTING, "drillSpeed".to_string());
pub static linkRange: Stat = Stat::new(StatCat::CRAFTING, "linkRange".to_string());
pub static instructions: Stat = Stat::new(StatCat::CRAFTING, "instructions".to_string());
pub static weapons: Stat = Stat::new(StatCat::FUNCTION, "weapons".to_string());
pub static bullet: Stat = Stat::new(StatCat::FUNCTION, "bullet".to_string());
pub static speedIncrease: Stat = Stat::new(StatCat::FUNCTION, "speedIncrease".to_string());
pub static repairTime: Stat = Stat::new(StatCat::FUNCTION, "repairTime".to_string());
pub static repairSpeed: Stat = Stat::new(StatCat::FUNCTION, "repairSpeed".to_string());
pub static range: Stat = Stat::new(StatCat::FUNCTION, "range".to_string());
pub static shootRange: Stat = Stat::new(StatCat::FUNCTION, "shootRange".to_string());
pub static inaccuracy: Stat = Stat::new(StatCat::FUNCTION, "inaccuracy".to_string());
pub static shots: Stat = Stat::new(StatCat::FUNCTION, "shots".to_string());
pub static reload: Stat = Stat::new(StatCat::FUNCTION, "reload".to_string());
pub static targetsAir: Stat = Stat::new(StatCat::FUNCTION, "targetsAir".to_string());
pub static targetsGround: Stat = Stat::new(StatCat::FUNCTION, "targetsGround".to_string());
pub static damage: Stat = Stat::new(StatCat::FUNCTION, "damage".to_string());
pub static ammo: Stat = Stat::new(StatCat::FUNCTION, "ammo".to_string());
pub static ammoUse: Stat = Stat::new(StatCat::FUNCTION, "ammoUse".to_string());
pub static shieldHealth: Stat = Stat::new(StatCat::FUNCTION, "shieldHealth".to_string());
pub static cooldownTime: Stat = Stat::new(StatCat::FUNCTION, "cooldownTime".to_string());
pub static moduleTier: Stat = Stat::new(StatCat::FUNCTION, "moduleTier".to_string());
pub static booster: Stat = Stat::new(StatCat::OPTIONAL, "booster".to_string());
pub static boostEffect: Stat = Stat::new(StatCat::OPTIONAL, "boostEffect".to_string());
pub static affinities: Stat = Stat::new(StatCat::OPTIONAL, "affinities".to_string());
pub static opposites: Stat = Stat::new(StatCat::OPTIONAL, "opposites".to_string());
