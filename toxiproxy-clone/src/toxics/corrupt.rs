use rand::Rng;
use crate::toxic::Toxic;

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
}
