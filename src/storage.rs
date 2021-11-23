use crate::hash::Hash;
use bytes::Bytes;
use std::error::Error as StdError;
use thiserror::Error;
use async_trait::async_trait;

#[cfg(feature = "storage-lru")]
pub mod lru;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum HashAlgorithm {
    Sha512,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Context {
    hash_algorithm: HashAlgorithm,
    chunk_size: usize,
    block_size: usize,
    entry_size: usize,
    tree_size: usize,
}

#[derive(Error, Clone, Debug)]
pub enum StorageReadError<E: StdError> {
    #[error("Got data with an invalid length")]
    InvalidLength,
    #[error("Got data with an invalid hash")]
    InvalidHash,
    #[error("Other error has occured: {0:}")]
    Other(E),
}

#[async_trait]
pub trait StorageRead {
    /// Errors from the underlying storage that can be produced.
    type Err: StdError;

    /// Retrieve a chunk.
    async fn chunk_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>>;

    /// Retrieve a block.
    async fn block_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>>;

    /// Retrieve an entry.
    async fn entry_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>>;

    /// Retrieve a tree.
    async fn tree_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>>;

    /// Retrieve the current head.
    async fn head_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>>;
}

#[async_trait::async_trait]
trait StorageWrite<E: StdError> {}
