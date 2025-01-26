use serde::Deserialize;
use wg_2024::packet::Fragment;
use dronegowski_hosts::TestMessage;

pub fn assembler(entry: &mut Vec<u8>, fragment: &Fragment) {
    // Calcola gli indici di inizio e fine basati sull'indice del frammento
    let start_index = fragment.fragment_index as usize * fragment.data.len();
    let end_index = start_index + fragment.data.len();

    // Assicurati che il vettore "entry" sia abbastanza grande per contenere il frammento
    if entry.len() < end_index {
        entry.resize(end_index, 0); // Resize del vettore fino a `end_index` con valori di default (0)
    }

    // Estendi il vettore con i dati del frammento a partire dall'indice di inizio
    entry[start_index..end_index].copy_from_slice(&fragment.data);
}

pub fn deserialize_message<'a, T: Deserialize<'a>>(entry: &'a [u8]) -> Result<TestMessage, bincode::Error> {
    log::info!("Lunghezza messaggio ricostruito: {}", entry.len());
    log::info!("Dati del messaggio ricostruito: {:?}", entry);
    bincode::deserialize(entry)
}
