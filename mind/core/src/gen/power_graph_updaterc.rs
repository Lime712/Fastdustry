use crate::gen::entityc::Entityc;
/// Interface for {@link mindustry.entities.comp.PowerGraphUpdaterComp}
pub trait PowerGraphUpdaterc : Entityc {
    fn graph() -> PowerGraph;

    fn graph_graph(graph: PowerGraph);

    fn update();
}
