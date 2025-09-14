use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Netlist {
    pub nets: Vec<NetlistNet>,
    pub components: Vec<NetComponent>,
    pub connections: HashMap<String, Vec<Connection>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetlistNet {
    pub id: String,
    pub name: String,
    pub nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetComponent {
    pub id: String,
    pub reference: String,
    pub component_type: String,
    pub value: String,
    pub pins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub component_id: String,
    pub pin_id: String,
    pub net_id: String,
}