use lru::LruCache;
use super::{StorageRead, StorageReadError, Context};
use bytes::Bytes;
use crate::hash::Hash;
use async_trait::async_trait;
use tokio::sync::Mutex;
use std::sync::Arc;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Key {
    Block(Hash),
    Chunk(Hash),
    Tree(Hash),
    Entry(Hash),
    Head(Hash),
}

pub struct LruStorage<S: StorageRead> {
    storage: S,
    cache: Arc<Mutex<LruCache<Key, Bytes>>>,
}

impl<S: StorageRead> LruStorage<S> {
    pub fn new(storage: S, capacity: usize) -> Self {
        LruStorage {
            cache: Arc::new(Mutex::new(LruCache::new(capacity))),
            storage,
        }
    }
}

#[async_trait]
impl<S: StorageRead + Sync + Send> StorageRead for LruStorage<S> {
    type Err = S::Err;

    async fn chunk_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        let mut cache = self.cache.lock().await;
        let key = Key::Chunk(hash.clone());
        if let Some(bytes) = cache.get(&key) {
            Ok(Some(bytes.clone()))
        } else {
            match self.storage.chunk_get(context, hash).await {
                Ok(Some(bytes)) => {
                    cache.put(key, bytes.clone());
                    Ok(Some(bytes))
                },
                other => other,
            }
        }
    }

    async fn block_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        let mut cache = self.cache.lock().await;
        let key = Key::Block(hash.clone());
        if let Some(bytes) = cache.get(&key) {
            Ok(Some(bytes.clone()))
        } else {
            match self.storage.block_get(context, hash).await {
                Ok(Some(bytes)) => {
                    cache.put(key, bytes.clone());
                    Ok(Some(bytes))
                },
                other => other,
            }
        }
    }

    async fn entry_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        let mut cache = self.cache.lock().await;
        let key = Key::Entry(hash.clone());
        if let Some(bytes) = cache.get(&key) {
            Ok(Some(bytes.clone()))
        } else {
            match self.storage.entry_get(context, hash).await {
                Ok(Some(bytes)) => {
                    cache.put(key, bytes.clone());
                    Ok(Some(bytes))
                },
                other => other,
            }
        }
    }

    async fn tree_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        let mut cache = self.cache.lock().await;
        let key = Key::Tree(hash.clone());
        if let Some(bytes) = cache.get(&key) {
            Ok(Some(bytes.clone()))
        } else {
            match self.storage.tree_get(context, hash).await {
                Ok(Some(bytes)) => {
                    cache.put(key, bytes.clone());
                    Ok(Some(bytes))
                },
                other => other,
            }
        }
    }

    async fn head_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        let mut cache = self.cache.lock().await;
        let key = Key::Head(hash.clone());
        if let Some(bytes) = cache.get(&key) {
            Ok(Some(bytes.clone()))
        } else {
            match self.storage.head_get(context, hash).await {
                Ok(Some(bytes)) => {
                    cache.put(key, bytes.clone());
                    Ok(Some(bytes))
                },
                other => other,
            }
        }
    }
}
