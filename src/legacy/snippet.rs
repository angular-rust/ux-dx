#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use crate::{Object, SnippetHook};

use glib::translate::*;
use glib::GString;
use std::fmt;

glib_wrapper! {
    pub struct Snippet(Object<ffi::CoglSnippet, SnippetClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_snippet_get_gtype(),
    }
}

impl Snippet {
    /// Allocates and initializes a new snippet with the given source strings.
    /// ## `hook`
    /// The point in the pipeline that this snippet will wrap around
    ///  or replace.
    /// ## `declarations`
    /// The source code for the declarations for this
    ///  snippet or `None`. See `Snippet::set_declarations`.
    /// ## `post`
    /// The source code to run after the hook point where this
    ///  shader snippet is attached or `None`. See `Snippet::set_post`.
    ///
    /// # Returns
    ///
    /// a pointer to a new `Snippet`
    pub fn new(hook: SnippetHook, declarations: &str, post: &str) -> Snippet {
        unsafe {
            from_glib_full(ffi::cogl_snippet_new(
                hook.to_glib(),
                declarations.to_glib_none().0,
                post.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the source string that was set with
    ///  `Snippet::set_declarations` or `None` if none was set.
    pub fn get_declarations(&self) -> Option<GString> {
        unsafe { from_glib_none(ffi::cogl_snippet_get_declarations(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the hook that was set when `Snippet::new` was
    ///  called.
    pub fn get_hook(&self) -> SnippetHook {
        unsafe { from_glib(ffi::cogl_snippet_get_hook(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the source string that was set with
    ///  `Snippet::set_post` or `None` if none was set.
    pub fn get_post(&self) -> Option<GString> {
        unsafe { from_glib_none(ffi::cogl_snippet_get_post(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the source string that was set with
    ///  `Snippet::set_pre` or `None` if none was set.
    pub fn get_pre(&self) -> Option<GString> {
        unsafe { from_glib_none(ffi::cogl_snippet_get_pre(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the source string that was set with
    ///  `Snippet::set_replace` or `None` if none was set.
    pub fn get_replace(&self) -> Option<GString> {
        unsafe { from_glib_none(ffi::cogl_snippet_get_replace(self.to_glib_none().0)) }
    }

    /// Sets a source string that will be inserted in the global scope of
    /// the generated shader when this snippet is used on a pipeline. This
    /// string is typically used to declare uniforms, attributes or
    /// functions that will be used by the other parts of the snippets.
    ///
    /// This function should only be called before the snippet is attached
    /// to its first pipeline. After that the snippet should be considered
    /// immutable.
    /// ## `declarations`
    /// The new source string for the declarations section
    ///  of this snippet.
    pub fn set_declarations(&self, declarations: &str) {
        unsafe {
            ffi::cogl_snippet_set_declarations(
                self.to_glib_none().0,
                declarations.to_glib_none().0,
            );
        }
    }

    /// Sets a source string that will be inserted after the hook point in
    /// the generated shader for the pipeline that this snippet is attached
    /// to. Please see the documentation of each hook point in
    /// `Pipeline` for a description of how this string should be used.
    ///
    /// This function should only be called before the snippet is attached
    /// to its first pipeline. After that the snippet should be considered
    /// immutable.
    /// ## `post`
    /// The new source string for the post section of this snippet.
    pub fn set_post(&self, post: &str) {
        unsafe {
            ffi::cogl_snippet_set_post(self.to_glib_none().0, post.to_glib_none().0);
        }
    }

    /// Sets a source string that will be inserted before the hook point in
    /// the generated shader for the pipeline that this snippet is attached
    /// to. Please see the documentation of each hook point in
    /// `Pipeline` for a description of how this string should be used.
    ///
    /// This function should only be called before the snippet is attached
    /// to its first pipeline. After that the snippet should be considered
    /// immutable.
    /// ## `pre`
    /// The new source string for the pre section of this snippet.
    pub fn set_pre(&self, pre: &str) {
        unsafe {
            ffi::cogl_snippet_set_pre(self.to_glib_none().0, pre.to_glib_none().0);
        }
    }

    /// Sets a source string that will be used instead of any generated
    /// source code or any previous snippets for this hook point. Please
    /// see the documentation of each hook point in `Pipeline` for a
    /// description of how this string should be used.
    ///
    /// This function should only be called before the snippet is attached
    /// to its first pipeline. After that the snippet should be considered
    /// immutable.
    /// ## `replace`
    /// The new source string for the replace section of this snippet.
    pub fn set_replace(&self, replace: &str) {
        unsafe {
            ffi::cogl_snippet_set_replace(self.to_glib_none().0, replace.to_glib_none().0);
        }
    }
}

impl fmt::Display for Snippet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Snippet")
    }
}
