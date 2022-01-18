/// Utility mixins for bit sets. Designed to be imported with 'using' to supplement regular Ints with
/// bitset functions.
pub trait BitSets {
    /// Adds all the bits included in the mask, and returns the new bitset.
    fn add(&self, mask: u32) -> u32;

    /// Removes all the bits included in the mask, and returns the new bitset.
    //  static
    fn remove(&self, mask: u32) -> u32;

    /// Toggles all the bits included in the mask, and returns the new bitset.
    //  static
    fn toggle(&self, mask: u32) -> u32;

    /// Returns true if the bitset contains ANY of the bits in the given mask.
    //  static
    fn contains(&self, mask: u32) -> bool;

    /// Returns true if the bitset contains ALL of the bits in the given mask.
    //  static
    fn contains_all(&self, mask: u32) -> bool;

    /// Either adds or removes all the bits included in the mask, and returns the new bitset.
    //  static
    fn set(&self, mask: u32, enabled: bool) -> u32;
}

impl BitSets for u32 {
    /// Adds all the bits included in the mask, and returns the new bitset.
    //  static
    #[inline]
    fn add(&self, mask: u32) -> u32 {
        self | mask
    }

    /// Removes all the bits included in the mask, and returns the new bitset.
    //  static
    #[inline]
    fn remove(&self, mask: u32) -> u32 {
        // return self & ~mask;
        unimplemented!()
    }

    /// Toggles all the bits included in the mask, and returns the new bitset.
    //  static
    #[inline]
    fn toggle(&self, mask: u32) -> u32 {
        self ^ mask
    }

    /// Returns true if the bitset contains ANY of the bits in the given mask.
    //  static
    #[inline]
    fn contains(&self, mask: u32) -> bool {
        self & mask != 0
    }

    /// Returns true if the bitset contains ALL of the bits in the given mask.
    //  static
    #[inline]
    fn contains_all(&self, mask: u32) -> bool {
        self & mask == mask
    }

    /// Either adds or removes all the bits included in the mask, and returns the new bitset.
    //  static
    fn set(&self, mask: u32, enabled: bool) -> u32 {
        if enabled {
            self.add(mask)
        } else {
            self.remove(mask)
        }
    }
}
