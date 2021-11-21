use bytes::Bytes;
//use anyhow::Result;
use crate::hash::Hash;
use std::error::Error;

#[async_trait::async_trait]
trait StorageRead<E: Error> {
    fn chunk_get(&self, hash: &Hash) -> Result<Option<Bytes>, E>;
    fn block_get(&self, hash: &Hash) -> Result<Option<Bytes>, E>;
    fn entry_get(&self, hash: &Hash) -> Result<Option<Bytes>, E>;
    fn tree_get(&self, hash: &Hash) -> Result<Option<Bytes>, E>;
}

#[async_trait::async_trait]
trait StorageWrite {
}
