use std::marker::PhantomData;

use super::{Disposable, SignalBase, SignalConnection};

/// An alias for Signal1 listeners.
pub type Listener1<A> = Box<dyn Fn(&A)>;

/// A one-argument signal. See Signal0 and Signal2 for different arities.
#[derive(Default, Clone, Debug)]
pub struct Signal1<A> {
    pub inner: SignalBase,
    marker: PhantomData<A>,
}

impl<A> Signal1<A> {
    /// @param listener An optional listener to immediately connect to the signal.
    pub fn new(listener: Option<Listener1<A>>) -> Self {
        todo!("should deal with it");
        // Self {
        //     inner: SignalBase::new(listener),
        //     marker: PhantomData,
        // }
    }

    /// Connects a listener to this signal.
    /// @param prioritize True if this listener should fire before others.
    /// @returns A SignalConnection, that can be disposed to remove the listener.
    // prioritize :bool = false
    pub fn connect(&self, listener: Listener1<A>, prioritize: bool) -> SignalConnection {
        todo!("should deal with it");
        // self.inner.connect_impl(listener, prioritize)
    }

    /// Emit the signal, notifying each connected listener.
    pub fn emit(&self, arg1: A) {
        if self.inner.dispatching() {
            todo!("should deal with it");
            // self.inner.defer(Box::new(|| {
            //     self.emitImpl(arg1);
            // }));
        } else {
            self.emit_impl(arg1);
        }
    }

    fn emit_impl(&self, arg1: A) {
        let head = self.inner.will_emit();
        let p = head;
        todo!("should deal with it")
        // while let Some(ref val) = p {
        //     if let Some(listener) = val.listener {
        //         // (listener)(arg1); //TODO:
        //     }

        //     if !val.stayInList {
        //         val.dispose();
        //     }
        //     p = val.next;
        // }
        // self.inner.didEmit(head.unwrap());
    }
}

impl<A> AsRef<SignalBase> for Signal1<A> {
    fn as_ref(&self) -> &SignalBase {
        &self.inner
    }
}
