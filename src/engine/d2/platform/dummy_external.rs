use crate::engine::d2::subsystem::ExternalSystem;

use super::Dynamic;

#[derive(Default, Clone, Copy, Debug)]
pub struct DummyExternal {
    pub supported: bool,
}

impl DummyExternal {
    pub fn new() -> Self {
        Default::default()
    }
}

impl ExternalSystem for DummyExternal {
    fn is_supported(&self) -> bool {
        false
    }

    fn call(&self, name: String, params: Option<Vec<Dynamic>>) -> Option<Dynamic> {
        None
    }

    fn bind(&self, name: String, function: Dynamic) {}
}
