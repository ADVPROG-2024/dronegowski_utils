use wg_2024::network::{NodeId, SourceRoutingHeader};
use wg_2024::packet::{Fragment, Packet, PacketType};
use serde::Serialize;
use bincode;

/// Serializes and fragments high-level messages into atomic packets.
pub fn fragment_message<T: Serialize>(
    message: &T,
    hops: Vec<NodeId>,
    session_id: u64,
) -> Vec<Packet> {
    // Step 1: Serialize the message
    let serialized_message = bincode::serialize(message).expect("Serialization failed");

    // Step 2: Fragment the serialized data into fixed-size packets
    const FRAGMENT_SIZE: usize = 128; // Fixed size of each fragment
    let total_n_fragments = (serialized_message.len() + FRAGMENT_SIZE - 1) / FRAGMENT_SIZE;

    let mut packets = Vec::new();

    for (fragment_index, chunk) in serialized_message.chunks(FRAGMENT_SIZE).enumerate() {
        // Copy only the actual chunk into the data array
        let data: Vec<u8> = chunk.to_vec();

        // Create the fragment
        let fragment = Fragment {
            fragment_index: fragment_index as u64,
            total_n_fragments: total_n_fragments as u64,
            length: chunk.len() as u8, // Actual length of this chunk
            data,
        };

        // Create the packet
        let packet = Packet {
            pack_type: PacketType::MsgFragment(fragment),
            routing_header: SourceRoutingHeader {
                hop_index: 0,
                hops: hops.clone(),
            },
            session_id,
        };

        packets.push(packet);
    }

    packets
}
