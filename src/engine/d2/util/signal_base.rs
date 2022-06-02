use std::{fmt, rc::Rc};

use crate::engine::d2::platform::Dynamic;

use super::SignalConnection;

struct Generate<T>(fn() -> T);

impl<T> Copy for Generate<T> {}

impl<T> Clone for Generate<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Default, Clone, Debug)]
pub struct SignalBase {
    head: Box<Option<SignalConnection>>,
    deferred_tasks: Option<Rc<Task>>,
}

impl SignalBase {
    // pub const DISPATCHING_SENTINEL: SignalConnection = SignalConnection {
    //     listener: None,
    //     next: None,
    //     signal: None,
    //     stayInList: false,
    // };

    pub fn new(listener: Option<Dynamic>) -> Self {
        // let instance = Self {
        //     head: None,
        //     deferred_tasks: None,
        // };

        // instance.head = if listener.is_some() {
        //     SignalConnection::new(Some(instance), listener)
        // } else {
        //     None
        // };

        // instance
        todo!("should deal with it");
    }

    /// Whether this signal has at least one listener.
    #[inline]
    pub fn has_listeners(&self) -> bool {
        self.head.is_some()
    }

    pub(crate) fn connect_impl(&self, listener: Dynamic, prioritize: bool) -> SignalConnection {
        todo!("should deal with it");
        // let conn = SignalConnection::new(Some(self), listener);
        // if self.dispatching() {
        //     todo!("should deal with it");
        //     // self.defer(Box::new(|| {
        //     //     self.listAdd(conn, prioritize);
        //     // }));
        // } else {
        //     self.listAdd(conn, prioritize);
        // }

        // conn
    }

    pub fn disconnect(&self, conn: &SignalConnection) {
        if self.dispatching() {
            todo!("should deal with it");
            // self.defer(Box::new(|| {
            //     self.listRemove(conn);
            // }));
        } else {
            self.list_remove(conn);
        }
    }

    pub fn defer(&self, function: Box<dyn Fn()>) {
        todo!("should deal with it");
        // let tail = None;
        // let p = self.deferred_tasks;
        // while let Some(ref val) = p {
        //     tail = p;
        //     p = val.next;
        // }

        // let task = Task::new(function);
        // if let Some(ref tail) = tail {
        //     tail.next = Some(task);
        // } else {
        //     self.deferred_tasks = Some(task);
        // }
    }

    pub fn will_emit(&self) -> Option<SignalConnection> {
        // // Should never happen, since the pub emit methods will defer, but just in case..
        // assert!(!self.dispatching());

        // let snapshot = self.head;
        // self.head = Self::DISPATCHING_SENTINEL;

        // snapshot
        todo!("should deal with it");
    }

    pub fn did_emit(&self, head: SignalConnection) {
        // self.head = Some(head);

        // let snapshot = self.deferred_tasks;
        // self.deferred_tasks = None;
        // while let Some(ref val) = snapshot {
        //     (val.func)();
        //     snapshot = val.next;
        // }
        todo!("should deal with it");
    }

    pub fn list_add(&self, conn: SignalConnection, prioritize: bool) {
        todo!("should deal with it");
        // if prioritize {
        //     // Prepend it to the beginning of the list
        //     conn.next = self.head;
        //     self.head = Some(conn);
        // } else {
        //     // Append it to the end of the list
        //     let tail = None;
        //     let p = self.head;
        //     while let Some(ref val) = p {
        //         tail = p;
        //         p = val._next;
        //     }

        //     if let Some(ref tail) = tail {
        //         tail._next = Some(conn);
        //     } else {
        //         self.head = Some(conn);
        //     }
        // }
    }

    pub fn list_remove(&self, conn: &SignalConnection) {
        let prev: Option<SignalConnection> = None;
        let p = &self.head;
        todo!("should deal with it");
        // while let Some(val) = p {
        //     if val == conn {
        //         // Splice out p
        //         let next = val._next;
        //         match prev {
        //             None => self.head = next,
        //             Some(ref prev) => prev.next = next,
        //         }
        //         return;
        //     }
        //     prev = *p;
        //     p = &val._next;
        // }
    }

    #[inline]
    pub fn dispatching(&self) -> bool {
        todo!("should deal with it");
        // self.head == Self::DISPATCHING_SENTINEL
    }
}

#[derive(Default, Clone)]
struct Task {
    pub func: Option<Rc<dyn Fn() + 'static>>,
    pub next: Option<Box<Task>>,
}

// impl Copy for Task { }

impl Task {
    pub fn new(func: Box<dyn Fn()>) -> Self {
        // Self {
        //     func: Some(func),
        //     next: None,
        // }
        todo!("should deal with it");
    }
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task")
            //  .field("x", &self.x)
            //  .field("y", &self.y)
            .finish()
    }
}
