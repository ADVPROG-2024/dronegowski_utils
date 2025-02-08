use wg_2024::network::NodeId;
use wg_2024::packet::Packet;
use crossbeam_channel::{Sender};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum ClientEvent {
    PacketSent(Packet),
    MessageReceived(TestMessage), // Keep this for generic messages
    ServerTypeReceived(NodeId, ServerType), // Specific event for ServerType
    ClientListReceived(NodeId, Vec<NodeId>), // Specific event for ClientList
    FilesListReceived(NodeId, Vec<(u64, String)>), // Specific event for FilesList
    FileReceived(NodeId, String), // Specific event for a File
    MediaReceived(NodeId, Vec<u8>), // Specific event for Media
    MessageFromReceived(NodeId, NodeId, String), // Specific event for MessageFrom
    RegistrationOk(NodeId),
    RegistrationError(NodeId),
}

#[derive(Clone, Debug)]
pub enum ClientCommand{
    RemoveSender(NodeId), // Rimuove un drone collegato al Client
    AddSender(NodeId, Sender<Packet>), // // Aggiunge un drone collegato al Client
    ServerType(NodeId),
    FilesList(NodeId),
    File(NodeId, u64),
    Media(NodeId, u64),
    RegistrationToChat(NodeId),
    ClientList(NodeId),
    MessageFor(NodeId, NodeId, String),
    RequestNetworkDiscovery,
}

#[derive(Clone, Debug)]
pub enum ClientType {
    WebBrowsers,
    ChatClients
}


// Definizione di strutture dati e tipi personalizzati
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CustomStruct {
    pub id: u32,
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum CustomEnum {
    Variant1(String),
    Variant2 { id: u32, value: f64 }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ClientMessages {
    ServerType,
    FilesList,
    File(u64),
    Media(u64),
    RegistrationToChat,
    ClientList,
    MessageFor(NodeId, String),
    ServerMessages(ServerMessages),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FileContent {
    pub title: String,
    pub text: String,
    pub media_ids: Vec<(u64, String)>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ServerMessages {
    ServerType(ServerType),
    ClientList(Vec<NodeId>),
    FilesList(Vec<(u64, String)>),
    File(FileContent),
    Media(Vec<u8>),
    Error(String),
    MessageFrom(NodeId, String),
    RegistrationOk,
    RegistrationError(String),
}


// Enum per rappresentare diversi tipi di messaggi
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TestMessage {
    Text(String),
    Number(u32),
    Vector(Vec<u8>),
    WebServerMessages(ClientMessages),
    Struct(CustomStruct),
    Enum(CustomEnum)
}

#[derive(Clone, Debug)]
pub enum ServerEvent {
    PacketSent(Packet), // Avvisa il SC che è stato inviato un pacchetto
    MessageReceived(TestMessage)  // Avvisa il SC che il messaggio ora è completo
}

pub enum ServerCommand {
    AddSender(NodeId, Sender<Packet>),
    RemoveSender(NodeId),
    AddClient(NodeId),
    SendClients(NodeId),
    SendMessage(NodeId, String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ServerType {
    Content,
    Communication,
}
