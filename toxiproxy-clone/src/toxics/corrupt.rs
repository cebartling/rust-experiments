use crate::toxic::{Toxic, ToxicConfig};
use rand::Rng;

pub struct CorruptToxic {
    pub probability: f64,
}

impl Toxic for CorruptToxic {
    fn modify_upstream(&self, data: &mut Vec<u8>) {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(self.probability) {
            if let Some(byte) = data.get_mut(0) {
                *byte = rng.random();
            }
        }
    }

    fn modify_downstream(&self, data: &mut Vec<u8>) {
        self.modify_upstream(data);
    }

    fn get_type(&self) -> String {
        "corrupt".to_string()
    }

    fn get_config(&self) -> ToxicConfig {
        ToxicConfig::Corrupt {
            probability: self.probability,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corrupt_toxic_modifies_upstream_with_probability() {
        let mut data = vec![1, 2, 3, 4];
        let toxic = CorruptToxic { probability: 1.0 };
        toxic.modify_upstream(&mut data);
        assert_ne!(data[0], 1);
    }

    #[test]
    fn corrupt_toxic_does_not_modify_upstream_with_zero_probability() {
        let mut data = vec![1, 2, 3, 4];
        let toxic = CorruptToxic { probability: 0.0 };
        toxic.modify_upstream(&mut data);
        assert_eq!(data[0], 1);
    }

    #[test]
    fn corrupt_toxic_modifies_downstream() {
        let mut data = vec![1, 2, 3, 4];
        let toxic = CorruptToxic { probability: 1.0 };
        toxic.modify_downstream(&mut data);
        assert_ne!(data[0], 1);
    }

    #[test]
    fn corrupt_toxic_get_type_returns_correct_type() {
        let toxic = CorruptToxic { probability: 0.5 };
        assert_eq!(toxic.get_type(), "corrupt");
    }

    #[test]
    fn corrupt_toxic_get_config_returns_correct_config() {
        let toxic = CorruptToxic { probability: 0.5 };
        if let ToxicConfig::Corrupt { probability } = toxic.get_config() {
            assert_eq!(probability, 0.5);
        } else {
            panic!("Expected ToxicConfig::Corrupt");
        }
    }
}