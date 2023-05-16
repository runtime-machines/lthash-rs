use std::marker::PhantomData;

use digest::ExtendableOutput;

use crate::{
    utils::{into_bytes, read_u16, HexDisplayRef16},
    LtHash,
};

/// A LtHash checksum with 16 bits per chunk and 1024 chunks.
#[derive(Clone, Copy)]
pub struct LtHash16<H> {
    pub(crate) checksum: [u16; 1024],
    hasher: PhantomData<H>,
}

// Ensure we don't accidentally remove Send/Sync, since LtHash16 should be Send/Sync.
static_assertions::assert_impl_all!(LtHash16<()>: Send, Sync, Unpin);

impl<H> LtHash16<H> {
    pub(crate) const fn name(&self) -> &'static str {
        "LtHash16"
    }
}

impl<H> LtHash16<H>
where
    H: ExtendableOutput + Default,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    fn hash_object(&mut self, object: impl AsRef<[u8]>) -> [u8; 2048] {
        let mut output = [0u8; 2048];
        H::digest_xof(object, output.as_mut());
        output
    }

    #[inline(always)]
    fn display_hex_ref(&self) -> HexDisplayRef16<'_> {
        HexDisplayRef16(&self.checksum[..])
    }
}

impl<H> Default for LtHash16<H>
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

impl<H> LtHash for LtHash16<H>
where
    H: ExtendableOutput + Default,
{
    /// Inserts an element to LtHash, actually it generates the hash (of size 2048 bytes) of the object and sums it to the checksum.
    fn insert(&mut self, element: impl AsRef<[u8]>) {
        let hashed = self.hash_object(element);
        let mut i = 0;
        while i < 2048 {
            let xi = &self.checksum[i / 2];
            let yi = &hashed[i..i + 2];
            let yi = read_u16(yi);
            let sum = xi.wrapping_add(yi);
            self.checksum[i / 2] = sum;
            i += 2;
        }
    }

    /// Removes an element to LtHash, actually it generates the hash (of size 2048 bytes) of the object and removes it from the checksum.
    fn remove(&mut self, element: impl AsRef<[u8]>) {
        let hashed = self.hash_object(element);
        let mut i = 0;
        while i < 2048 {
            let xi = &self.checksum[i / 2];
            let yi = &hashed[i..i + 2];
            let yi = read_u16(yi);
            let diff = xi.wrapping_sub(yi);
            self.checksum[i / 2] = diff;
            i += 2;
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
    /// # use lthash_rs::LtHash16;
    /// # use sha3::Shake256;
    /// # use std::iter::FromIterator;
    /// let mut left = LtHash16::<Shake256>::new();
    /// left.insert("hello");
    ///
    /// let mut right = LtHash16::<Shake256>::new();
    /// right.insert("world");
    ///
    /// assert_eq!(left.union(&right), LtHash16::<Shake256>::from_iter(&["hello", "world"]));
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
    /// # use lthash_rs::LtHash16;
    /// # use sha3::Shake256;
    /// # use std::iter::FromIterator;
    /// let mut left = LtHash16::<Shake256>::new();
    /// left.extend(&["hello", "world"]);
    ///
    /// let mut right = LtHash16::<Shake256>::new();
    /// right.insert("hello");
    ///
    /// assert_eq!(left.difference(&right), LtHash16::from_iter(&["world"]));
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
        into_bytes(self.checksum)
    }
}

impl<H> TryFrom<&[u8]> for LtHash16<H> {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 2048 {
            return Err(String::from("Wrong number of bytes."));
        }

        let mut checksum = [0; 1024];

        for (checksum, bytes) in checksum.iter_mut().zip(bytes.chunks_exact(2))
        {
            *checksum =
                u16::from_le_bytes(bytes.try_into().map_err(|_| {
                    String::from("Error converting bytes to u16.")
                })?);
        }

        Ok(Self {
            checksum,
            hasher: PhantomData,
        })
    }
}
