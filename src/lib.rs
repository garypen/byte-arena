use std::cmp::min;

use bytes::{Bytes, BytesMut};
use thiserror::Error;
use typed_arena::Arena;
use typed_builder::TypedBuilder;
#[cfg(test)]
mod test;

/// A Byte Arena
#[derive(Default, TypedBuilder)]
pub struct ByteArena {
    #[builder(default, setter(skip))]
    arena: Arena<Bytes>,
    #[builder(default=usize::MAX)]
    limit: usize,
    #[builder(default=usize::MAX)]
    size: usize,
}

/// A Byte Arena Error
#[derive(Debug, Error, PartialEq)]
pub enum ByteArenaError {
    #[error("Arena block limit reached")]
    /// Arena block limit reached
    TooManyBlocks,

    #[error("Arena block too large")]
    /// Arena block too large
    BlockTooLarge,
}

impl ByteArena {
    /// Utility function which allocates a correctly sized BytesMut
    pub fn alloc(&mut self) -> BytesMut {
        BytesMut::with_capacity(self.size)
    }

    /// Append some Bytes into our Arena. Enforce limit and size constraints
    ///
    /// Return a mutable reference to the allocated bytes
    pub fn append<B: Into<Bytes>>(&mut self, value: B) -> Result<&mut Bytes, ByteArenaError> {
        if self.arena.len() == self.limit {
            return Err(ByteArenaError::TooManyBlocks);
        };

        let block = value.into();

        if block.len() > self.size {
            return Err(ByteArenaError::BlockTooLarge);
        }

        Ok(self.arena.alloc(block))
    }
}

// Helpful converter in case we want a single Bytes from our arena.
// Note: We limit initial capacity to 1MB since we may have some big arenas.
impl From<ByteArena> for Bytes {
    fn from(mut value: ByteArena) -> Self {
        let capacity = min(value.arena.len() * value.size, 1_024_000);
        let mut target = BytesMut::with_capacity(capacity);
        // It's ok to iter_mut() since we have taken ownership of the value
        for block in value.arena.iter_mut() {
            target.extend_from_slice(block);
        }
        target.freeze()
    }
}

// Helpful converter in case we want a Vec of Bytes from our arena.
impl From<ByteArena> for Vec<Bytes> {
    fn from(value: ByteArena) -> Self {
        value.arena.into_vec()
    }
}
