use serde::{Deserialize, Serialize};

// The Toxic trait defines behavior modifications for the proxy
pub trait Toxic: Send + Sync {
    fn modify_upstream(&self, data: &mut Vec<u8>);
    fn modify_downstream(&self, data: &mut Vec<u8>);
    fn get_type(&self) -> String;
    fn get_config(&self) -> ToxicConfig;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ToxicConfig {
    Latency { latency_ms: u64 },
    Corrupt { probability: f64 },
    SlowClose { delay_ms: u64 },
}

