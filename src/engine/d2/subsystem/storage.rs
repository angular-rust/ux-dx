pub trait StorageItem {}

/// A simple key/value store that persists between sessions.
pub trait StorageSystem {
    /// True if the environment supports persisted storage. Otherwise, the storage is backed by a
    /// Map and not actually persisted between sessions.
    fn is_supported(&self) -> bool;

    /// Add a key to the storage, replacing any existing value.
    /// @param value An object that can be serialized with Serializer.
    /// @returns True if the value was successfully serialized and persisted.
    fn set<A: StorageItem>(&self, key: String, value: A) -> bool
    where
        Self: Sized;

    /// Retrieve a value from storage for a given key.
    fn get<A: StorageItem>(&self, key: String) -> Option<&A>
    where
        Self: Sized;

    /// Deletes a key/value pair from storage.
    fn remove(&mut self, key: String);

    /// Clears the entire storage contents.
    fn clear(&mut self);
}
