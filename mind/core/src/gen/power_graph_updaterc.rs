use crate::gen::entityc::Entityc;
use crate::world::blocks::power::power_graph::PowerGraph;

/// Interface for {@link mindustry.entities.comp.PowerGraphUpdaterComp}
pub trait PowerGraphUpdaterc : Entityc {
    fn graph(&mut self) -> &PowerGraph;

    fn graph_graph(&mut self, graph: PowerGraph);

    fn update(&mut self);
}
