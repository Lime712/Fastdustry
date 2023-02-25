use crate::arc_core::assets::asset_descriptor::AssetDescriptor;

pub trait Loadable {
    fn load_async(&mut self);
    fn load_sync(&mut self);
    fn get_name(&self) -> String;
    fn get_dependencies<T>(&self) -> Vec<AssetDescriptor<T>>;
}