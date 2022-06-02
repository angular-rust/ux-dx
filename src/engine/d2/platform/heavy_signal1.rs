use crate::engine::d2::util::{Listener1, Signal1, SignalConnection, Value};

/// An internal Signal1 with extra frills.
pub struct HeavySignal1<A> {
    pub inner: Signal1<A>,
    /// A watchable value, for detecting when the first listener was connected, or the last
    /// connection was disposed.
    pub has_listeners_value: Value<bool>,
}

impl<A> HeavySignal1<A> {
    pub fn new(listener: Option<Listener1<A>>) -> Self {
        let inner = Signal1::new(listener);
        let has_listeners_value = Value::<bool>::new(inner.inner.has_listeners(), None);

        Self {
            inner,
            has_listeners_value,
        }
    }

    // override
    // prioritize :bool = false
    pub fn connect(&mut self, listener: Listener1<A>, prioritize: bool) -> SignalConnection {
        let connection = self.inner.connect(listener, prioritize);
        self.has_listeners_value
            .set(self.inner.inner.has_listeners());
        return connection;
    }

    // override
    fn disconnect(&mut self, conn: SignalConnection) {
        self.inner.inner.disconnect(&conn);
        self.has_listeners_value
            .set(self.inner.inner.has_listeners());
    }

    // override
    fn did_emit(&mut self, head: SignalConnection) {
        self.inner.inner.did_emit(head);
        self.has_listeners_value
            .set(self.inner.inner.has_listeners());
    }
}

impl<A> AsRef<Signal1<A>> for HeavySignal1<A> {
    fn as_ref(&self) -> &Signal1<A> {
        &self.inner
    }
}
