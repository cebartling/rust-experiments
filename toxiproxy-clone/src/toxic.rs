// Toxic trait defines behavior modifications for the proxy
pub trait Toxic: Send + Sync {
    fn modify_upstream(&self, data: &mut Vec<u8>);
    fn modify_downstream(&self, data: &mut Vec<u8>);
}
