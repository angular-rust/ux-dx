//
// Any object that can be disposed to free its resources and clean things up properly.
//
pub trait Disposable {
    fn dispose(&self);
}
