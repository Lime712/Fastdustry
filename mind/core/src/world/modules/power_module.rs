use crate::world::blocks::power::power_graph::PowerGraph;

pub struct PowerModule {
    /// In case of unbuffered consumers, this is the percentage (1.0f = 100%) of the demanded power which can be supplied.
    /// Blocks will work at a reduced efficiency if this is not equal to 1.0f.
    /// In case of buffered consumers, this is the percentage of power stored in relation to the maximum capacity.
    pub status: f32,
    pub init: bool,
    pub graph: PowerGraph,
    pub links: Vec<i32>
}