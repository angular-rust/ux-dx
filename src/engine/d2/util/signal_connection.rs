use crate::engine::d2::platform::Dynamic;

use super::{Disposable, SignalBase};

/// Represents a connected signal listener.
#[derive(Default, Clone, Debug)]
pub struct SignalConnection {
    /// True if the listener will remain connected after being used.
    pub stay_in_list: bool,
    pub next: Box<Option<SignalConnection>>,

    pub listener: Option<Dynamic>,
    pub signal: Option<SignalBase>,
}

impl SignalConnection {
    pub fn new(signal: Option<SignalBase>, listener: Option<Dynamic>) -> Self {
        // Self {
        //     signal,
        //     listener,
        //     stayInList: true,
        //     next: None,
        // }
        todo!("should deal with it");
    }

    /// Tells the connection to dispose itself after being used once.
    /// @returns This instance, for chaining.
    pub fn once(&mut self) -> &Self {
        self.stay_in_list = false;

        self
    }
}

impl Disposable for SignalConnection {
    /// Disconnects the listener from the signal.
    fn dispose(&self) {
        if let Some(ref signal) = self.signal {
            signal.disconnect(self);
            // self.signal = None; // DV
        }
    }
}
