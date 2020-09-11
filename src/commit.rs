//! [Hash](commit::Hash), [Commit]

use crate::*;

use std::io::{self, BufRead, BufReader};



/// A [Hash](generic::Hash) referencing a [Commit]
pub type Hash = generic::Hash<Commit>;

/// A parsed git commit
/// 
/// # Example
///
/// ```rust
/// # use clgit::*;
/// // The initial commit of this project's git repository
/// let repository  = Repository::from_path(".").unwrap();
/// let hcommit     = commit::Hash::from_str("02c4f0499bcf979ad86d9ef5b61ffc51b1394bef").unwrap();
/// let htree       = tree  ::Hash::from_str("88824f5315abd219d2f6f5f68fe69f32386ffc00").unwrap();
///
/// let commit = Commit::read(&repository, &hcommit).unwrap();
/// assert_eq!(commit.hash, hcommit);
/// assert_eq!(commit.tree, htree);
/// assert_eq!(commit.parents.len(), 0); // initial commit
/// ```
pub struct Commit {
    /// The [Hash](commit::Hash) representing this [Commit]
    pub hash:           commit::Hash,

    /// The [Hash](tree::Hash) referencing the root directory / [Tree] of this [Commit]
    pub tree:           tree::Hash,

    /// The [Hash](commit::Hash)es of the 0 or more parent [Commit]s of this [Commit]
    /// 
    /// Initial [Commit]s have 0 parents.
    /// Merge [Commit]s have multiple parents.
    /// Vanilla boring [Commit]s have 1 parent, the previous commit.
    pub parents:        Vec<commit::Hash>,

    //authors:        Vec<String>,
    //committers:     Vec<String>,
    //description:    String,
    _nonexhaustive:     (),
}

impl Commit {
    /// [Read] a local [Commit] from a given [Repository]
    ///
    /// [Read]:         std::io::Read
    pub fn read(repository: &Repository, hash: &commit::Hash) -> io::Result<Self> {
        let mut tree : Option<tree::Hash> = None;
        let mut parents : Vec<commit::Hash> = Vec::new();

        for line in BufReader::new(repository.cat_file_commit(&hash)?).lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() { break } // commit message follows, but we're not parsing that at the moment

            if line.starts_with("tree ") {
                if tree.is_some() { return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Multiple tree s specified by commit {}", hash))); }
                tree = Some(tree::Hash::from_str(&line[5..])?);
            } else if line.starts_with("parent ") {
                parents.push(commit::Hash::from_str(&line[7..])?);
            } else {
                // author, committer, ...
            }
        }

        let tree = tree.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, format!("treeless commit {}", hash)))?;

        Ok(Self {
            hash: hash.clone(),
            tree,
            parents,

            _nonexhaustive: ()
        })
    }
}
