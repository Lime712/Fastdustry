use crate::arc_core::assets::asset_loader_parameters::AssetLoaderParameters;
use std::fs::File;

pub struct AssetDescriptor<T> {
    pub name: String,
    pub ty: T,
    pub params: AssetLoaderParameters,
    pub file: Option<File>,
    // callback for when the asset is loaded
    pub loaded_callback: Option<fn()>,
    // callback for when the asset has an error
    pub error_callback: Option<fn()>,
}

impl<T> AssetDescriptor<T> {
    pub fn new(name: String, ty: T, params: AssetLoaderParameters) -> AssetDescriptor<T> {
        AssetDescriptor {
            name,
            ty,
            params,
            file: None,
            loaded_callback: None,
            error_callback: None,
        }
    }
}
