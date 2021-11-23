use bytes::Bytes;
use crate::hash::Hash;
use std::error::Error as StdError;
use thiserror::Error;

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
    tree_size: usize
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

#[async_trait::async_trait]
pub trait StorageRead<E: StdError> {
    async fn chunk_get(&self, context: &Context, hash: &Hash) -> Result<Option<Bytes>, StorageReadError<E>>;
    async fn block_get(&self, context: &Context, hash: &Hash) -> Result<Option<Bytes>, StorageReadError<E>>;
    async fn entry_get(&self, context: &Context, hash: &Hash) -> Result<Option<Bytes>, StorageReadError<E>>;
    async fn tree_get(&self, context: &Context, hash: &Hash) -> Result<Option<Bytes>, StorageReadError<E>>;
    async fn head_get(&self, context: &Context, hash: &Hash) -> Result<Option<Bytes>, StorageReadError<E>>;
}

#[async_trait::async_trait]
trait StorageWrite<E: StdError> {
}
