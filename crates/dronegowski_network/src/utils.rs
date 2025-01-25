use wg_2024::network::NodeId;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum SimulationControllerNodeType {
    SERVER,
    CLIENT,
    DRONE,
}

#[derive(Clone)]
pub struct SimulationControllerNode {
    pub node_type: SimulationControllerNodeType,
    pub node_id: NodeId,
    pub neighbours: Vec<NodeId>,
    pub xy: (f32, f32),
}