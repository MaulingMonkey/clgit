use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileType {
    Blob,
    Commit,
    Tree,

    #[doc(hidden)] _Unknown(String),
}

impl FileType {
    pub fn as_str(&self) -> &str {
        match self {
            FileType::Blob          => "blob",
            FileType::Commit        => "commit",
            FileType::Tree          => "tree",
            FileType::_Unknown(s)   => s.as_str(),
        }
    }
}

impl From<&str> for FileType {
    fn from(t: &str) -> Self {
        match t {
            "blob"      => FileType::Blob,
            "commit"    => FileType::Commit,
            "tree"      => FileType::Tree,
            _other      => FileType::_Unknown(t.to_owned()),
        }
    }
}

impl From<String>   for FileType { fn from(t: String) -> Self { t.as_str().into() } }
impl Display        for FileType { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{}", self.as_str()) } }
impl Debug          for FileType { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "FileType({:?})", self.as_str()) } }
