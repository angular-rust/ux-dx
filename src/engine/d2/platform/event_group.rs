use crate::engine::d2::util::Disposable;

use super::{Dynamic, Event, IEventDispatcher};

pub type Listener = Box<dyn Fn(Dynamic)>;

/// Manages a group of event listeners. When the group is disposed, all listeners are removed.
pub struct EventGroup {
    entries: Vec<Entry>,
}

impl EventGroup {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Register a listener with this group.
    pub fn add_listener(&self, dispatcher: IEventDispatcher, type_: String, listener: Listener) {
        // dispatcher.addEventListener(type_, listener, false);
        // self.entries.push(Entry::new(dispatcher, type_, listener));
        unimplemented!()
    }

    /// Register a listener with this group, all listeners are removed when it's fired.
    pub fn add_disposing_listener(
        &self,
        dispatcher: IEventDispatcher,
        type_: String,
        listener: Listener,
    ) {
        // self.add_listener(dispatcher, type_, Box::new(|event: Event| {
        //     self.dispose();
        //     listener(event);
        // }));
        unimplemented!()
    }
}

impl Disposable for EventGroup {
    /// Detach all listeners registered with this group.
    fn dispose(&self) {
        // for entry in self.entries {
        //     entry.dispatcher.removeEventListener(entry.type_, entry.listener, false);
        // }
        // self.entries = Vec::new();
        unimplemented!()
    }
}

pub struct Entry {
    pub dispatcher: IEventDispatcher,
    pub type_: String,
    pub listener: Listener,
}

impl Entry {
    pub fn new(dispatcher: IEventDispatcher, type_: String, listener: Listener) -> Self {
        Self {
            dispatcher,
            type_,
            listener,
        }
    }
}
