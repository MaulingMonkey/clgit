use crate::{Branch, CatFileReader, FileType, Hash, HashTempStr, gather_branches};

use std::fmt::{self, Debug, Formatter};
use std::ffi::OsStr;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc};



/// A git repository (a reference to a local path containing a .git directory, or a bare some_repository.git directory)
#[derive(Clone)]
pub struct Repository {
    dot_git:    Arc<PathBuf>,
}

impl Repository {
    /// # Examples
    ///
    /// ```rust
    /// let repository = clgit::Repository::from_bare_repository(".git").unwrap();
    /// ```
    pub fn from_bare_repository(dir: impl Into<PathBuf>) -> io::Result<Self> {
        // TODO: canonicalize?
        let dir = dir.into();
        if dir.join(".git").exists() { return Err(io::Error::new(io::ErrorKind::InvalidData, "not a bare repository")); }
        Ok(Self { dot_git: Arc::new(dir) })
    }

    /// # Examples
    ///
    /// ```rust
    /// let repository = clgit::Repository::from_regular_repository(".").unwrap();
    /// ```
    pub fn from_regular_repository(dir: impl AsRef<Path>) -> io::Result<Self> {
        // TODO: canonicalize?
        let dir = dir.as_ref();
        let dot_git = dir.join(".git");
        if !dot_git.exists() { return Err(io::Error::new(io::ErrorKind::InvalidData, "not a regular repository")); }
        Ok(Self { dot_git: Arc::new(dot_git) })
    }


    /// # Examples
    ///
    /// ```rust
    /// let repository = clgit::Repository::from_path(".").unwrap();
    /// let repository = clgit::Repository::from_path(".git").unwrap();
    /// ```
    pub fn from_path(dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref();
        let dot_git = dir.join(".git");
        if dot_git.exists() {
            Ok(Self { dot_git: Arc::new(dot_git) })
        } else {
            Self::from_bare_repository(dir)
        }
    }

    /// # Examples
    ///
    /// ```rust
    /// # fn examples() -> std::io::Result<()> {
    /// # let repository = clgit::Repository::from_regular_repository(".")?;
    /// for branch in repository.local_branches()? {
    ///     let branch : clgit::Branch = branch?;
    ///     println!("{}", branch.name().to_string_lossy());
    ///     let _ = branch.commit();
    /// }
    /// # Ok(())
    /// # }
    /// # examples().unwrap()
    /// ```
    pub fn local_branches(&self) -> io::Result<impl Iterator<Item = io::Result<Branch>>> {
        let mut branches = Default::default();
        gather_branches(OsStr::new(""), &self.dot_git.join("refs/heads"), &mut branches)?;
        Ok(branches.into_iter().map(|(name, commit)| Ok(Branch { name, commit })))
    }

    /// # Examples
    ///
    /// ```rust
    /// # fn examples() -> std::io::Result<()> {
    /// # let repository = clgit::Repository::from_regular_repository(".")?;
    /// for branch in repository.remote_branches()? {
    ///     let branch : clgit::Branch = branch?;
    ///     println!("{}", branch.name().to_string_lossy());
    ///     let _ = branch.commit();
    /// }
    /// # Ok(())
    /// # }
    /// # examples().unwrap()
    /// ```
    pub fn remote_branches(&self) -> io::Result<impl Iterator<Item = io::Result<Branch>>> {
        let mut branches = Default::default();
        gather_branches(OsStr::new(""), &self.dot_git.join("refs/remotes"), &mut branches)?;
        Ok(branches.into_iter().map(|(name, commit)| Ok(Branch { name, commit })))
    }

    /// Run/parse `git cat-file -s [hash]`
    pub fn cat_file_size(&self, hash: &Hash) -> io::Result<u64> {
        let hash = HashTempStr::new(hash);
        let git = self.git().args(&["cat-file", "-s", hash.as_str()]).output()?;
        match git.status.code() {
            Some(0) => {},
            Some(_) => return Err(io::Error::new(io::ErrorKind::Other, "git cat-file -s ... exited non-zero")),
            None    => return Err(io::Error::new(io::ErrorKind::Other, "git cat-file -s ... died by signal")),
        }
        Ok(String::from_utf8(git.stdout)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "git cat-file -s ... returned non-utf8 size"))?
            .trim()
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "git cat-file -s ... returned non-u64 file size"))?
        )
    }

    /// Run/parse `git cat-file -t [hash]`
    pub fn cat_file_type(&self, hash: &Hash) -> io::Result<FileType> {
        let hash = HashTempStr::new(hash);
        let git = self.git().args(&["cat-file", "-t", hash.as_str()]).output()?;
        match git.status.code() {
            Some(0) => {},
            Some(_) => return Err(io::Error::new(io::ErrorKind::Other, "git cat-file -t ... exited non-zero")),
            None    => return Err(io::Error::new(io::ErrorKind::Other, "git cat-file -t ... died by signal")),
        }
        Ok(String::from_utf8(git.stdout).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "git cat-file -t ... returned non-utf8 type"))?.trim().into())
    }

    /// Run/parse `git cat-file commit [hash]`
    pub fn cat_file_commit  (&self, hash: &Hash) -> io::Result<impl Read> { self.cat_file("commit", hash) }

    /// Run/parse `git cat-file tree [hash]`
    pub fn cat_file_tree    (&self, hash: &Hash) -> io::Result<impl Read> { self.cat_file("tree",   hash) }

    /// Run/parse `git cat-file blob [hash]`
    pub fn cat_file_blob    (&self, hash: &Hash) -> io::Result<impl Read> { self.cat_file("blob",   hash) }

    fn git(&self) -> Command {
        let mut c = Command::new("git");
        c.current_dir(&*self.dot_git);
        c
    }

    fn cat_file(&self, ty: &str, hash: &Hash) -> io::Result<impl Read> {
        let hash = HashTempStr::new(hash);
        let mut git = self.git()
            .args(&["cat-file", ty, hash.as_str()])
            .stdin (Stdio::null())
            .stderr(Stdio::null())
            .stdout(Stdio::piped())
            .spawn()?;
        Ok(CatFileReader { stdout: git.stdout.take().unwrap(), child: git })
    }
}

impl Debug for Repository {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("Repository")
            .field("dot_git", &self.dot_git)
            .field("local_branches",    &self.local_branches().map(|b| b.collect::<Vec<_>>()))
            .field("remote_branches",   &self.remote_branches().map(|r| r.collect::<Vec<_>>()))
            .finish()
    }
}
