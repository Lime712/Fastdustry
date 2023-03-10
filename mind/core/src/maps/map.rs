use std::collections::{HashMap};

use arc::arc_core::files::fi::Fi;

use crate::vars::DATA_DIRECTORY;

pub struct Map<'a> {
    pub custom: bool,
    pub tags: HashMap<&'a str, &'a str>,
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

impl<'a> Default for Map<'a> {
    fn default() -> Self {
        let mut tags = HashMap::new();
        tags.insert("name", "unknown");
        Map {
            custom: false,
            tags,
            file: Fi::new_from_path(unsafe { DATA_DIRECTORY.to_string() }),
            version: 0,
            width: 0,
            height: 0,
            build: -1,
            teams: vec![],
            spawns: 0,
        }
    }
}

impl<'a> Map<'a> {
    pub fn new(custom: bool, tags: HashMap<&'a str, &'a str>, file: Fi, version: i32, width: i32, height: i32, build: i32, teams: Vec<i32>, spawns: i32) -> Map<'a> {
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

    pub fn new_tags(tags: HashMap<&'a str, &'a str>) -> Map<'a> {
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