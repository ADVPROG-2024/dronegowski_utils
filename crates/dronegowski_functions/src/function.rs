use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use thiserror::Error;
use wg_2024::config::Config;
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

pub fn validate_network(network: &Vec<SimulationControllerNode>) -> Result<(), ValidationError> {
    let mut graph: HashMap<NodeId, HashSet<NodeId>> = HashMap::new();

    //building the graph
    for node in network {
        println!("aggiungo nodo {}", node.node_id);
        match node.node_type{
            SimulationControllerNodeType::DRONE => {
                graph.entry(node.node_id).or_insert(HashSet::new()).insert(node.node_id);
                for &connected_id in &node.neighbours {
                    graph.entry(node.node_id).or_default().insert(connected_id);
                    graph.entry(connected_id).or_default(); // insert the neighbour node in the graph if not there
                }
            },
            SimulationControllerNodeType::SERVER => {
                if node.neighbours.len() < 2 {
                    return Err(ValidationError::ServerConnectionError);
                }
                for &connected_id in &node.neighbours {
                    if let Some(neighbor) = network.iter().find(|neighbor| neighbor.node_id == connected_id) {
                        match neighbor.node_type {
                            SimulationControllerNodeType::DRONE => {
                                graph.entry(node.node_id).or_default().insert(connected_id);
                                graph.entry(connected_id).or_default();
                            },
                            _ => {
                                return Err(ValidationError::ServerConnectionError);
                            }
                        }
                    }
                }
            },
            SimulationControllerNodeType::CLIENT => {
                if node.neighbours.len() > 2 || node.neighbours.len() < 1 {
                    return Err(ValidationError::ClientConnectionError);
                }
                for &connected_id in &node.neighbours {
                    if let Some(neighbor) = network.iter().find(|neighbor| neighbor.node_id == connected_id) {
                        match neighbor.node_type {
                            SimulationControllerNodeType::DRONE => {
                                graph.entry(node.node_id).or_default().insert(connected_id);
                                graph.entry(connected_id).or_default();
                            },
                            _ => {
                                return Err(ValidationError::ClientConnectionError);
                            }
                        }
                    }
                }
            }
        }
    }


    // bidirectional links checking
    for (&node, connections) in &graph {
        for &connected_node in connections {
            //checking of the opposite link
            if !graph
                .get(&connected_node)
                .map_or(false, |set| set.contains(&node))
            {
                return Err(ValidationError::NotBidirectional(node, connected_node));
            }
        }
    }

    // connected graph checking
    let all_nodes: HashSet<_> = graph.keys().copied().collect();
    let mut visited = HashSet::new();
    // takes any node as starting point
    let start_node = *all_nodes.iter().next().unwrap();

    dfs(start_node, &graph, &mut visited);

    println!("node visited {:?}", visited);
    println!("all nodes {:?}", all_nodes);
    if visited != all_nodes {
        return Err(ValidationError::NotConnected);
    }

    Ok(())
}

// DFS function used in the connected graph checking
fn dfs(node: NodeId, graph: &HashMap<NodeId, HashSet<NodeId>>, visited: &mut HashSet<NodeId>) {
    if visited.contains(&node) {
        return;
    }
    visited.insert(node);
    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            dfs(neighbor, graph, visited);
        }
    }
}

pub fn parse_config(file: &str) -> Config {
    let file_str = fs::read_to_string(file).expect("error reading config file");
    println!("Parsing configuration file...");
    toml::from_str(&file_str).expect("Error occurred during config file parsing")
}

fn parse_file(config: Config,  nodi: &mut Vec<SimulationControllerNode>) {

    for drone in config.drone {
        let mut neighbours = Vec::new();
        for neighbour in drone.connected_node_ids {
            neighbours.push(neighbour);
        }
        println!("aggiungo drone {}", drone.id);
        nodi.push(SimulationControllerNode{
            node_type: SimulationControllerNodeType::DRONE,
            node_id: drone.id,
            neighbours: neighbours,
            xy: (0.0, 0.0)
        });
    }

    for client in config.client {
        let mut neighbours = Vec::new();
        for neighbour in client.connected_drone_ids {
            neighbours.push(neighbour);
        }
        nodi.push(SimulationControllerNode{
            node_type: SimulationControllerNodeType::CLIENT,
            node_id: client.id,
            neighbours: neighbours,
            xy: (0.0, 0.0)
        });
    }

    for server in config.server {
        let mut neighbours = Vec::new();
        for neighbour in server.connected_drone_ids {
            neighbours.push(neighbour);
        }
        nodi.push(SimulationControllerNode{
            node_type: SimulationControllerNodeType::SERVER,
            node_id: server.id,
            neighbours: neighbours,
            xy: (0.0, 0.0)
        });
    }
}
#[test]
fn test_validate_network() {
    let mut network = Vec::new();
    let config = parse_config("config.toml");
    parse_file(config, &mut network);
    match validate_network(&network) {
        Ok(_) => println!("Network validation passed."),
        Err(e) => println!("Network validation failed: {:?}", e),
    }
}