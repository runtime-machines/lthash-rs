mod lthash16;
mod lthash32;
mod utils;

pub use lthash16::*;
pub use lthash32::*;

use digest::ExtendableOutput;

/// Generic trait for LtHash, these functions will be implemented by all the instances of LtHash.
pub trait LtHash {
    /// Inserts an element to LtHash, actually it generates the hash (of size 2048 bytes) of the object and sums it to the checksum.
    fn insert(&mut self, element: impl AsRef<[u8]>);
    /// Removes an element to LtHash, actually it generates the hash (of size 2048 bytes) of the object and removes it from the checksum.
    fn remove(&mut self, element: impl AsRef<[u8]>);
    /// Provides the hex value as String of the checksum.
    fn to_hex_string(&self) -> String;
    /// Takes the union of `self` and `rhs`.
    ///
    /// Equivalent to cloning `self`, then adding all the objects in `rhs`.
    ///
    /// Equivalent to `self | other`.
    fn union(&self, rhs: &Self) -> Self;
    /// Takes the difference of `self` and `rhs`.
    ///
    /// Equivalent to cloning `self`, then removing all the objects in `rhs`.
    ///
    /// Equivalent to `self - other`.
    fn difference(&self, rhs: &Self) -> Self;
    /// Clears the internal checksum.
    fn reset(&mut self);
    /// Converts self into the inner list of bytes.
    fn into_bytes(self) -> Vec<u8>;
}

macro_rules! common {
    ($lthash:ty) => {
        impl<A, H> Extend<A> for $lthash
        where
            A: AsRef<[u8]>,
            H: ExtendableOutput + Default,
        {
            fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
                for item in iter {
                    self.insert(item);
                }
            }
        }

        impl<H> PartialEq for $lthash {
            fn eq(&self, other: &Self) -> bool {
                subtle::ConstantTimeEq::ct_eq(
                    &self.checksum[..],
                    &other.checksum[..],
                )
                .into()
            }
        }

        impl<H> core::fmt::Debug for $lthash {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                write!(f, "{} {:?}", self.name(), &self.checksum)
            }
        }

        impl<A, H> FromIterator<A> for $lthash
        where
            A: AsRef<[u8]>,
            H: ExtendableOutput + Default,
        {
            fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
                let mut this = Self::default();
                this.extend(iter);
                this
            }
        }

        impl<'a, H> std::ops::BitOr for &'a $lthash
        where
            H: ExtendableOutput + Default,
        {
            type Output = $lthash;

            fn bitor(self, rhs: Self) -> Self::Output {
                self.union(rhs)
            }
        }

        impl<H> std::ops::BitOr for $lthash
        where
            H: ExtendableOutput + Default,
        {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                self.union(&rhs)
            }
        }

        impl<'a, H> std::ops::Sub for &'a $lthash
        where
            H: ExtendableOutput + Default,
        {
            type Output = $lthash;

            fn sub(self, rhs: Self) -> Self::Output {
                self.difference(rhs)
            }
        }

        impl<H> std::ops::Sub for $lthash
        where
            H: ExtendableOutput + Default,
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                self.difference(&rhs)
            }
        }
    };
}

common!(LtHash16<H>);
common!(LtHash32<H>);
