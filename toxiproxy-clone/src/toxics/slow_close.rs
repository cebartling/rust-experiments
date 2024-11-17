use std::time::Duration;
use crate::toxic::Toxic;

pub struct SlowCloseToxic {
    #[allow(dead_code)]
    pub delay: Duration,
}

impl Toxic for SlowCloseToxic {
    fn modify_upstream(&self, _data: &mut Vec<u8>) {}
    fn modify_downstream(&self, _data: &mut Vec<u8>) {}
}
