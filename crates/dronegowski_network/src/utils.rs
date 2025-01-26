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

impl SimulationControllerNode {
    pub fn new(node_type: SimulationControllerNodeType, node_id: NodeId, neighbours: Vec<SimulationControllerNode>, nodi: &mut Vec<SimulationControllerNode>) -> Self {
        let node = Self {
            node_type,
            node_id,
            neighbours,
            xy: Self::set_coordinates(nodi),
        };
        nodi.push(node.clone());
        node
    }

    fn set_coordinates(nodi: &mut Vec<SimulationControllerNode>) -> (f32, f32){
        let mut x;
        let mut y;
        loop{
            x = rand::thread_rng().gen_range(50. ..550.);
            y = rand::thread_rng().gen_range(50. ..550.);
            if !nodi.iter().any(|node| {
                let dist = ((node.xy.0 - x).powi(2) + (node.xy.1 - y).powi(2)).sqrt();
                dist < 100.}) {
                break;
            }
        }
        (x, y)
    }
}
