use crate::r#type::item::Item;

pub struct ItemStack {
    pub item: &'static Item,
    pub amount: i32,
}

impl ItemStack {
    pub fn new(item: &'static Item, amount: i32) -> ItemStack {
        ItemStack { item, amount }
    }
}

// create a Vec of ItemStacks from a list of items and a list of amounts
pub fn create_item_stacks(items: Vec<&'static Item>, amounts: Vec<i32>) -> Vec<ItemStack> {
    // check if the lists are the same length
    if items.len() != amounts.len() {
        panic!("create_item_stacks: items and amounts are not the same length: {} != {}", items.len(), amounts.len());
    }
    let mut item_stacks = Vec::new();
    for i in 0..items.len() {
        item_stacks.push(ItemStack::new(items[i], amounts[i]));
    }
    item_stacks
}
