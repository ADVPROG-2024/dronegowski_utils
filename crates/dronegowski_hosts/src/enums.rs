use wg_2024::network::NodeId;
use wg_2024::packet::Packet;
use crossbeam_channel::{Sender};
use serde::{Deserialize, Serialize};

pub enum ClientEvent{
    PacketSent(Packet), // Avvisa il SC che è stato inviato un pacchetto
    MessageReceived(Vec<u8>),  // Avvisa il SC che il messaggio ora è completo
}

pub enum ClientCommand{
    RemoveSender(NodeId), // Rimuove un drone collegato al Client
    AddSender(NodeId, Sender<Packet>), // // Aggiunge un drone collegato al Client
}

#[derive(Clone, Debug)]
pub enum ClientType {
    WebBrowsers,
    ChatClients,
}


// Definizione di strutture dati e tipi personalizzati
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct CustomStruct {
    id: u32,
    name: String,
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum CustomEnum {
    Variant1(String),
    Variant2 { id: u32, value: f64 },
}

// Enum per rappresentare diversi tipi di messaggi
#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum TestMessage {
    Text(String),
    Number(u32),
    Vector(Vec<u8>),
    Struct(CustomStruct),
    Enum(CustomEnum),
}


pub enum ServerType {
    ContentServer,
    CommunicationServer
}