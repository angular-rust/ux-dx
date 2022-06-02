use std::{marker::PhantomData, rc::Rc};

use super::{Disposable, SignalBase, SignalConnection};

/// An alias for Signal2 listeners.
pub type Listener2<A, B> = Rc<dyn Fn(&A, &B)>;

/// A two-argument signal. See Signal0 and Signal1 for different arities.
#[derive(Default, Clone, Debug)]
pub struct Signal2<A, B> {
    pub inner: SignalBase,
    marker_a: PhantomData<A>,
    marker_b: PhantomData<B>,
}

impl<A, B> Signal2<A, B> {
    /// @param listener An optional listener to immediately connect to the signal.
    pub fn new(listener: Option<Listener2<A, B>>) -> Self {
        // Self {
        //     inner: SignalBase::new(listener),
        //     marker_a: PhantomData,
        //     marker_b: PhantomData,
        // }
        todo!("should deal with it");
    }

    /// Connects a listener to this signal.
    /// @param prioritize True if this listener should fire before others.
    /// @returns A SignalConnection, that can be disposed to remove the listener.
    // prioritize :bool = false
    pub fn connect(&self, listener: Listener2<A, B>, prioritize: bool) -> SignalConnection {
        todo!("should deal with it");
        // self.inner.connect_impl(listener, prioritize)
    }

    /// Emit the signal, notifying each connected listener.
    pub fn emit(&self, arg1: A, arg2: B) {
        if self.inner.dispatching() {
            todo!("should deal with it");
            // self.inner.defer(Box::new(|| {
            //     self.emitImpl(arg1, arg2);
            // }));
        } else {
            self.emit_impl(arg1, arg2);
        }
    }

    fn emit_impl(&self, arg1: A, arg2: B) {
        let head = self.inner.will_emit();
        let p = head;
        todo!("should deal with it");
        // while let Some(ref val) = p {
        //     if let Some(listener) = val.listener {
        //         // (listener)(arg1, arg2); // TODO:
        //     }
        //     if !val.stayInList {
        //         val.dispose();
        //     }
        //     p = val.next;
        // }
        // self.inner.didEmit(head.unwrap());
    }
}

impl<A, B> AsRef<SignalBase> for Signal2<A, B> {
    fn as_ref(&self) -> &SignalBase {
        &self.inner
    }
}
