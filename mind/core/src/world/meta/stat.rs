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
pub static ref HEALTH: Stat = Stat::new_general("health".to_string());
pub static ref ARMOR: Stat = Stat::new_general("armor".to_string());
pub static ref SIZE: Stat = Stat::new_general("size".to_string());
pub static ref DISPLAY_SIZE: Stat = Stat::new_general("displaySize".to_string());
pub static ref BUILD_TIME: Stat = Stat::new_general("buildTime".to_string());
pub static ref BUILD_COST: Stat = Stat::new_general("buildCost".to_string());
pub static ref MEMORY_CAPACITY: Stat = Stat::new_general("memoryCapacity".to_string());
pub static ref EXPLOSIVENESS: Stat = Stat::new_general("explosiveness".to_string());
pub static ref FLAMMABILITY: Stat = Stat::new_general("flammability".to_string());
pub static ref RADIOACTIVITY: Stat = Stat::new_general("radioactivity".to_string());
pub static ref CHARGE: Stat = Stat::new_general("charge".to_string());
pub static ref HEAT_CAPACITY: Stat = Stat::new_general("heatCapacity".to_string());
pub static ref VISCOSITY: Stat = Stat::new_general("viscosity".to_string());
pub static ref TEMPERATURE: Stat = Stat::new_general("temperature".to_string());
pub static ref FLYING: Stat = Stat::new_general("flying".to_string());
pub static ref SPEED: Stat = Stat::new_general("speed".to_string());
pub static ref BUILD_SPEED: Stat = Stat::new_general("buildSpeed".to_string());
pub static ref MINE_SPEED: Stat = Stat::new_general("mineSpeed".to_string());
pub static ref MINE_TIER: Stat = Stat::new_general("mineTier".to_string());
pub static ref PAYLOAD_CAPACITY: Stat = Stat::new_general("payloadCapacity".to_string());
pub static ref BASE_DEFLECT_CHANCE: Stat = Stat::new_general("baseDeflectChance".to_string());
pub static ref LIGHTNING_CHANCE: Stat = Stat::new_general("lightningChance".to_string());
pub static ref LIGHTNING_DAMAGE: Stat = Stat::new_general("lightningDamage".to_string());
pub static ref ABILITIES: Stat = Stat::new_general("abilities".to_string());
pub static ref CAN_BOOST: Stat = Stat::new_general("canBoost".to_string());
pub static ref MAX_UNITS: Stat = Stat::new_general("maxUnits".to_string());
pub static ref DAMAGE_MULTIPLIER: Stat = Stat::new_general("damageMultiplier".to_string());
}
lazy_static! {
    pub static ref HEALTH_MULTIPLIER: Stat = Stat::new_general("healthMultiplier".to_string());
pub static ref SPEED_MULTIPLIER: Stat = Stat::new_general("speedMultiplier".to_string());
pub static ref RELOAD_MULTIPLIER: Stat = Stat::new_general("reloadMultiplier".to_string());
pub static ref BUILD_SPEED_MULTIPLIER: Stat = Stat::new_general("buildSpeedMultiplier".to_string());
pub static ref REACTIVE: Stat = Stat::new_general("reactive".to_string());
pub static ref HEALING: Stat = Stat::new_general("healing".to_string());
pub static ref IMMUNITIES: Stat = Stat::new_general("immunities".to_string());
pub static ref ITEM_CAPACITY: Stat = Stat::new(StatCat::ITEMS, "itemCapacity".to_string());
pub static ref ITEMS_MOVED: Stat = Stat::new(StatCat::ITEMS, "itemsMoved".to_string());
pub static ref LAUNCH_TIME: Stat = Stat::new(StatCat::ITEMS, "launchTime".to_string());
pub static ref MAX_CONSECUTIVE: Stat = Stat::new(StatCat::ITEMS, "maxConsecutive".to_string());
pub static ref LIQUID_CAPACITY: Stat = Stat::new(StatCat::LIQUIDS, "liquidCapacity".to_string());
pub static ref POWER_CAPACITY: Stat = Stat::new(StatCat::POWER, "powerCapacity".to_string());
pub static ref POWER_USE: Stat = Stat::new(StatCat::POWER, "powerUse".to_string());
pub static ref POWER_DAMAGE: Stat = Stat::new(StatCat::POWER, "powerDamage".to_string());
pub static ref POWER_RANGE: Stat = Stat::new(StatCat::POWER, "powerRange".to_string());
pub static ref POWER_CONNECTIONS: Stat = Stat::new(StatCat::POWER, "powerConnections".to_string());
pub static ref BASE_POWER_GENERATION: Stat = Stat::new(StatCat::POWER, "basePowerGeneration".to_string());
pub static ref TILES: Stat = Stat::new(StatCat::CRAFTING, "tiles".to_string());
pub static ref INPUT: Stat = Stat::new(StatCat::CRAFTING, "input".to_string());
pub static ref OUTPUT: Stat = Stat::new(StatCat::CRAFTING, "output".to_string());
pub static ref PRODUCTION_TIME: Stat = Stat::new(StatCat::CRAFTING, "productionTime".to_string());
pub static ref MAX_EFFICIENCY: Stat = Stat::new(StatCat::CRAFTING, "maxEfficiency".to_string());
pub static ref DRILL_TIER: Stat = Stat::new(StatCat::CRAFTING, "drillTier".to_string());
pub static ref DRILL_SPEED: Stat = Stat::new(StatCat::CRAFTING, "drillSpeed".to_string());
pub static ref LINK_RANGE: Stat = Stat::new(StatCat::CRAFTING, "linkRange".to_string());
pub static ref INSTRUCTIONS: Stat = Stat::new(StatCat::CRAFTING, "instructions".to_string());
pub static ref WEAPONS: Stat = Stat::new(StatCat::FUNCTION, "weapons".to_string());
pub static ref BULLET: Stat = Stat::new(StatCat::FUNCTION, "bullet".to_string());
pub static ref SPEED_INCREASE: Stat = Stat::new(StatCat::FUNCTION, "speedIncrease".to_string());
pub static ref REPAIR_TIME: Stat = Stat::new(StatCat::FUNCTION, "repairTime".to_string());
pub static ref REPAIR_SPEED: Stat = Stat::new(StatCat::FUNCTION, "repairSpeed".to_string());
pub static ref RANGE: Stat = Stat::new(StatCat::FUNCTION, "range".to_string());
pub static ref SHOOT_RANGE: Stat = Stat::new(StatCat::FUNCTION, "shootRange".to_string());
pub static ref INACCURACY: Stat = Stat::new(StatCat::FUNCTION, "inaccuracy".to_string());
pub static ref SHOTS: Stat = Stat::new(StatCat::FUNCTION, "shots".to_string());
pub static ref RELOAD: Stat = Stat::new(StatCat::FUNCTION, "reload".to_string());
pub static ref TARGETS_AIR: Stat = Stat::new(StatCat::FUNCTION, "targetsAir".to_string());
pub static ref TARGETS_GROUND: Stat = Stat::new(StatCat::FUNCTION, "targetsGround".to_string());
pub static ref DAMAGE: Stat = Stat::new(StatCat::FUNCTION, "damage".to_string());
pub static ref AMMO: Stat = Stat::new(StatCat::FUNCTION, "ammo".to_string());
pub static ref AMMO_USE: Stat = Stat::new(StatCat::FUNCTION, "ammoUse".to_string());
pub static ref SHIELD_HEALTH: Stat = Stat::new(StatCat::FUNCTION, "shieldHealth".to_string());
pub static ref COOLDOWN_TIME: Stat = Stat::new(StatCat::FUNCTION, "cooldownTime".to_string());
pub static ref MODULE_TIER: Stat = Stat::new(StatCat::FUNCTION, "moduleTier".to_string());
pub static ref BOOSTER: Stat = Stat::new(StatCat::OPTIONAL, "booster".to_string());
pub static ref BOOST_EFFECT: Stat = Stat::new(StatCat::OPTIONAL, "boostEffect".to_string());
pub static ref AFFINITIES: Stat = Stat::new(StatCat::OPTIONAL, "affinities".to_string());
pub static ref OPPOSITES: Stat = Stat::new(StatCat::OPTIONAL, "opposites".to_string());
}