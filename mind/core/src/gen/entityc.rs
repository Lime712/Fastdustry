/// Interface for {@link mindustry.entities.comp.EntityComp}
pub trait Entityc {
    /// Replaced with `this` after code generation.
    fn extends_entityc>_t_self() -> <T;

    fn t_as() -> <T>;

    fn is_added() -> bool;

    fn is_local() -> bool;

    fn is_null() -> bool;

    fn is_remote() -> bool;

    fn serialize() -> bool;

    fn class_id() -> i32;

    fn id() -> i32;

    fn add();

    fn after_read();

    fn id_id(id: i32);

    fn read(read: Reads);

    fn remove();

    fn update();

    fn write(write: Writes);
}