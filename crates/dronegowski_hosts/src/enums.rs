use wg_2024::network::NodeId;
use wg_2024::packet::Packet;
use crossbeam_channel::{Sender};

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

pub enum ServerType {
    ContentServer,
    CommunicationServer
}