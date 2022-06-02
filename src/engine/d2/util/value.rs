use std::{cmp, fmt};

use crate::engine::d2::util::{Listener2, Signal2};

use super::Disposable;

/// Wraps a single value, notifying listeners when the value changes.
#[derive(Clone)]
pub struct Value<A>
where
    A: fmt::Display + cmp::PartialEq + fmt::Debug + Clone,
{
    // The wrapped value, setting this to a different value will fire the `changed` signal.
    pub value: A,

    // Emitted when the value has changed. The first listener parameter is the new current value,
    // the second parameter is the old previous value.
    pub signal: Option<Signal2<A, A>>,
}

impl<A> Value<A>
where
    A: fmt::Display + cmp::PartialEq + fmt::Debug + Clone,
{
    pub fn new(value: A, listener: Option<Listener2<A, A>>) -> Self {
        Self {
            value,
            signal: listener.map(|v| Signal2::new(Some(v))),
        }
    }

    /// Immediately calls a listener with the current value, and again whenever the value changes.
    /// @returns A handle that can be disposed to stop watching for changes.
    pub fn watch(&mut self, listener: Listener2<A, A>) -> Option<impl Disposable> {
        listener(&self.value, &self.value);
        if self.signal.is_none() {
            self.signal = Some(Signal2::new(None));
        }

        self.signal.as_ref().map(|s| s.connect(listener, false))
    }

    #[inline]
    pub fn get(&self) -> &A {
        &self.value
    }

    pub fn set(&mut self, new_value: A) {
        let old_value = self.value.clone();
        if new_value != old_value {
            self.value = new_value.clone();
            if let Some(ref signal) = self.signal {
                signal.emit(new_value, old_value);
            }
        }
    }

    pub fn changed(&mut self) -> Option<Signal2<A, A>> {
        if self.signal.is_none() {
            self.signal = Some(Signal2::new(None));
        }

        self.signal.clone()
    }
}

impl<A> fmt::Display for Value<A>
where
    A: fmt::Display + cmp::PartialEq + fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<A> fmt::Debug for Value<A>
where
    A: fmt::Display + cmp::PartialEq + fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Value")
            //  .field("x", &self.x)
            //  .field("y", &self.y)
            .finish()
    }
}

// impl<A: fmt::Display> fmt::Debug for Value<A> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Value")
//             //  .field("x", &self.x)
//             //  .field("y", &self.y)
//             .finish()
//     }
// }
