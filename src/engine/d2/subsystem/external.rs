use crate::engine::d2::platform::Dynamic;

/// Functions for interacting with external code. When running in a web browser, this means
/// Javascript running on the page.
pub trait ExternalSystem {
    /// Whether the environment supports interaction with external code.
    fn is_supported(&self) -> bool;

    /// Call an external fn with the given parameters, and returns the result. Errors thrown by
    /// the called fn will propogate.
    fn call(&self, name: String, params: Option<Vec<Dynamic>>) -> Option<Dynamic>;

    /// Bind a fn to be called by external code.
    /// @param name The name to bind to. The namespace may be shared with third party code, so it's
    ///   good practice to prefix names. In Javascript, the fn will be hooked onto the window
    ///   object.
    /// @param fn The function, or None to unbind.
    fn bind(&self, name: String, function: Dynamic);
}
