//! [Hash](tree::Hash), [Tree]

#![allow(dead_code)] // XXX

use crate::*;

use std::collections::*;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;



/// A [Hash](crate::Hash) referencing a [Tree]
pub type Hash = crate::Hash<Tree>;

/// A parsed git tree (~directory)
/// 
/// # Example
///
/// ```rust
/// # use clgit::*;
/// // The initial commit of this project's git repository
/// let repository  = Repository::from_regular_repository(".").unwrap();
/// let hcommit     = commit::Hash::from_str("02c4f0499bcf979ad86d9ef5b61ffc51b1394bef").unwrap();
/// let htree       = tree  ::Hash::from_str("88824f5315abd219d2f6f5f68fe69f32386ffc00").unwrap();
/// let hgitignore  = blob  ::Hash::from_str("96ef6c0b944e24fc22f51f18136cd62ffd5b0b8f").unwrap();
///
/// let tree = Tree::read(&repository, &htree).unwrap();
/// assert_eq!(tree.hash, htree);
///
/// let gitignore = tree.entries.get(".gitignore").unwrap();
/// assert_eq!(gitignore.permissions,   "100644".parse().unwrap());
/// assert_eq!(gitignore.hash,          hgitignore);
/// assert_eq!(gitignore.name,          ".gitignore");
/// ```
pub struct Tree {
    /// The [Hash](tree::Hash) referencing this [Tree]
    pub hash:           Hash,

    /// A dictionary of references to [Tree]s or Blobs and their associated [Name]s and [Permissions]
    pub entries:        BTreeMap<Name, Entry>,

    _non_exhaustive:    (),
}

impl Tree {
    /// [Read] a local [Tree] from a given [Repository]
    ///
    /// [Read]:         std::io::Read
    pub fn read(repository: &Repository, hash: &Hash) -> io::Result<Self> {
        let mut reader = BufReader::new(repository.cat_file_tree(&hash)?);
        let mut out = Tree {
            hash:               hash.clone(),
            entries:            Default::default(),
            _non_exhaustive:    (),
        };
        loop {
            let mut permissions = Vec::new();
            let read = reader.read_until(b' ', &mut permissions)?;
            if read == 0 { break }
            if permissions.pop() != Some(b' ') { return Err(io::Error::new(io::ErrorKind::InvalidData, "file permissions not space terminated in tree")); }
            let permissions = Permissions(Name::from(permissions));
            
            let mut name = Vec::new();
            reader.read_until(b'\0', &mut name)?;
            if name.pop() != Some(b'\0') { return Err(io::Error::new(io::ErrorKind::InvalidData, "file name not nul terminated in tree")); }
            let name = Name::from(name);

            let hash = crate::Hash::read_sha1(&mut reader)?;

            out.entries.insert(name.clone(), Entry {
                permissions,
                hash,
                name,
                _non_exhaustive: ()
            });
        }
        Ok(out)
    }
}



/// A [Tree] entry (e.g. { [Permissions], [Hash](crate::Hash), [Name], .. })
#[derive(Debug)]
pub struct Entry {
    /// [Permissions] for a given file or directory (typically "100644" for files or "040000" for trees)
    pub permissions:    Permissions,

    /// A [Hash](crate::Hash) referencing the contents of [Tree] or Blob
    pub hash:           crate::Hash<()>,

    /// A [Name] describing this [Tree] or Blob (e.g. ".gitignore", ".vscode", ...)
    pub name:           Name,

    _non_exhaustive:    ()
}



/// [Tree] [Entry] permissions (typically "100644" for files or "040000" for trees)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Permissions(Name);

impl FromStr for Permissions {
    type Err = io::Error;
    fn from_str(s: &str) -> io::Result<Self> { Ok(Self(Name::from(s))) }
}
