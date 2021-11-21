use bytes::Bytes;
use anyhow::Result;
use crate::hash::Hash;

#[async_trait::async_trait]
trait StorageRead {
    fn chunk_get(&self, hash: &Hash) -> Result<Option<Bytes>>;
    fn block_get(&self, hash: &Hash) -> Result<Option<Bytes>>;
    fn entry_get(&self, hash: &Hash) -> Result<Option<Bytes>>;
    fn tree_get(&self, hash: &Hash) -> Result<Option<Bytes>>;
}

#[async_trait::async_trait]
trait StorageWrite {
}
