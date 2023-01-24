use std::marker::PhantomData;

use digest::ExtendableOutput;

pub trait LtHash {
    fn insert(&mut self, element: impl AsRef<[u8]>);
    fn remove(&mut self, element: impl AsRef<[u8]>);
    fn checksum(&self) -> Vec<u8>;
}

pub struct LtHash16<T: ExtendableOutput + Default> {
    checksum: [u8; 2048],
    hasher: PhantomData<T>,
}

impl<T> LtHash16<T>
where
    T: ExtendableOutput + Default,
{
    pub fn as_bytes(&self) -> &[u8; 2048] {
        &self.checksum
    }

    fn hash_object(&mut self, object: impl AsRef<[u8]>) -> [u8; 2048] {
        let mut output = [0u8; 2048];
        T::digest_xof(object, output.as_mut());
        output
    }
}

impl<T> LtHash for LtHash16<T>
where
    T: ExtendableOutput + Default,
{
    fn insert(&mut self, element: impl AsRef<[u8]>) {
        let hashed = self.hash_object(element);
        for (i, elem) in hashed.into_iter().enumerate() {
            self.checksum[i] = self.checksum[i].wrapping_add(elem);
        }
    }

    fn remove(&mut self, element: impl AsRef<[u8]>) {
        let hashed = self.hash_object(element);
        for (i, elem) in hashed.into_iter().enumerate() {
            self.checksum[i] = self.checksum[i].wrapping_sub(elem);
        }
    }

    fn checksum(&self) -> Vec<u8> {
        self.checksum.to_vec()
    }
}
