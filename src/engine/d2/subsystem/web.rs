use crate::engine::d2::web::WebView;

/// Functions related to the environment's web browser.
pub trait WebSystem {
    /// True if the environment supports WebViews. Note that this will always be false on the browser
    /// Flash target.
    fn is_supported(&self) -> bool;

    /// Creates a blank WebView with the given viewport bounds, in pixels. Fails with an assertion if
    /// this environment doesn't support WebViews.
    fn create_view(&self, x: f32, y: f32, width: f32, height: f32) -> Box<dyn WebView>;

    /// Open a new browser window or tab to the given URL. This operation is always supported. URI
    /// schemes such as mailto: are also available. On mobile, sms: and tel: are supported. On
    /// Android, market: is supported.
    fn open_browser(&self, url: String);
}
