use crossbeam_channel::{unbounded, Receiver, Sender};
use rand::Rng;
use wg_2024::controller::DroneCommand;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;
use dronegowski_hosts::{ClientCommand, ClientType, ServerCommand};

#[derive(Debug, Clone)]
pub enum SimulationControllerNodeType {
    SERVER {server_channel: Sender<ServerCommand>},
    CLIENT {client_channel: Sender<ClientCommand>, client_type: ClientType},
    DRONE {drone_channel: Sender<DroneCommand>, pdr: f32},
}

#[derive(Clone)]
pub struct SimulationControllerNode {
    pub node_type: SimulationControllerNodeType,
    pub node_id: NodeId,
    pub neighbours: Vec<NodeId>,
    pub xy: (f32, f32),
}

impl SimulationControllerNode {
    pub fn new(node_type: SimulationControllerNodeType, node_id: NodeId, neighbours: Vec<NodeId>, nodi: &mut Vec<SimulationControllerNode>) -> Self {
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
            x = rand::rng().random_range(50. ..1500.);
            y = rand::rng().random_range(50. ..800.);
            if !nodi.iter().any(|node| {
                let dist = ((node.xy.0 - x).powi(2) + (node.xy.1 - y).powi(2)).sqrt();
                dist < 200.}) {
                break;
            }
        }
        (x, y)
    }
}
