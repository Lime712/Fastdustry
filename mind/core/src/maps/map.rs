use std::collections::{HashMap, HashSet};

use arc::arc_core::files::fi::Fi;

use crate::vars::DATA_DIRECTORY;

pub struct Map {
    pub custom: bool,
    pub tags: HashMap<String, String>,
    pub file: Fi,
    pub version: i32,
    pub width: i32,
    pub height: i32,
    // pub texture: Texture,
    pub build: i32,
    pub teams: Vec<i32>,
    pub spawns: i32,
    // pub loaded_mod: mod,
}

impl Map {
    pub fn new(custom: bool, tags: HashMap<String, String>, file: Fi, version: i32, width: i32, height: i32, build: i32, teams: Vec<i32>, spawns: i32) -> Map {
        Map {
            custom,
            tags,
            file,
            version,
            width,
            height,
            build,
            teams,
            spawns,
        }
    }

    pub fn new_tags(tags: HashMap<String, String>) -> Map {
        if !tags.contains_key("name") {
            panic!("Map must have a name");
        }
        let map = Map::new(false, tags.clone(), Fi::new_from_path(unsafe { DATA_DIRECTORY.to_string() } + tags.get("name").clone().unwrap()), 0, 0, 0, -1, vec![], 0);
        map
    }

    // pub fn save_texture(&mut self) {}

    // pub fn preview_file() {}

    // pub fn apply_rules(mode: Gamemode) -> Rules {
    //     // mode specific defaults have been applied
    //     let out = Rules::new();
    //     mode.apply(out);
    //
    //     // now apply map-specific rules
    //     return rules(out);
    // }
}