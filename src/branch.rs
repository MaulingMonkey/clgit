use crate::Hash;

use std::collections::*;
use std::fmt::{self, Debug, Formatter};
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::Path;



/// A named reference to a commit (e.g. "master" => "074d881e29cc3bff82da905adcde2aea7cb5b165")
pub struct Branch {
    pub(crate) name:    OsString,
    pub(crate) commit:  Hash
}

impl Branch {
    /// Name of the branch (e.g. "master")
    pub fn name(&self) -> &OsStr { &self.name }

    /// [Hash] of the [Commit] this branch points to (e.g. "074d881e29cc3bff82da905adcde2aea7cb5b165")
    ///
    /// [Commit]:   crate::Commit
    /// [Hash]:     crate::Hash
    pub fn commit(&self) -> &Hash { &self.commit }
}

impl Debug for Branch {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Branch({:?} => \"{}\")", self.name, self.commit)
    }
}



/// A named reference to a commit (e.g. "master" => "074d881e29cc3bff82da905adcde2aea7cb5b165") w/o deep copies
pub struct BranchRef<'r> {
    pub(crate) name:    &'r OsString,
    pub(crate) commit:  &'r Hash
}

impl BranchRef<'_> {
    /// Name of the branch (e.g. "master")
    pub fn name(&self) -> &OsStr { &self.name }

    /// Hash of the commit this branch points to (e.g. "074d881e29cc3bff82da905adcde2aea7cb5b165")
    pub fn commit(&self) -> &Hash { &self.commit }
}

impl Debug for BranchRef<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "BranchRef({:?} => \"{}\")", self.name, self.commit)
    }
}



pub(crate) fn gather_branches(parent_name: &OsStr, parent_path: &Path, branches: &mut BTreeMap<OsString, Hash>) -> io::Result<()> {
    let dir = match parent_path.read_dir() {
        Ok(dir)     => dir,
        Err(ref e)  if e.kind() == io::ErrorKind::NotFound && parent_name.is_empty() => return Ok(()),
        Err(e)      => return Err(e),
    };

    for e in dir {
        let e = e?;
        let meta = e.metadata()?;

        let full_path = e.path();

        let mut name = parent_name.to_os_string();
        if !name.is_empty() { name.push("/"); }
        name.push(e.file_name());

        if meta.is_dir() {
            gather_branches(&name, &full_path, branches)?;
        } else if meta.is_file() {
            branches.insert(name, Hash::from_str(std::fs::read_to_string(&full_path)?.trim())?);
        }
    }
    Ok(())
}
