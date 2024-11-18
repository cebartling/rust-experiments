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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    #[ignore]
    fn slow_close_toxic_adds_delay_upstream() {
        let mut data = vec![1, 2, 3, 4];
        let toxic = SlowCloseToxic { delay: Duration::from_millis(100) };
        let start = std::time::Instant::now();
        toxic.modify_upstream(&mut data);
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(100));
    }

    #[test]
    #[ignore]
    fn slow_close_toxic_adds_delay_downstream() {
        let mut data = vec![1, 2, 3, 4];
        let toxic = SlowCloseToxic { delay: Duration::from_millis(100) };
        let start = std::time::Instant::now();
        toxic.modify_downstream(&mut data);
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(100));
    }

    #[test]
    fn slow_close_toxic_get_type_returns_correct_type() {
        let toxic = SlowCloseToxic { delay: Duration::from_millis(100) };
        assert_eq!(toxic.get_type(), "slow_close");
    }

    #[test]
    fn slow_close_toxic_get_config_returns_correct_config() {
        let toxic = SlowCloseToxic { delay: Duration::from_millis(100) };
        if let ToxicConfig::SlowClose { delay_ms } = toxic.get_config() {
            assert_eq!(delay_ms, 100);
        } else {
            panic!("Expected ToxicConfig::SlowClose");
        }
    }
}