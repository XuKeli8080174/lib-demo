#![deny(missing_docs)]
//! A simple key/value store.

pub use client::KvsClient;
pub use engines::{KvStore, KvsEngine, SledKvsEngine};
pub use error::{KvsError, Result};
pub use data_struct::{SafeDeque, UnsafeList, LibVec, Arc};

mod client;
mod common;
mod engines;
mod error;
/// 为什么
pub mod server;
pub mod thread_pool;
mod connection;
mod data_struct;
/// interactive with naive library and C header file
pub mod ffi_test;


