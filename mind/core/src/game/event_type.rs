pub struct UnitDamageEvent {
    pub unit: Unit,
    pub bullet: Bullet,
}

impl UnitDamageEvent {
    pub fn new(unit: Unit, bullet: Bullet) -> UnitDamageEvent {
        UnitDamageEvent {
            unit: unit,
            bullet: bullet,
        }
    }
}