use std::{future::Future, pin::Pin};

pub use self::kvs::KvStore;
pub use self::sled::SledKvsEngine;
use crate::{Result};

mod kvs;
mod sled;

/// Trait for a key value storage engine.
/// box dyn future 需要加上Pin才能await
pub trait KvsEngine: Clone + Send + 'static {
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    fn set(&self, key: String, value: String) -> Pin<Box<dyn Future<Output = Result<()>>>>;

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    fn get(&self, key: String) -> Pin<Box<dyn Future<Output = Result<Option<String>>>>>;

    /// Removes a given key.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::KeyNotFound` if the given key is not found.
    fn remove(&self, key: String) -> Pin<Box<dyn Future<Output = Result<()>>>>;
}