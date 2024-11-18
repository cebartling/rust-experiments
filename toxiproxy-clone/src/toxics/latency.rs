use crate::toxic::{Toxic, ToxicConfig};
use std::thread;
use std::time::Duration;

// Latency toxic adds delay to the connection
pub struct LatencyToxic {
    pub latency: Duration,
}

impl Toxic for LatencyToxic {
    fn modify_upstream(&self, _data: &mut Vec<u8>) {
        thread::sleep(self.latency);
    }

    fn modify_downstream(&self, _data: &mut Vec<u8>) {
        thread::sleep(self.latency);
    }

    fn get_type(&self) -> String {
        "latency".to_string()
    }

    fn get_config(&self) -> ToxicConfig {
        ToxicConfig::Latency {
            latency_ms: self.latency.as_millis() as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn latency_toxic_adds_delay_upstream() {
        let mut data = vec![1, 2, 3, 4];
        let toxic = LatencyToxic { latency: Duration::from_millis(100) };
        let start = std::time::Instant::now();
        toxic.modify_upstream(&mut data);
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(100));
    }

    #[test]
    fn latency_toxic_adds_delay_downstream() {
        let mut data = vec![1, 2, 3, 4];
        let toxic = LatencyToxic { latency: Duration::from_millis(100) };
        let start = std::time::Instant::now();
        toxic.modify_downstream(&mut data);
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(100));
    }

    #[test]
    fn latency_toxic_get_type_returns_correct_type() {
        let toxic = LatencyToxic { latency: Duration::from_millis(100) };
        assert_eq!(toxic.get_type(), "latency");
    }

    #[test]
    fn latency_toxic_get_config_returns_correct_config() {
        let toxic = LatencyToxic { latency: Duration::from_millis(100) };
        if let ToxicConfig::Latency { latency_ms } = toxic.get_config() {
            assert_eq!(latency_ms, 100);
        } else {
            panic!("Expected ToxicConfig::Latency");
        }
    }
}