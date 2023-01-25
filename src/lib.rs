mod lthash16;
mod utils;

pub use lthash16::*;

/// Generic trait for LtHash, these functions will be implemented by all the instances of LtHash (e.g., if LtHash32 is implemented)
pub trait LtHash {
    /// Inserts an element to LtHash, actually it generates the hash (of size 2048 bytes) of the object and sums it to the checksum.
    fn insert(&mut self, element: impl AsRef<[u8]>);
    /// Removes an element to LtHash, actually it generates the hash (of size 2048 bytes) of the object and removes it from the checksum.
    fn remove(&mut self, element: impl AsRef<[u8]>);
    /// Provides the hex value as String of the checksum.
    fn to_hex_string(&self) -> String;
    /// Takes the union of `self` and `rhs`
    ///
    /// Equivalent to cloning `self`, then adding all the objects in `rhs`.
    ///
    /// Equivalent to `self | other`
    fn union(&self, rhs: &Self) -> Self;
    /// Takes the difference of `self` and `rhs`.
    ///
    /// Equivalent to cloning `self`, then removing all the objects in `rhs`.
    ///
    /// Equivalent to `self - other`
    fn difference(&self, rhs: &Self) -> Self;
    /// Clears the internal checksum
    fn reset(&mut self);
    /// Converts self into the inner list of bytes
    fn into_bytes(self) -> Vec<u8>;
}
