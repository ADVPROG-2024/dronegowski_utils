use std::collections::{HashMap, HashSet};
use std::fs::File;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use thiserror::Error;
use wg_2024::network::NodeId;
use dronegowski_network::{SimulationControllerNode, SimulationControllerNodeType};

pub fn generate_unique_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_millis() as u64
}

pub fn simple_log() {
    let log_level = LevelFilter::Info;
    let _logger = WriteLogger::init(
        log_level,
        ConfigBuilder::new().set_thread_level(log_level).build(),
        File::create("output.log").expect("Could not create log file"),
    );
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("The connection between node {0} and node {1} is not bidirectional.")]
    NotBidirectional(NodeId, NodeId),
    #[error("The graph is not connected.")]
    NotConnected,
    #[error("Client connection error")]
    ClientConnectionError,
    #[error("Server connection error")]
    ServerConnectionError,
    // Other errors needed can be added here.
}

// pub fn validate_network(network: &mut Vec<SimulationControllerNode>) -> Result<(), ValidationError> {
//     let mut graph: HashMap<NodeId, HashSet<NodeId>> = HashMap::new();
//
//     //building the graph
//     for node in network {
//         match node.node_type{
//             SimulationControllerNodeType::DRONE => {
//                 for connected_id in node.neighbours {
//                     graph.entry(node.node_id).or_default().insert(connected_id);
//                     graph.entry(connected_id).or_default(); // insert the neighbour node in the graph if not there
//                 }
//             }
//             SimulationControllerNodeType::SERVER => {
//                 if node.neighbours.len() < 2 {
//                     return Err(ValidationError::ServerConnectionError);
//                 }
//                 for connected_id in node.neighbours {
//                     graph.entry(node.node_id).or_default().insert(connected_id);
//                     graph.entry(connected_id).or_default();
//                 }
//             }
//             SimulationControllerNodeType::CLIENT => {
//                 if node.neighbours.len() > 2 || node.neighbours.len() < 1 {
//                     return Err(ValidationError::ClientConnectionError);
//                 }
//                 for connected_id in node.neighbours {
//                     graph.entry(node.node_id).or_default().insert(connected_id);
//                     graph.entry(connected_id).or_default();
//                 }
//             }
//         }
//     }
//
//
//     // bidirectional links checking
//     for (&node, connections) in &graph {
//         for &connected_node in connections {
//             //checking of the opposite link
//             if !graph
//                 .get(&connected_node)
//                 .map_or(false, |set| set.contains(&node))
//             {
//                 return Err(ValidationError::NotBidirectional(node, connected_node));
//             }
//         }
//     }
//
//     // connected graph checking
//     let all_nodes: HashSet<_> = graph.keys().copied().collect();
//     let mut visited = HashSet::new();
//     // takes any node as starting point
//     let start_node = *all_nodes.iter().next().unwrap();
//
//     dfs(start_node, &graph, &mut visited);
//
//     if visited != all_nodes {
//         return Err(ValidationError::NotConnected);
//     }
//
//     Ok(())
// }
//
// // DFS function used in the connected graph checking
// fn dfs(node: NodeId, graph: &HashMap<NodeId, HashSet<NodeId>>, visited: &mut HashSet<NodeId>) {
//     if visited.contains(&node) {
//         return;
//     }
//     visited.insert(node);
//     if let Some(neighbors) = graph.get(&node) {
//         for &neighbor in neighbors {
//             dfs(neighbor, graph, visited);
//         }
//     }
// }