use crate::engine::d2::util::{
    Disposable, Listener0, Listener1, Listener2, Signal0, Signal1, Signal2,
};

use super::Component;

/// A component that manages a set of Disposable objects. When this component is removed from its
/// owner, all managed Disposables are disposed.

pub struct Disposer {
    pub inner: Component,
    disposables: Vec<Box<dyn Disposable>>,
}

impl Disposer {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
            disposables: Vec::new(),
        }
    }

    /// Add a Disposable, so that it also gets disposed when this component does.
    /// @returns This instance, for chaining.
    pub fn add(&mut self, disposable: Box<dyn Disposable>) -> &Self {
        self.disposables.push(disposable);

        self
    }

    /// Remove a Disposable from this disposer.
    /// @returns True if the disposable was removed.
    pub fn remove(&self, disposable: impl Disposable) /*-> bool*/
    {
        // self._disposables.remove(disposable)
        unimplemented!()
    }

    /// Chainable convenience method for connecting a signal listener and adding its SignalConnection
    /// to this disposer.
    /// @returns This instance, for chaining.
    pub fn connect0(&mut self, signal: Signal0, listener: Listener0) -> &Self {
        self.add(Box::new(signal.connect(listener, false)));

        self
    }

    /// Chainable convenience method for connecting a signal listener and adding its SignalConnection
    /// to this disposer.
    /// @returns This instance, for chaining.
    pub fn connect1<A>(&mut self, signal: Signal1<A>, listener: Listener1<A>) -> &Self {
        self.add(Box::new(signal.connect(listener, false)));

        self
    }

    /// Chainable convenience method for connecting a signal listener and adding its SignalConnection
    /// to this disposer.
    /// @returns This instance, for chaining.
    pub fn connect2<A, B>(&mut self, signal: Signal2<A, B>, listener: Listener2<A, B>) -> &Self {
        self.add(Box::new(signal.connect(listener, false)));

        self
    }

    // override
    pub fn on_removed(&mut self) {
        self.free_disposables();
    }

    // override
    pub fn dispose(&mut self) {
        self.inner.dispose();
        self.free_disposables(); // Cleanup even if this component had no owner
    }

    fn free_disposables(&mut self) {
        while let Some(disposable) = self.disposables.pop() {
            disposable.dispose()
        }
    }
}

impl AsRef<Component> for Disposer {
    fn as_ref(&self) -> &Component {
        &self.inner
    }
}
