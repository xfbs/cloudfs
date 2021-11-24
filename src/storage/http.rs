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
}

#[async_trait]
impl StorageRead for HttpStorage {
    type Err = HttpStorageError;

    async fn chunk_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        let url = self
            .url
            .join(&format!("/chunk/{:x}", hash.0))
            .map_err(|e| StorageReadError::Other(HttpStorageError::UrlParse(e)))?;
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| StorageReadError::Other(HttpStorageError::Reqwest(e)))?;
        match response.status() {
            StatusCode::OK => {
                Ok(Some(response.bytes().await.map_err(|e| {
                    StorageReadError::Other(HttpStorageError::Reqwest(e))
                })?))
            }
            StatusCode::NO_CONTENT => Ok(None),
            status => Err(StorageReadError::Other(HttpStorageError::Response(status))),
        }
    }

    async fn block_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        let url = self
            .url
            .join(&format!("/block/{:x}", hash.0))
            .map_err(|e| StorageReadError::Other(HttpStorageError::UrlParse(e)))?;
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| StorageReadError::Other(HttpStorageError::Reqwest(e)))?;
        match response.status() {
            StatusCode::OK => {
                Ok(Some(response.bytes().await.map_err(|e| {
                    StorageReadError::Other(HttpStorageError::Reqwest(e))
                })?))
            }
            StatusCode::NO_CONTENT => Ok(None),
            status => Err(StorageReadError::Other(HttpStorageError::Response(status))),
        }
    }

    async fn entry_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        let url = self
            .url
            .join(&format!("/entry/{:x}", hash.0))
            .map_err(|e| StorageReadError::Other(HttpStorageError::UrlParse(e)))?;
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| StorageReadError::Other(HttpStorageError::Reqwest(e)))?;
        match response.status() {
            StatusCode::OK => {
                Ok(Some(response.bytes().await.map_err(|e| {
                    StorageReadError::Other(HttpStorageError::Reqwest(e))
                })?))
            }
            StatusCode::NO_CONTENT => Ok(None),
            status => Err(StorageReadError::Other(HttpStorageError::Response(status))),
        }
    }

    async fn tree_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        let url = self
            .url
            .join(&format!("/tree/{:x}", hash.0))
            .map_err(|e| StorageReadError::Other(HttpStorageError::UrlParse(e)))?;
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| StorageReadError::Other(HttpStorageError::Reqwest(e)))?;
        match response.status() {
            StatusCode::OK => {
                Ok(Some(response.bytes().await.map_err(|e| {
                    StorageReadError::Other(HttpStorageError::Reqwest(e))
                })?))
            }
            StatusCode::NO_CONTENT => Ok(None),
            status => Err(StorageReadError::Other(HttpStorageError::Response(status))),
        }
    }

    async fn head_get(
        &self,
        context: &Context,
        hash: &Hash,
    ) -> Result<Option<Bytes>, StorageReadError<Self::Err>> {
        let url = self
            .url
            .join(&format!("/head/{:x}", hash.0))
            .map_err(|e| StorageReadError::Other(HttpStorageError::UrlParse(e)))?;
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| StorageReadError::Other(HttpStorageError::Reqwest(e)))?;
        match response.status() {
            StatusCode::OK => {
                Ok(Some(response.bytes().await.map_err(|e| {
                    StorageReadError::Other(HttpStorageError::Reqwest(e))
                })?))
            }
            StatusCode::NO_CONTENT => Ok(None),
            status => Err(StorageReadError::Other(HttpStorageError::Response(status))),
        }
    }
}
