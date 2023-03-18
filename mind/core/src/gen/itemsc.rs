use crate::gen::entityc::Entityc;
use crate::gen::posc::Posc;
use crate::r#type::item::Item;
/// Interface for {@link mindustry.entities.comp.ItemsComp}
pub trait Itemsc : Entityc + Posc {
    fn accepts_item(item: Item) -> bool;

    fn has_item() -> bool;

    fn item_time() -> f32;

    fn item_capacity() -> i32;

    fn max_accepted(item: Item) -> i32;

    fn item() -> Item;

    fn stack() -> ItemStack;

    fn add_item(item: Item);

    fn add_item_item(item: Item, amount: i32);

    fn clear_item();

    fn item_time_item_time(item_time: f32);

    fn stack_stack(stack: ItemStack);

    fn update();
}
