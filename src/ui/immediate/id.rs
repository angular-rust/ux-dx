use super::*;
pub struct Id;

impl Id {
    pub fn handle(id: u64, ops: Option<HandleOptions>) -> Handle {
        // TODO: should prevent overrride the ROOT_HANDLE
        Handle::global().nest(id, ops)
    }
}
