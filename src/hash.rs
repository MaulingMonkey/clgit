use std::convert::*;
use std::fmt::{self, Debug, Display, Formatter};
use std::io::{self, Read};



/// A [SHA-1] or [SHA-256] reference to a git [Commit], [Tree], or Blob
///
/// [Commit]:   crate::Commit
/// [Tree]:     crate::Tree
/// [SHA-1]:    https://en.wikipedia.org/wiki/SHA-1
/// [SHA-256]:  https://en.wikipedia.org/wiki/SHA-2
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash {
    bytes:  [u8; 32],
    len:    u8,
}

impl Hash {
    /// Construct a [Hash] from a hexidecimal string.  The entire hash must be specified: 40 characters ([SHA-1]) or 64 ([SHA-256])
    ///
    /// # Examples
    /// ```rust
    /// # use clgit::Hash;
    /// for good in [
    ///     // Legal SHA-1 hashes (20 bytes / 40 characters)
    ///     "74da26a93c3eac22884a62bd8d70aab3434c9174",
    ///     "89dd60cc88e4f89e0af91e2739c42a31c3a106bb",
    ///     "eb6c43cb699caa2ccbc4e28f9ab75a2a17e4ee7c",
    ///
    ///     // Uppercase is legal too
    ///     "74DA26A93C3EAC22884A62BD8D70AAB3434C9174",
    ///     "89DD60CC88E4F89E0AF91E2739C42A31C3A106BB",
    ///     "EB6C43CB699CAA2CCBC4E28F9AB75A2A17E4EE7C",
    /// 
    ///     // SHA-256 hashes (40 bytes / 64 characters)
    ///     "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    /// ].iter().cloned() {
    ///     Hash::from_str(good).unwrap_or_else(|e| panic!("Failed to parse {}: {}", good, e));
    /// }
    ///
    /// for bad in [
    ///     "eb6c43cb699caa2ccbc4e28f9ab75a2a17e4ee7c0", // too long
    ///     "eb6c43cb699caa2ccbc4e28f9ab75a2a17e4ee7",   // too short
    ///     "eb6c43cb699caa2ccbc4e28f9ab75a2a17e4ee7!",  // invalid character
    ///     "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcde",   // too short
    ///     "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0", // too long
    /// ].iter().cloned() {
    ///     assert!(Hash::from_str(bad).is_err(), "Didn't expect to parse {}", bad);
    /// }
    /// ```
    ///
    /// [Hash]:     crate::Hash
    ///
    /// [SHA-1]:    https://en.wikipedia.org/wiki/SHA-1
    /// [SHA-256]:  https://en.wikipedia.org/wiki/SHA-2
    pub fn from_str(s: &str) -> Result<Self, HashParseError> {
        let mut bytes = [0u8; 32];
        let mut ascii = s.as_bytes();
        match ascii.len() {
            40 | 64 => {},
            _       => return Err(HashParseError::LengthMismatch),
        }

        let len = ascii.len() / 2;
        let mut dst = &mut bytes[..len];
        while !dst.is_empty() {
            let h = ascii_byte_to_hex(ascii[0])?;
            let l = ascii_byte_to_hex(ascii[1])?;
            dst[0] = (h << 4) | l;
            ascii = &ascii[2..];
            dst   = &mut dst[1..];
        }

        Ok(Self { bytes, len: len as u8 })
    }

    /// Construct a [Hash] from a slice of bytes.  The entire hash must be specified: 20 bytes ([SHA-1]) or 32 ([SHA-256])
    ///
    /// # Examples
    /// ```rust
    /// # use clgit::Hash;
    /// Hash::from_bytes(&[0u8; 20][..]).expect("20 bytes OK");
    /// Hash::from_bytes(&[0u8; 32][..]).expect("32 bytes OK");
    /// 
    /// Hash::from_bytes(&[0u8; 19][..]).expect_err("19 bytes invalid");
    /// Hash::from_bytes(&[0u8; 21][..]).expect_err("21 bytes invalid");
    /// Hash::from_bytes(&[0u8; 31][..]).expect_err("31 bytes invalid");
    /// Hash::from_bytes(&[0u8; 33][..]).expect_err("33 bytes invalid");
    /// ```
    ///
    /// [Hash]:     crate::Hash
    ///
    /// [SHA-1]:    https://en.wikipedia.org/wiki/SHA-1
    /// [SHA-256]:  https://en.wikipedia.org/wiki/SHA-2
    pub fn from_bytes(src: &[u8]) -> Result<Self, HashParseError> {
        let mut bytes = [0u8; 32];
        let len = src.len();
        match len {
            20 | 32 => bytes[..len].copy_from_slice(src),
            _       => return Err(HashParseError::LengthMismatch),
        }
        Ok(Self { bytes, len: len as u8 })
    }

