use crate::engine::d2::{
    animation::AnimatedFloat,
    util::{Disposable, Signal1, Value},
};

/// Displays a web page over the stage. In the HTML target, this is implemented with an iframe. In
/// AIR, it uses StageWebView. On Android, make sure your app manifest contains the INTERNET
/// permission.
pub trait WebView: Disposable {
    /// The URL currently being displayed. Can be set to load a different URL. In AIR, this value
    /// will change automatically if the user navigates to a different page.
    fn url(&self) -> Value<String>;

    /// An error message emitted if the page could not be loaded.
    fn error(&self) -> Signal1<String>;

    /// Viewport X position, in pixels.
    fn x(&self) -> AnimatedFloat;

    /// Viewport Y position, in pixels.
    fn y(&self) -> AnimatedFloat;

    /// Viewport width, in pixels.
    fn width(&self) -> AnimatedFloat;

    /// Viewport height, in pixels.
    fn height(&self) -> AnimatedFloat;
}
