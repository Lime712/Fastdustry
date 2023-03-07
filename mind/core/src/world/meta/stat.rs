use std::hash::{Hash, Hasher};

#[derive(Eq, Hash, PartialEq)]
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

pub static mut ALL: Vec<Stat> = Vec::new();

#[derive(Eq, Hash, PartialEq)]
pub struct Stat {
    pub category: StatCat,
    pub name: String,
    pub id: u32,
}

impl Stat {
    pub fn new(category: StatCat, name: String) {
        unsafe {
            let s = Stat {
                category,
                name,
                id: ALL.len() as u32,
            };
            ALL.push(s);
        };
    }

    pub fn new_general(name: String) {
        Stat::new(StatCat::GENERAL, name);
    }
}

pub fn init() {
    unsafe {
        Stat::new_general("health".to_string());
        Stat::new_general("armor".to_string());
        Stat::new_general("size".to_string());
        Stat::new_general("displaySize".to_string());
        Stat::new_general("buildTime".to_string());
        Stat::new_general("buildCost".to_string());
        Stat::new_general("memoryCapacity".to_string());
        Stat::new_general("explosiveness".to_string());
        Stat::new_general("flammability".to_string());
        Stat::new_general("radioactivity".to_string());
        Stat::new_general("charge".to_string());
        Stat::new_general("heatCapacity".to_string());
        Stat::new_general("viscosity".to_string());
        Stat::new_general("temperature".to_string());
        Stat::new_general("flying".to_string());
        Stat::new_general("speed".to_string());
        Stat::new_general("buildSpeed".to_string());
        Stat::new_general("mineSpeed".to_string());
        Stat::new_general("mineTier".to_string());
        Stat::new_general("payloadCapacity".to_string());
        Stat::new_general("baseDeflectChance".to_string());
        Stat::new_general("lightningChance".to_string());
        Stat::new_general("lightningDamage".to_string());
        Stat::new_general("abilities".to_string());
        Stat::new_general("canBoost".to_string());
        Stat::new_general("maxUnits".to_string());
        Stat::new_general("damageMultiplier".to_string());
        Stat::new_general("healthMultiplier".to_string());
        Stat::new_general("speedMultiplier".to_string());
        Stat::new_general("reloadMultiplier".to_string());
        Stat::new_general("buildSpeedMultiplier".to_string());
        Stat::new_general("reactive".to_string());
        Stat::new_general("healing".to_string());
        Stat::new_general("immunities".to_string());
        Stat::new(StatCat::ITEMS,"itemCapacity".to_string());
        Stat::new(StatCat::ITEMS,"itemsMoved".to_string());
        Stat::new(StatCat::ITEMS,"launchTime".to_string());
        Stat::new(StatCat::ITEMS,"maxConsecutive".to_string());
        Stat::new(StatCat::LIQUIDS,"liquidCapacity".to_string());
        Stat::new(StatCat::POWER,"powerCapacity".to_string());
        Stat::new(StatCat::POWER,"powerUse".to_string());
        Stat::new(StatCat::POWER,"powerDamage".to_string());
        Stat::new(StatCat::POWER,"powerRange".to_string());
        Stat::new(StatCat::POWER,"powerConnections".to_string());
        Stat::new(StatCat::POWER,"basePowerGeneration".to_string());
        Stat::new(StatCat::CRAFTING,"tiles".to_string());
        Stat::new(StatCat::CRAFTING,"input".to_string());
        Stat::new(StatCat::CRAFTING,"output".to_string());
        Stat::new(StatCat::CRAFTING,"productionTime".to_string());
        Stat::new(StatCat::CRAFTING,"maxEfficiency".to_string());
        Stat::new(StatCat::CRAFTING,"drillTier".to_string());
        Stat::new(StatCat::CRAFTING,"drillSpeed".to_string());
        Stat::new(StatCat::CRAFTING,"linkRange".to_string());
        Stat::new(StatCat::CRAFTING,"instructions".to_string());
        Stat::new(StatCat::FUNCTION,"weapons".to_string());
        Stat::new(StatCat::FUNCTION,"bullet".to_string());
        Stat::new(StatCat::FUNCTION,"speedIncrease".to_string());
        Stat::new(StatCat::FUNCTION,"repairTime".to_string());
        Stat::new(StatCat::FUNCTION,"repairSpeed".to_string());
        Stat::new(StatCat::FUNCTION,"range".to_string());
        Stat::new(StatCat::FUNCTION,"shootRange".to_string());
        Stat::new(StatCat::FUNCTION,"inaccuracy".to_string());
        Stat::new(StatCat::FUNCTION,"shots".to_string());
        Stat::new(StatCat::FUNCTION,"reload".to_string());
        Stat::new(StatCat::FUNCTION,"targetsAir".to_string());
        Stat::new(StatCat::FUNCTION,"targetsGround".to_string());
        Stat::new(StatCat::FUNCTION,"damage".to_string());
        Stat::new(StatCat::FUNCTION,"ammo".to_string());
        Stat::new(StatCat::FUNCTION,"ammoUse".to_string());
        Stat::new(StatCat::FUNCTION,"shieldHealth".to_string());
        Stat::new(StatCat::FUNCTION,"cooldownTime".to_string());
        Stat::new(StatCat::FUNCTION,"moduleTier".to_string());
        Stat::new(StatCat::OPTIONAL,"booster".to_string());
        Stat::new(StatCat::OPTIONAL,"boostEffect".to_string());
        Stat::new(StatCat::OPTIONAL,"affinities".to_string());
        Stat::new(StatCat::OPTIONAL,"opposites".to_string());
    }
}