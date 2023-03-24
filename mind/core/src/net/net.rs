use std::collections::HashMap;
use crate::net::packet::Packet;

static mut PACKET_PROVS: Vec<Box<dyn Packet>> = Vec::new();
static mut PACKET_CLASSES: Vec<Box<dyn Packet>> = Vec::new();
static mut PACKET_TO_ID: HashMap<String, i32> = HashMap::new();

pub struct Net {
    pub server: bool,
    pub active: bool,
    client_loaded: bool,
    packet_queue: Vec<Box<dyn Packet>>,
}