use std::thread;
use std::time::Duration;
use crate::toxic::Toxic;

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
}
