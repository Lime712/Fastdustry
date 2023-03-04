use std::fs::{File};
use crate::arc_core::files::FType;

pub struct Fi {
    file: File,
    file_type: FType,
}

impl Fi {
    pub fn new(file: File, file_type: FType) -> Fi {
        Fi { file, file_type }
    }

    pub fn new_from_path(path: String, file_type: FType) -> Fi {
        let f = match File::create(path) {
            Ok(f) => f,
            Err(e) => panic!("Error opening file: {}", e),
        };
        Fi::new(f, file_type)
    }

    pub fn new_from_file(file: File) -> Fi {
        Fi::new(file, FType::Absolute)
    }

    pub fn new_from_path_and_type(path: String, file_type: FType) -> Fi {
        Fi::new_from_path(path, file_type)
    }

    pub fn get(path: String) -> Fi {
        Fi::new_from_path(path, FType::Absolute)
    }
}