    /// [Read] 20 bytes from `r` and treat it as a [SHA-1] hash
    /// 
    /// # Example
    /// ```rust
    /// # use clgit::Hash;
    /// let mut io = std::io::Cursor::new(vec![0; 128]);
    /// Hash::read_sha1(&mut io).unwrap();
    /// ```
    ///
    /// [SHA-1]:    https://en.wikipedia.org/wiki/SHA-1
    pub fn read_sha1(r: &mut impl Read) -> io::Result<Self> {
        let mut bytes = [0u8; 32];
        r.read_exact(&mut bytes[..20])?;
        Ok(Self { bytes, len: 20 })
    }

    /// [Read] 32 bytes from `r` and treat it as a [SHA-256] hash
    /// 
    /// # Example
    /// ```rust
    /// # use clgit::Hash;
    /// let mut io = std::io::Cursor::new(vec![0; 128]);
    /// Hash::read_sha256(&mut io).unwrap();
    /// ```
    ///
    /// [SHA-256]:  https://en.wikipedia.org/wiki/SHA-2
    pub fn read_sha256(r: &mut impl Read) -> io::Result<Self> {
        let mut bytes = [0u8; 32];
        r.read_exact(&mut bytes[..])?;
        Ok(Self { bytes, len: 32 })
    }

    /// Get the number of bytes in this hash (20 or 32)
    /// 
    /// # Example
    /// ```rust
    /// # use clgit::Hash;
    /// # let hash = Hash::default();
    /// assert!(hash.len() == 20 || hash.len() == 32);
    /// ```
    pub fn len(&self) -> usize { usize::from(self.len) }

    /// Get the bytes in this hash (length of 20 or 32)
    /// 
    /// # Example
    /// ```rust
    /// # use clgit::Hash;
    /// # let hash = Hash::default();
    /// let bytes : &[u8] = hash.bytes();
    /// assert!(bytes.len() == 20 || bytes.len() == 32);
    /// ```
    pub fn bytes(&self) -> &[u8] { &self.bytes[..self.len()] }

    /// Get the first byte of this hash
    /// 
    /// # Example
    /// ```rust
    /// # use clgit::Hash;
    /// # let hash = Hash::default();
    /// println!("byte: {:02x}", hash.first_byte());
    /// ```
    pub fn first_byte(&self) -> u8 { self.bytes[0] }
}

impl Display for Hash {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        for b in &self.bytes[..self.len as usize] {
            write!(fmt, "{:02x}", b)?;
        }
        Ok(())
    }
}

impl Debug for Hash {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Hash(\"{}\")", self)
    }
}

impl Default for Hash {
    fn default() -> Self {
        Self {
            bytes:  [0u8; 32],
            len:    20, // sha1
        }
    }
}



/// Describes how a [Hash] failed to [parse].
/// Convertable to [std::io::Error], [Box]&lt;dyn [std::error::Error]&gt;.
///
/// [Hash]:         crate::Hash
/// [parse]:        str::parse
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HashParseError {
    /// [Hash] wasn't an expected length (20/32 bytes, or 40/64 characters)
    ///
    /// [Hash]:         crate::Hash
    LengthMismatch,

    /// [Hash] contained an invalid character (expected [hexadecimal](https://simple.wikipedia.org/wiki/Hexadecimal) characters only)
    ///
    /// [Hash]:         crate::Hash
    BadCharacter(char),
}

impl std::error::Error for HashParseError {}

impl Display for HashParseError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            HashParseError::LengthMismatch  => write!(fmt, "Hash length mismatch"),
            HashParseError::BadCharacter(c) => write!(fmt, "Invalid character {:?} in hash", c),
        }
    }
}

impl From<HashParseError> for io::Error {
    fn from(hpe: HashParseError) -> Self {
        io::Error::new(io::ErrorKind::InvalidData, hpe)
    }
}



pub(crate) struct HashTempStr {
    ascii:  [u8; 64],
    len:    usize,
}

impl HashTempStr {
    pub fn new(hash: &Hash) -> Self {
        let mut ascii = [0u8; 64];
        let len = usize::from(hash.len) * 2;

        let mut dst = &mut ascii[..];

        let hex = b"0123456789abcdef";
        for b in &hash.bytes[..usize::from(hash.len)] {
            dst[0] = hex[usize::from(b >> 4)];
            dst[1] = hex[usize::from(b & 0xF)];
            dst = &mut dst[2..];
        }

        Self {
            ascii,
            len
        }
    }

    pub fn bytes(&self) -> &[u8] { &self.ascii[..self.len] }
    pub fn as_str(&self) -> &str { std::str::from_utf8(self.bytes()).unwrap() }
}



fn ascii_byte_to_hex(b: u8) -> Result<u8, HashParseError> {
    match b {
        b'0' ..= b'9'   => Ok(b - b'0'),
        b'a' ..= b'f'   => Ok(b - b'a' + 10),
        b'A' ..= b'F'   => Ok(b - b'A' + 10),
        _               => Err(HashParseError::BadCharacter(b as char)),
    }
}
