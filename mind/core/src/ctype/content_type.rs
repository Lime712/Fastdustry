pub enum ContentType {
    Item(Item),
    Block(Block),
    MechUnused(Mech),
    Bullet(BulletType),
    Liquid(Liquid),
    Status(StatusEffect),
    Unit(UnitType),
    Weather(Weather),
    EffectUnused(Effect),
    Sector(SectorPreset),
    LoadOutUnused(Loadout),
    TypeIdUnused(TypeID),
    Error(Error),
    Planet(Planet),
    AmmoUnused(AmmoType),
    Team(TeamEntry),
}

impl ContentType {
    fn all() -> Vec<ContentType> {
        vec![
            ContentType::Item(Item::all()),
            ContentType::Block(Block::all()),
            ContentType::MechUnused(Mech::all()),
            ContentType::Bullet(BulletType::all()),
            ContentType::Liquid(Liquid::all()),
            ContentType::Status(StatusEffect::all()),
            ContentType::Unit(UnitType::all()),
            ContentType::Weather(Weather::all()),
            ContentType::EffectUnused(Effect::all()),
            ContentType::Sector(SectorPreset::all()),
            ContentType::LoadOutUnused(Loadout::all()),
            ContentType::TypeIdUnused(TypeID::all()),
            ContentType::Error(Error::all()),
            ContentType::Planet(Planet::all()),
            ContentType::AmmoUnused(AmmoType::all()),
            ContentType::Team(TeamEntry::all()),
        ]
    }
}
