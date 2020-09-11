//! [Hash](crate::blob::Hash)

/// A [Hash](crate::Hash) referencing a Blob
pub type Hash = crate::Hash<Blob>;

#[doc(hidden)]
pub struct Blob;
