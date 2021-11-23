use bytes::Bytes;

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
