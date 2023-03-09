pub trait Sensible {
    fn sense(&self, sensor: LAccess) -> f64;
    fn sense_content(&self, content: Content) -> f64 {
        0.0
    }
}

pub enum LAcess {
    totalItems,
    firstItem,
    totalLiquids,
    totalPower,
    itemCapacity,
    liquidCapacity,
    powerCapacity,
    powerNetStored,
    powerNetCapacity,
    powerNetIn,
    powerNetOut,
    ammo,
    ammoCapacity,
    health,
    maxHealth,
    heat,
    efficiency,
    progress,
    timescale,
    rotation,
    x,
    y,
    shootX,
    shootY,
    size,
    dead,
    range,
    shooting,
    boosting,
    mineX,
    mineY,
    mining,
    speed,
    team,
    ty,
    flag,
    controlled,
    controller,
    name,
    payloadCount,
    payloadType,

    //values with parameters are considered controllable
    enabled("to"),
    //"to" is standard for single parameter access
    shoot("x", "y", "shoot"),
    shootp(true, "unit", "shoot"),
    config(true, "to"),
    color("to");
}