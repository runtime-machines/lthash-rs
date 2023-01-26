use std::marker::PhantomData;

use digest::ExtendableOutput;

use crate::{
    utils::{read_u32, HexDisplayRef32},
    LtHash,
};

/// A LtHash checksum with 32 bits per chunk and 1024 chunks.
#[derive(Clone, Copy)]
pub struct LtHash32<H> {
    checksum: [u32; 1024],
    hasher: PhantomData<H>,
}

// Ensure we don't accidentally remove Send/Sync, since LtHash32 should be Send/Sync.
static_assertions::assert_impl_all!(LtHash32<()>: Send, Sync, Unpin);

impl<H> LtHash32<H>
where
    H: ExtendableOutput + Default,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    fn hash_object(&mut self, object: impl AsRef<[u8]>) -> [u8; 4096] {
        let mut output = [0u8; 4096];
        H::digest_xof(object, output.as_mut());
        output
    }

    #[inline(always)]
    fn display_hex_ref(&self) -> HexDisplayRef32<'_> {
        HexDisplayRef32(&self.checksum[..])
    }
}

impl<H> Default for LtHash32<H>
where
    H: ExtendableOutput + Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self {
            checksum: [0; 1024],
            hasher: Default::default(),
        }
    }
}

impl<H> LtHash for LtHash32<H>
where
    H: ExtendableOutput + Default,
{
    /// Inserts an element to LtHash, actually it generates the hash (of size 4096 bytes) of the object and sums it to the checksum.
    fn insert(&mut self, element: impl AsRef<[u8]>) {
        let hashed = self.hash_object(element);
        let mut i = 0;
        while i < 4096 {
            let xi = &self.checksum[i / 4];
            let yi = &hashed[i..i + 4];
            let yi = read_u32(yi);
            let sum = xi.wrapping_add(yi);
            self.checksum[i / 4] = sum;
            i += 4;
        }
    }

    /// Removes an element to LtHash, actually it generates the hash (of size 4096 bytes) of the object and removes it from the checksum.
    fn remove(&mut self, element: impl AsRef<[u8]>) {
        let hashed = self.hash_object(element);
        let mut i = 0;
        while i < 4096 {
            let xi = &self.checksum[i / 4];
            let yi = &hashed[i..i + 4];
            let yi = read_u32(yi);
            let diff = xi.wrapping_sub(yi);
            self.checksum[i / 4] = diff;
            i += 4;
        }
    }

    /// Provides the hex value as String of the checksum.
    fn to_hex_string(&self) -> String {
        self.display_hex_ref().to_string()
    }

    /// Takes the union of `self` and `rhs`
    ///
    /// Equivalent to cloning `self`, then adding all the objects in `rhs`.
    ///
    /// Equivalent to `self | other`
    ///
    /// # Examples
    /// ```
    /// # use lthash_rs::LtHash;
    /// # use lthash_rs::LtHash32;
    /// # use sha3::Shake256;
    /// # use std::iter::FromIterator;
    /// let mut left = LtHash32::<Shake256>::new();
    /// left.insert("hello");
    ///
    /// let mut right = LtHash32::<Shake256>::new();
    /// right.insert("world");
    ///
    /// assert_eq!(left.union(&right), LtHash32::<Shake256>::from_iter(&["hello", "world"]));
    /// ```
    fn union(&self, rhs: &Self) -> Self {
        let mut checksum = [0; 1024];

        for (checksum, (&lhs, &rhs)) in checksum
            .iter_mut()
            .zip(self.checksum.iter().zip(rhs.checksum.iter()))
        {
            *checksum = lhs.wrapping_add(rhs);
        }

        Self {
            checksum,
            hasher: PhantomData,
        }
    }

    /// Takes the difference of `self` and `rhs`.
    ///
    /// Equivalent to cloning `self`, then removing all the objects in `rhs`.
    ///
    /// Equivalent to `self - other`
    ///
    /// # Examples
    /// ```
    /// # use lthash_rs::LtHash;
    /// # use lthash_rs::LtHash32;
    /// # use sha3::Shake256;
    /// # use std::iter::FromIterator;
    /// let mut left = LtHash32::<Shake256>::new();
    /// left.extend(&["hello", "world"]);
    ///
    /// let mut right = LtHash32::<Shake256>::new();
    /// right.insert("hello");
    ///
    /// assert_eq!(left.difference(&right), LtHash32::from_iter(&["world"]));
    /// ```
    fn difference(&self, rhs: &Self) -> Self {
        let mut checksum = [0; 1024];

        for (checksum, (&lhs, &rhs)) in checksum
            .iter_mut()
            .zip(self.checksum.iter().zip(rhs.checksum.iter()))
        {
            *checksum = lhs.wrapping_sub(rhs);
        }

        Self {
            checksum,
            hasher: PhantomData,
        }
    }

    /// Clears the internal checksum
    fn reset(&mut self) {
        self.checksum.fill(0);
    }

    /// Converts self into the inner list of bytes
    fn into_bytes(self) -> Vec<u8> {
        let Self {
            mut checksum,
            hasher: _,
        } = self;

        // pessimization for big endian platforms, byte swapping is required because the words are currently in big endian order and need to be reversed.
        if cfg!(target_endian = "big") {
            for elem in &mut checksum {
                *elem = elem.swap_bytes();
            }
        }

        bytemuck::bytes_of(&checksum).to_vec()
    }
}

impl<H> TryFrom<&[u8]> for LtHash32<H> {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 4096 {
            return Err(String::from("Wrong number of bytes."));
        }

        let mut checksum = [0; 1024];

        for (checksum, bytes) in checksum.iter_mut().zip(bytes.chunks_exact(4))
        {
            *checksum =
                u32::from_le_bytes(bytes.try_into().map_err(|_| {
                    String::from("Error converting bytes to u32.")
                })?);
        }

        Ok(Self {
            checksum,
            hasher: PhantomData,
        })
    }
}

impl<A, H> Extend<A> for LtHash32<H>
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

impl<H> PartialEq for LtHash32<H> {
    fn eq(&self, other: &Self) -> bool {
        subtle::ConstantTimeEq::ct_eq(&self.checksum[..], &other.checksum[..])
            .into()
    }
}

impl<H> core::fmt::Debug for LtHash32<H> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "LtHash32 {:?}", &self.checksum)
    }
}

impl<A, H> FromIterator<A> for LtHash32<H>
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

impl<'a, H> std::ops::BitOr for &'a LtHash32<H>
where
    H: ExtendableOutput + Default,
{
    type Output = LtHash32<H>;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl<H> std::ops::BitOr for LtHash32<H>
where
    H: ExtendableOutput + Default,
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(&rhs)
    }
}

impl<'a, H> std::ops::Sub for &'a LtHash32<H>
where
    H: ExtendableOutput + Default,
{
    type Output = LtHash32<H>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl<H> std::ops::Sub for LtHash32<H>
where
    H: ExtendableOutput + Default,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(&rhs)
    }
}