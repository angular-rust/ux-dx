use crate::engine::d2::math::Math;

/// A pool of reusable objects that can be used to avoid allocation.
///  *
// ```
// // Create a pool and preallocate it with 10 ExpensiveObjects.
// let pool = Pool::new(fn () return ExpensiveObject::new()).setSize(10);
//  *
// let obj = pool.take();
// // ..
// pool.put(obj);
// ```
pub struct Pool<A>
where
    A: Default + Clone,
{
    allocator: Box<dyn Fn() -> Option<A>>,
    free_objects: Vec<A>,
    capacity: usize,
}

impl<A: Default + Clone> Pool<A> {
    /// @param allocator A fn that creates a new object.
    pub fn new(allocator: Box<dyn Fn() -> Option<A>>) -> Self {
        Self {
            allocator,
            free_objects: Vec::new(),
            capacity: Math::INT_MAX as usize,
        }
    }

    /// Take an object from the pool. If the pool is empty, a new object will be allocated.
    ///  *
    /// You should later release the object back into the pool by calling `put()`.
    pub fn take(&mut self) -> Option<A> {
        if self.free_objects.len() > 0 {
            return self.free_objects.pop();
        }
        let object = (self.allocator)();
        assert!(object.is_some());

        object
    }

    /// Put an object into the pool. This should be called to release objects previously claimed with
    /// `take()`. Can also be called to pre-allocate the pool with new objects.
    pub fn put(&mut self, object: A) {
        if self.free_objects.len() < self.capacity {
            self.free_objects.push(object);
        }
    }

    /// Resizes the pool. If the given size is larger than the current number of pooled objects, new
    /// objects are allocated to expand the pool. Otherwise, objects are trimmed out of the pool.
    ///  *
    /// @returns This instance, for chaining.
    pub fn set_size(&mut self, size: usize) -> &Self {
        if self.free_objects.len() > size {
            self.free_objects.resize(size, Default::default());
        } else {
            let needed = size - self.free_objects.len();
            for ii in 0..needed {
                let object = (self.allocator)();
                assert!(object.is_some());
                if let Some(val) = object {
                    self.free_objects.push(val);
                }
            }
        }

        self
    }

    /// Sets the maximum capacity of the pool. By default, the pool can grow to any size.
    ///  *
    /// @returns This instance, for chaining.
    pub fn set_capacity(&mut self, capacity: usize) -> &Self {
        if self.free_objects.len() > capacity {
            self.free_objects.resize(capacity, Default::default());
        }
        self.capacity = capacity;

        self
    }
}
