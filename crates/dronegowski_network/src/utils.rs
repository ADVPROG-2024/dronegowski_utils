use crossbeam_channel::{Sender};
use rand::Rng;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::network::NodeId;
use dronegowski_hosts::{ClientCommand, ClientEvent, ClientType, ServerCommand, ServerEvent, ServerType};

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    ClientEvent(ClientEvent),
    ServerEvent(ServerEvent),
    DroneEvent(DroneEvent),
}

#[derive(Debug, Clone)]
pub enum SimulationControllerNodeType {
    SERVER {server_channel: Sender<ServerCommand>, server_type: ServerType},
    CLIENT {client_channel: Sender<ClientCommand>, client_type: ClientType},
    DRONE {drone_channel: Sender<DroneCommand>, pdr: f32, drone_type: String},
}

#[derive(Clone)]
pub struct SimulationControllerNode {
    pub node_type: SimulationControllerNodeType,
    pub node_id: NodeId,
    pub neighbours: Vec<NodeId>,
    pub xy: (f32, f32),
    pub details: bool,
    pub event: Vec<Event>,
}

impl SimulationControllerNode {
    pub fn new(node_type: SimulationControllerNodeType, node_id: NodeId, neighbours: Vec<NodeId>, nodi: &mut Vec<SimulationControllerNode>) -> Self {
        let node = Self {
            node_type,
            node_id,
            neighbours,
            xy: Self::set_coordinates(nodi),
            details: false,
            event: Vec::new(),
        };
        nodi.push(node.clone());
        node
    }

    fn set_coordinates(nodi: &mut Vec<SimulationControllerNode>) -> (f32, f32){
        let mut x;
        let mut y;
        loop{
            x = rand::rng().random_range(50. ..800.);
            y = rand::rng().random_range(50. ..800.);
            if !nodi.iter().any(|node| {
                let dist = ((node.xy.0 - x).powi(2) + (node.xy.1 - y).powi(2)).sqrt();
                dist < 150.}) {
                break;
            }
        }
        (x, y)
    }
}
