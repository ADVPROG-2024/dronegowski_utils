use serde::Serialize;
use wg_2024::network::NodeId;
use wg_2024::packet::{Fragment, Packet};

pub fn assembler(entry: &mut Vec<u8>, fragment: &Fragment) {
    // Calcola gli indici di inizio e fine basati sull'indice del frammento
    let start_index = fragment.fragment_index as usize * 128;
    let end_index = start_index + fragment.data.len();

    // Assicurati che il vettore "entry" sia abbastanza grande per contenere il frammento
    if entry.len() < end_index {
        entry.resize(end_index, 0);
    }

    // Copia i dati del frammento nella posizione corretta del vettore
    entry[start_index..end_index].copy_from_slice(&fragment.data);
}

pub fn deserialize_message<T: serde::de::DeserializeOwned>(entry: &[u8]) -> Result<T, bincode::Error> {
    // Tenta di deserializzare i dati nel tipo specificato
    bincode::deserialize(entry)
}

