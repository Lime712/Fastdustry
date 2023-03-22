/// Interface for {@link mindustry.entities.comp.EntityComp}
pub trait Entityc {
    fn is_added(&self) -> bool;

    fn is_local(&self) -> bool;

    fn is_null(&self) -> bool;

    fn is_remote(&self) -> bool;

    fn serialize(&self) -> bool;

    fn class_id(&self) -> i32;

    fn id(&self) -> i32;

    fn add(&mut self);

    fn after_read(&self);

    fn id_id(id: i32);

    // fn read(read: Reads);

    fn remove(&mut self);

    fn update(&self);

    // fn write(&selfwrite: Writes);
}
