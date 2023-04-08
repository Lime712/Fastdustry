use std::collections::{HashMap, HashSet};

use arc::arc_core::func::Cons;

use crate::content::items::Items;
use crate::content::liquids::Liquids;
use crate::ctype::content::Content;
use crate::ctype::content_type::ContentType;
use crate::ctype::unlockable_content::MappableContent;

pub struct ContentLoader {
    content_name_map: Vec<HashMap<String, Box<dyn MappableContent>>>,
    content_map: Vec<Vec<Content>>,
    temporary_mapper: Vec<Vec<Box<dyn MappableContent>>>,
    // current_mod: LoadedMod,
    last_added: Content,
    initialization: HashSet<Cons<Content>>,
}

impl ContentLoader {
    pub fn new() -> ContentLoader {
        let mut s = ContentLoader {
            content_name_map: Vec::new(),
            content_map: Vec::new(),
            temporary_mapper: Vec::new(),
            // current_mod: LoadedMod::new(),
            last_added: Content::default(),
            initialization: HashSet::new(),
        };
        for _ty in ContentType::all() {
            s.content_map.push(Vec::new());
            s.content_name_map.push(HashMap::new());
        }
        s
    }

    pub fn create_base_content() {
        // TeamEntries.load();
        Liquids::load();
    }
}
