use crate::gen::entityc::Entityc;
use crate::gen::indexable_entity_all::IndexableEntityAll;
use crate::gen::indexable_entity_power_graph::IndexableEntityPowerGraph;
use crate::gen::power_graph_updaterc::PowerGraphUpdaterc;
use crate::world::blocks::power::power_graph::PowerGraph;

pub struct PowerGraphUpdater {
    added: bool,
    graph: Option<&'static PowerGraph>,
    id: i32,
    index__all: i32,
    index__power_graph: i32,
}

impl Entityc for PowerGraphUpdater {
    fn is_added(&self) -> bool {
        self.added
    }

    fn is_local(&self) -> bool {
        todo!()
    }

    fn is_null(&self) -> bool {
        false
    }

    fn is_remote(&self) -> bool {
        todo!()
    }

    fn serialize(&self) -> bool {
        false
    }

    fn class_id(&self) -> i32 {
        42
    }

    fn id(&self) -> i32 {
        self.id
    }

    fn add(&mut self) {
        if self.added {
            return;
        }
        self.index__all = Groups.all.add_index(&self);
        self.index__power_graph = Groups.power_graph.add_index(&self);
        self.added = true;
    }

    fn after_read(&self) {
    }

    fn id_id(id: i32) {
        todo!()
    }

    fn remove(&mut self) {
        if !self.added {
            return;
        }
        Groups.all.remove_index(&self, self.index__all);
        self.index__all = -1;
        Groups.power_graph.remove_index(&self, self.index__power_graph);
        self.index__power_graph = -1;
        self.added = false;
    }

    fn update(&self) {
       self.graph.unwrap().update();
    }
}

impl IndexableEntityAll for PowerGraphUpdater {
    fn set_index_all(&mut self, index: i32) {
        self.index__all = index;
    }
}

impl IndexableEntityPowerGraph for PowerGraphUpdater {
    fn set_index_power_graph(&mut self, index: i32) {
        self.index__power_graph = index;
    }
}

impl PowerGraphUpdaterc for PowerGraphUpdater {
    fn graph(&mut self) -> &PowerGraph {
        self.graph.unwrap()
    }

    fn graph_graph(&mut self, graph: PowerGraph) {
        self.graph = Some(&graph);
    }

    fn update(&mut self) {
    }
}

impl PowerGraphUpdater {
    pub fn create() -> PowerGraphUpdater {
        let mut s = Self {
            added: false,
            graph: None,
            id: -1,
            index__all: -1,
            index__power_graph: -1,
        };
        s.id = entity_group.next_id();
        s
    }
}