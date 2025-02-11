use std::collections::{HashMap, HashSet};
use std::fs::File;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use thiserror::Error;
use wg_2024::network::NodeId;
use dronegowski_network::{SimulationControllerNode, SimulationControllerNodeType};
use fern;
use std::env;

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
            SimulationControllerNodeType::DRONE { .. } => {
                graph.entry(node.node_id).or_insert(HashSet::new()).insert(node.node_id);
                for &connected_id in &node.neighbours {
                    graph.entry(node.node_id).or_default().insert(connected_id);
                    graph.entry(connected_id).or_default(); // insert the neighbour node in the graph if not there
                }
            },
            SimulationControllerNodeType::SERVER { .. } => {
                if node.neighbours.len() < 2 {
                    return Err(ValidationError::ServerConnectionError);
                }
                for &connected_id in &node.neighbours {
                    if let Some(neighbor) = network.iter().find(|neighbor| neighbor.node_id == connected_id) {
                        match neighbor.node_type {
                            SimulationControllerNodeType::DRONE { .. } => {
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
            SimulationControllerNodeType::CLIENT { .. } => {
                if node.neighbours.len() > 2 || node.neighbours.len() < 1 {
                    return Err(ValidationError::ClientConnectionError);
                }
                for &connected_id in &node.neighbours {
                    if let Some(neighbor) = network.iter().find(|neighbor| neighbor.node_id == connected_id) {
                        match neighbor.node_type {
                            SimulationControllerNodeType::DRONE { .. } => {
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

pub fn setup_logging() {
    let client_filter = env::var("CLIENT_FILTER").unwrap_or_default();
    let drone_filter = env::var("DRONE_FILTER").unwrap_or_default();
    let server_filter = env::var("SERVER_FILTER").unwrap_or_default();

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .filter(move |metadata| {
            let target = metadata.target();
            
            // Client filter
            client_filter.split(',')
                .any(|f| target.starts_with(&format!("client_{}", f))) ||
            
            // Drone filter
            drone_filter.split(',')
                .any(|f| target.starts_with(&format!("drone_{}", f))) ||
            
            // Server filter
            server_filter.split(',')
                .any(|f| target.starts_with(&format!("server_{}", f)))
        })
        .chain(fern::log_file("output.log").unwrap())
        .apply()
        .unwrap();
}

pub fn enable_debug_for(target_type: &str, ids: &[u64]) {
    let filter_var = match target_type {
        "client" => "CLIENT_FILTER",
        "drone" => "DRONE_FILTER",
        "server" => "SERVER_FILTER",
        _ => return
    };
    
    let existing = env::var(filter_var).unwrap_or_default();
    let new_filter = if !existing.is_empty() {
        format!("{},{}", existing, ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(","))
    } else {
        ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",")
    };
    
    env::set_var(filter_var, new_filter);
}

pub fn disable_debug() {
    env::remove_var("CLIENT_FILTER");
    env::remove_var("DRONE_FILTER");
    env::remove_var("SERVER_FILTER");
}
