use std::marker::PhantomData;

use digest::ExtendableOutput;

use crate::utils::{read_u16, HexDisplayRef16};

pub trait LtHash {
    fn insert(&mut self, element: impl AsRef<[u8]>);
    fn remove(&mut self, element: impl AsRef<[u8]>);
    fn as_bytes(&self) -> &[u8];
    fn to_hex_string(&self) -> String;
}

/// A LtHash checksum with 16 bits per chunk and 1024 chunks.
pub struct LtHash16<H> {
    checksum: [u16; 1024],
    hasher: PhantomData<H>,
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

    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(&self.checksum)
    }

    fn to_hex_string(&self) -> String {
        self.display_hex_ref().to_string()
    }
}

impl<A, H> Extend<A> for LtHash16<H>
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

impl<H> PartialEq for LtHash16<H> {
    fn eq(&self, other: &Self) -> bool {
        subtle::ConstantTimeEq::ct_eq(&self.checksum[..], &other.checksum[..])
            .into()
    }
}

impl<H> core::fmt::Debug for LtHash16<H> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "LtHash16 {:?}", &self.checksum)
    }
}
