use super::{Context, StorageRead, StorageReadError};
use crate::hash::Hash;
use async_trait::async_trait;
use bytes::Bytes;
use reqwest::{Client, Error as ReqwestError, StatusCode, Url};
use thiserror::Error;
use url::ParseError;

pub struct HttpStorage {
    client: Client,
    url: Url,
}

#[derive(Error, Debug)]
pub enum HttpStorageError {
    #[error("Error while performing HTTP request: {0:}")]
    Reqwest(#[from] ReqwestError),
    #[error("Error while parsing URL: {0:}")]
    UrlParse(#[from] ParseError),
    #[error("Error response")]
    Response(StatusCode),
}

impl HttpStorage {
    pub fn new(client: Client, url: Url) -> Self {
        HttpStorage { client, url }
    }

    pub async fn fetch(&self, kind: &str, hash: &Hash) -> Result<Option<Bytes>, HttpStorageError> {
        let url = self
            .url
            .join(&format!("/{}/{:x}", kind, hash.0))?;
        let response = self
            .client
            .get(url)
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => Ok(Some(response.bytes().await?)),
            StatusCode::NO_CONTENT => Ok(None),
            status => Err(HttpStorageError::Response(status)),
        }
    }
}

#[async_trait]
impl StorageRead for HttpStorage {
    type Err = HttpStorageError;

    async fn chunk_get(
        &self,
        _context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        match self.fetch("chunk", hash).await {
            Ok(other) => Ok(other),
            Err(e) => Err(StorageReadError::Other(e)),
        }
    }

    async fn block_get(
        &self,
        _context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        match self.fetch("block", hash).await {
            Ok(other) => Ok(other),
            Err(e) => Err(StorageReadError::Other(e)),
        }
    }

    async fn entry_get(
        &self,
        _context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        match self.fetch("entry", hash).await {
            Ok(other) => Ok(other),
            Err(e) => Err(StorageReadError::Other(e)),
        }
    }

    async fn tree_get(
        &self,
        _context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        match self.fetch("tree", hash).await {
            Ok(other) => Ok(other),
            Err(e) => Err(StorageReadError::Other(e)),
        }
    }

    async fn head_get(
        &self,
        _context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        match self.fetch("head", hash).await {
            Ok(other) => Ok(other),
            Err(e) => Err(StorageReadError::Other(e)),
        }
    }
}
