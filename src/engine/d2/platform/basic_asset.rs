use std::{cell::RefCell, marker::PhantomData};

use crate::engine::d2::{
    asset::Asset,
    util::{Disposable, Value},
};

#[derive(Clone, Debug)]
struct BasicAssetProps {
    pub disposed: bool,
    reload_count: Value<usize>,
}

#[derive(Clone, Debug)]
pub struct BasicAsset<A> {
    props: RefCell<BasicAssetProps>,
    marker: PhantomData<A>,
}

impl<A> BasicAsset<A> {
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn assert_not_disposed(&self) {
        let props = self.props.borrow();
        assert!(!props.disposed, "Asset cannot be used after being disposed");
    }

    pub fn reload(&self, asset: A) {
        let mut props = self.props.borrow_mut();
        self.dispose();
        props.disposed = false;
        self.copy_from(asset);
        let reload_count = *props.reload_count.get();
        props.reload_count.set(reload_count + 1);
    }

    /// Fully copy the content from another asset type, for reloading.
    pub fn copy_from(&self, asset: A) {
        panic!("See subclasses");
    }

    /// Handle disposing.
    pub fn on_disposed(&self) {
        panic!("See subclasses");
    }
}

impl<A> Default for BasicAsset<A> {
    fn default() -> Self {
        Self {
            props: RefCell::new(BasicAssetProps {
                disposed: false,
                reload_count: Value::<usize>::new(0, None),
            }),
            marker: PhantomData,
        }
    }
}

impl<A> Asset for BasicAsset<A> {
    // Overridden in subclasses!
    fn reload_count(&self) -> usize {
        let props = self.props.borrow();
        *props.reload_count.get()
    }
}

impl<A> Disposable for BasicAsset<A> {
    fn dispose(&self) {
        let mut props = self.props.borrow_mut();
        if !props.disposed {
            props.disposed = true;
            self.on_disposed();
        }
    }
}
