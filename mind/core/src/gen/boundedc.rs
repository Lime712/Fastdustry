pub trait Boundec: Entityc + Flyingc + Healthc + Hitboxc + Posc + Velc {
    fn update();
}

pub trait Entityc {
    fn get_self<T: Entityc>() -> T;
    fn as_t<T>() -> T;
    fn is_added() -> bool;
    fn is_local() -> bool;
    fn is_null() -> bool;
    fn is_remote() -> bool;
    fn serialize() -> bool;
    fn class_id() -> i32;
    fn id() -> i32;
    fn add();
    fn after_read();
    fn set_id(id: i32);
    // todo fix this
    fn read(read: String);
    fn remove();
    fn update();
    fn write(write: String);
}

pub trait Flyingc {
    fn last_drown_floor() -> Option<Floor>;
    fn can_drown() -> bool;
    fn check_target(target_air: bool, target_ground: bool) -> bool;
    fn emit_walk_sound() -> bool;
    fn hovering() -> bool;
    fn is_flying() -> bool;
    fn is_grounded() -> bool;
    fn floor_speed_multiplier() -> f32;
    fn splash_timer() -> f32;
    fn drown_time() -> f32;
    fn elevation() -> f32;
    fn drown_floor() -> Floor;
    fn move_at(vector: Vec2, acceleration: f32);
    fn update();
    fn update_drowning();
    fn wobble();
}