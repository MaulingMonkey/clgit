use std::borrow::Borrow;
use std::fmt::{self, Debug, Formatter};



/// A git file or tree name (e.g. ".gitignore") - typically UTF8, but not guaranteed.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name(Int);

impl Name {
    /// Construct a [Name] from a [String], &[str], [Vec]\<[u8]\>, or &\[[u8]\]
    pub fn from(v: impl Into<Self>) -> Self {
        v.into()
    }

    /// Get this [Name] as a UTF8 string.  Use [as_str_lossy] if you don't care about preserving non-UTF8 byte sequences.
    ///
    /// [as_str_lossy]:     Self::as_str_lossy
    pub fn as_str(&self) -> Option<&str> {
        match &self.0 {
            Int::UTF8(s)        => Some(s.as_str()),
            Int::Bytes { .. }   => None,
        }
    }

    /// Get this [Name] as a UTF8 string.  If the original name wasn't UTF8, this won't be exactly the same as the original.
    pub fn as_str_lossy(&self) -> &str {
        match &self.0 {
            Int::UTF8(s)                => s.as_str(),
            Int::Bytes { lossy, .. }    => lossy.as_str(),
        }
    }

    /// Get this [Name] as a series of bytes.  If the original name was an [OsString] or [PathBuf], this might fail.
    ///
    /// [OsString]: std::ffi::OsString
    /// [PathBuf]:  std::path::PathBuf
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match &self.0 {
            Int::UTF8(s)                => Some(s.as_bytes()),
            Int::Bytes { original, .. } => Some(&original[..]),
        }
    }
}

impl Debug              for Name { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{:?}", self.as_str_lossy()) } }
impl Default            for Name { fn default() -> Self { Self(Int::UTF8(String::new())) }}
impl From<String>       for Name { fn from(v: String) -> Self { Name(Int::UTF8(v)) } }
impl From<&str>         for Name { fn from(v: &str) -> Self { Name(Int::UTF8(v.to_owned())) } }
impl Borrow<str>        for Name { fn borrow(&self) -> &str { self.as_str_lossy() } }

impl PartialEq<str>     for Name { fn eq(&self, other: &str     ) -> bool { self.as_str() == Some(other) } }
impl PartialEq<&str>    for Name { fn eq(&self, other: &&str    ) -> bool { self.as_str() == Some(*other) } }
impl PartialEq<[u8]>    for Name { fn eq(&self, other: &[u8]    ) -> bool { self.as_bytes() == Some(other) } }
impl PartialEq<&[u8]>   for Name { fn eq(&self, other: &&[u8]   ) -> bool { self.as_bytes() == Some(*other) } }
impl PartialEq<Name>    for str  { fn eq(&self, other: &Name    ) -> bool { other == self } }
impl PartialEq<Name>    for [u8] { fn eq(&self, other: &Name    ) -> bool { other == self } }
impl PartialEq<&Name>   for str  { fn eq(&self, other: &&Name   ) -> bool { *other == self } }
impl PartialEq<&Name>   for [u8] { fn eq(&self, other: &&Name   ) -> bool { *other == self } }

impl From<&[u8]> for Name {
    fn from(v: &[u8]) -> Self {
        match std::str::from_utf8(v) {
            Ok(s) => Self(Int::UTF8(s.to_owned())),
            Err(_) => {
                Self(Int::Bytes {
                    lossy:      String::from_utf8_lossy(v).into(),
                    original:   v.to_owned(),
                })
            },
        }
    }
}

impl From<Vec<u8>> for Name {
    fn from(v: Vec<u8>) -> Self {
        match String::from_utf8(v) {
            Ok(s) => Self(Int::UTF8(s)),
            Err(e) => {
                let v = e.into_bytes();
                Self(Int::Bytes {
                    lossy:      String::from_utf8_lossy(&v[..]).into(),
                    original:   v,
                })
            },
        }
    }
}



#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Int {
    UTF8(String),
    Bytes {
        lossy:      String,
        original:   Vec<u8>,
    },
    // TODO: OsString variant for !unix?
}
