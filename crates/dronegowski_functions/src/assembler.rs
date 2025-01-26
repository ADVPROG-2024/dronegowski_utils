use serde::Deserialize;
use wg_2024::packet::Fragment;

pub fn assembler(entry: &mut Vec<u8>, fragment: &Fragment) {
    // Calcola gli indici di inizio e fine basati sull'indice del frammento
    let start_index = fragment.fragment_index as usize * fragment.data.len();
    let end_index = start_index + fragment.data.len();

    // Assicurati che il vettore "entry" sia abbastanza grande per contenere il frammento
    if entry.len() < end_index {
        entry.resize(end_index, 0);
    }

    // Copia i dati del frammento nella posizione corretta del vettore
    entry[start_index..end_index].copy_from_slice(&fragment.data);
}

pub fn deserialize_message<'a, T: Deserialize<'a>>(entry: &'a [u8]) -> Result<T, bincode::Error> {
    // Tenta di deserializzare i dati nel tipo specificato
    log::info!("Dati del messaggio: {:?}", entry);
    bincode::deserialize(entry)
}
