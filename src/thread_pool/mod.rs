//! This module provides various thread pools. All thread pools should implement
//! the `ThreadPool` trait.

use crate::Result;

/// The trait that all thread pools should implement.
pub trait ThreadPool {
    /// Creates a new thread pool, immediately spawning the specified number of
    /// threads.
    ///
    /// Returns an error if any thread fails to spawn. All previously-spawned threads
    /// are terminated.
    fn new(threads: u32) -> Result<Self>
    where
        Self: Sized;
}
