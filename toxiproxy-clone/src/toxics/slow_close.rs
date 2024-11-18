use crate::toxic::{Toxic, ToxicConfig};
use std::time::Duration;

pub struct SlowCloseToxic {
    pub delay: Duration,
}

impl Toxic for SlowCloseToxic {
    fn modify_upstream(&self, _data: &mut Vec<u8>) {}
    fn modify_downstream(&self, _data: &mut Vec<u8>) {}
    fn get_type(&self) -> String {
        "slow_close".to_string()
    }

    fn get_config(&self) -> ToxicConfig {
        ToxicConfig::SlowClose {
            delay_ms: self.delay.as_millis() as u64,
        }
    }
}
