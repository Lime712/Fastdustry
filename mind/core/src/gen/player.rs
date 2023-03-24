pub const DEATH_DELAY: f32 = 60.0;

pub struct Player {
    pub added: bool,
    pub admin: bool,
    // sync local
    pub boosting: bool,
    // color: Color,
    pub con: Option<NetConnection>,
    pub death_timer: f32,
    pub id: i32, // entity_group::next_id(),
    // some more stuff
    // todo: add more stuff
}

