use crate::engine::d2::{subsystem::WebSystem, web::WebView};

pub struct DummyWeb;

impl DummyWeb {
    pub fn new() -> Self {
        Self {}
    }
}

impl WebSystem for DummyWeb {
    fn create_view(&self, x: f32, y: f32, width: f32, height: f32) -> Box<dyn WebView> {
        panic!("Web.createView is unsupported in this environment, check the `supported` flag.");
    }

    fn is_supported(&self) -> bool {
        false
    }

    fn open_browser(&self, url: String) {
        // Nothing
    }
}
