//! [Hash](crate::blob::Hash)

/// A [Hash](crate::generic::Hash) referencing a Blob
pub type Hash = crate::generic::Hash<Blob>;

#[doc(hidden)]
pub struct Blob;
