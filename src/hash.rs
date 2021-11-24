use bytes::{Buf, Bytes};
use digest::Digest;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum HashAlgorithm {
    #[cfg(feature = "hash-sha2")]
    Sha512,
    #[cfg(feature = "hash-sha2")]
    Sha256,
    #[cfg(feature = "hash-sha3")]
    Sha3_256,
    #[cfg(feature = "hash-sha3")]
    Sha3_512,
    #[cfg(feature = "hash-blake2")]
    Blake2b,
    #[cfg(feature = "hash-blake2")]
    Blake2s,
}

fn hash_data<H: Digest, B: Buf>(mut data: B) -> Hash {
    let mut hasher = H::new();
    while data.remaining() > 0 {
        let slice = data.chunk();
        hasher.update(slice);
        let amount = slice.len();
        data.advance(amount);
    }
    let digest = hasher.finalize();
    Hash(Bytes::copy_from_slice(&digest))
}

impl HashAlgorithm {
    pub fn hash<B: Buf>(self, data: B) -> Hash {
        use HashAlgorithm::*;
        match self {
            #[cfg(feature = "hash-sha2")]
            Sha512 => hash_data::<sha2::Sha512, B>(data),
            #[cfg(feature = "hash-sha2")]
            Sha256 => hash_data::<sha2::Sha256, B>(data),
            #[cfg(feature = "hash-sha3")]
            Sha3_256 => hash_data::<sha3::Sha3_256, B>(data),
            #[cfg(feature = "hash-sha3")]
            Sha3_512 => hash_data::<sha3::Sha3_512, B>(data),
            #[cfg(feature = "hash-blake2")]
            Blake2b => hash_data::<blake2::Blake2b, B>(data),
            #[cfg(feature = "hash-blake2")]
            Blake2s => hash_data::<blake2::Blake2s, B>(data),
        }
    }
}

#[cfg(feature = "hash-sha2")]
#[test]
fn test_hash_sha512() {
    let alg = HashAlgorithm::Sha512;
    let hash = alg.hash(&b"hello"[..]);
    assert_eq!(
        &hash.to_string()[0..50],
        "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2"
    );
}

#[cfg(feature = "hash-sha2")]
#[test]
fn test_hash_sha256() {
    let alg = HashAlgorithm::Sha256;
    let hash = alg.hash(&b"hello"[..]);
    assert_eq!(
        &hash.to_string()[0..50],
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73"
    );
}

#[cfg(feature = "hash-sha3")]
#[test]
fn test_hash_sha3_512() {
    let alg = HashAlgorithm::Sha3_512;
    let hash = alg.hash(&b"hello"[..]);
    assert_eq!(
        &hash.to_string()[0..50],
        "75d527c368f2efe848ecf6b073a36767800805e9eef2b1857d"
    );
}

#[cfg(feature = "hash-sha3")]
#[test]
fn test_hash_sha3_256() {
    let alg = HashAlgorithm::Sha3_256;
    let hash = alg.hash(&b"hello"[..]);
    assert_eq!(
        &hash.to_string()[0..50],
        "3338be694f50c5f338814986cdf0686453a888b84f424d792a"
    );
}

#[cfg(feature = "hash-blake2")]
#[test]
fn test_hash_blake2b() {
    let alg = HashAlgorithm::Blake2b;
    let hash = alg.hash(&b"hello"[..]);
    assert_eq!(
        &hash.to_string()[0..50],
        "e4cfa39a3d37be31c59609e807970799caa68a19bfaa15135f"
    );
}

#[cfg(feature = "hash-blake2")]
#[test]
fn test_hash_blake2s() {
    let alg = HashAlgorithm::Blake2s;
    let hash = alg.hash(&b"hello"[..]);
    assert_eq!(
        &hash.to_string()[0..50],
        "19213bacc58dee6dbde3ceb9a47cbb330b3d86f8cca8997eb0"
    );
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash(pub Bytes);

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in &self.0 {
            write!(f, "{:02x}", x)?;
        }
        Ok(())
    }
}

impl Hash {
    pub fn is_null(&self) -> bool {
        self.0.iter().all(|x| *x == 0)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}
