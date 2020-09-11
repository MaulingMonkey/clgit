use crate::*;

use std::convert::*;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;



/// A git [Repository] + in-memory caches for [Commit]s, [Tree]s, and possibly Blobs
pub struct RepositoryCache {
    pub(crate) repository: Repository,
    commits:    SharedHashMap<Commit, Arc<Commit>>,
    trees:      SharedHashMap<Tree,   Arc<Tree>  >,
}

impl RepositoryCache {
    /// Create a cache for a given repository
    pub fn new(repository: Repository) -> Self {
        Self {
            repository,
            commits:    Default::default(),
            trees:      Default::default(),
        }
    }

    /// Attempt to read a [Commit] by it's given [Hash](commit::Hash)
    pub fn commit(&self, hash: &commit::Hash) -> io::Result<Arc<Commit>> {
        if let Some(commit) = self.commits.get_clone(hash) {
            return Ok(commit);
        }

        let commit = Arc::new(Commit::read(&self.repository, hash)?);
        let mut bucket = self.commits.bucket_for(hash).lock().unwrap();
        if let Some(commit) = bucket.get(hash) {
            Ok(commit.clone())
        } else {
            bucket.insert(hash.clone(), commit.clone());
            Ok(commit)
        }
    }

    /// Attempt to read a [Tree] by it's given [Hash](tree::Hash)
    pub fn tree(&self, hash: &tree::Hash) -> io::Result<Arc<Tree>> {
        if let Some(tree) = self.trees.get_clone(hash) {
            return Ok(tree);
        }

        let tree = Arc::new(Tree::read(&self.repository, hash)?);
        let mut bucket = self.trees.bucket_for(hash).lock().unwrap();
        if let Some(tree) = bucket.get(hash) {
            Ok(tree.clone())
        } else {
            bucket.insert(hash.clone(), tree.clone());
            Ok(tree)
        }
    }
}

impl From<Repository> for RepositoryCache {
    fn from(repository: Repository) -> Self {
        Self::new(repository)
    }
}



/// Try to convert into an [Arc]\<[RepositoryCache]\> on pain of [std::io::Error].  Implemented for:<br>
/// &[Path], \(&\)[PathBuf], [Repository], [RepositoryCache], \(&\)[Arc]\<[RepositoryCache]\>, ...
pub trait TryIntoSharedRepositoryCache {
    /// Try to convert into an [Arc]\<[RepositoryCache]\> on pain of [std::io::Error].
    ///
    /// May return an [std::io::Error] on an invalid path, invalid git repository, or any number of other error cases.
    fn try_into_src(self) -> io::Result<Arc<RepositoryCache>>;
}

impl TryIntoSharedRepositoryCache for PathBuf               { fn try_into_src(self) -> io::Result<Arc<RepositoryCache>> { Ok(Arc::new(RepositoryCache::new(Repository::from_path(&self)?))) } }
impl TryIntoSharedRepositoryCache for &PathBuf              { fn try_into_src(self) -> io::Result<Arc<RepositoryCache>> { Ok(Arc::new(RepositoryCache::new(Repository::from_path(self)?))) } }
impl TryIntoSharedRepositoryCache for &Path                 { fn try_into_src(self) -> io::Result<Arc<RepositoryCache>> { Ok(Arc::new(RepositoryCache::new(Repository::from_path(self)?))) } }

impl TryIntoSharedRepositoryCache for String                { fn try_into_src(self) -> io::Result<Arc<RepositoryCache>> { Ok(Arc::new(RepositoryCache::new(Repository::from_path(&self)?))) } }
impl TryIntoSharedRepositoryCache for &String               { fn try_into_src(self) -> io::Result<Arc<RepositoryCache>> { Ok(Arc::new(RepositoryCache::new(Repository::from_path(self)?))) } }
impl TryIntoSharedRepositoryCache for &str                  { fn try_into_src(self) -> io::Result<Arc<RepositoryCache>> { Ok(Arc::new(RepositoryCache::new(Repository::from_path(self)?))) } }

impl TryIntoSharedRepositoryCache for Repository            { fn try_into_src(self) -> io::Result<Arc<RepositoryCache>> { Ok(Arc::new(RepositoryCache::new(self))) } }
impl TryIntoSharedRepositoryCache for RepositoryCache       { fn try_into_src(self) -> io::Result<Arc<RepositoryCache>> { Ok(Arc::new(self)) } }
impl TryIntoSharedRepositoryCache for Arc<RepositoryCache>  { fn try_into_src(self) -> io::Result<Arc<RepositoryCache>> { Ok(self) } }
impl TryIntoSharedRepositoryCache for &Arc<RepositoryCache> { fn try_into_src(self) -> io::Result<Arc<RepositoryCache>> { Ok(self.clone()) } }
