use super::{Disposable, SignalBase, SignalConnection};

/// An alias for Signal0 listeners.
pub type Listener0 = Box<dyn Fn()>;

/// A zero-argument signal. See Signal1 and Signal2 for different arities.

#[derive(Default, Clone, Debug)]
pub struct Signal0 {
    pub inner: SignalBase,
}

impl Signal0 {
    /// @param listener An optional listener to immediately connect to the signal.
    pub fn new(listener: Option<Listener0>) -> Self {
        // Self {
        //     inner: SignalBase::new(listener),
        // }
        todo!("should deal with it");
    }

    /// Connects a listener to this signal.
    /// @param prioritize True if this listener should fire before others.
    /// @returns A SignalConnection, that can be disposed to remove the listener.
    //  prioritize :bool = false
    pub fn connect(&self, listener: Listener0, prioritize: bool) -> SignalConnection {
        // self.inner.connect_impl(listener, prioritize)
        todo!("should deal with it");
    }

    /// Emit the signal, notifying each connected listener.
    pub fn emit(&self) {
        if self.inner.dispatching() {
            todo!("should deal with it");
            // self.inner.defer(Box::new(|| {
            //     self.emitImpl();
            // }));
        } else {
            self.emit_impl();
        }
    }

    fn emit_impl(&self) {
        let head = self.inner.will_emit();
        let p = head;
        todo!("should deal with it");
        // while let Some(ref val) = p {
        //     if let Some(listener) = val.listener {
        //         (listener)();
        //     }
        //     if !val.stayInList {
        //         val.dispose();
        //     }
        //     p = val.next;
        // }
        // self.inner.didEmit(head.unwrap());
    }
}

impl AsRef<SignalBase> for Signal0 {
    fn as_ref(&self) -> &SignalBase {
        &self.inner
    }
}
