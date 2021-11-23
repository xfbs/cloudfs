//use bytes::Bytes;
//use crate::hash::Hash;
use std::error::Error;

#[async_trait::async_trait]
trait FilesystemRead<E: Error> {}

#[async_trait::async_trait]
trait FilesystemWrite<E: Error> {}
