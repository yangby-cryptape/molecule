use std::{default::Default, fmt};

use crate::{bytes::Bytes, error::VerificationResult, verification_error};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Byte([u8; 1]);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteReader<'r>(&'r [u8]);

impl fmt::Debug for Byte {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(0x{:02x})", Self::NAME, self.0[0])
    }
}

impl<'r> fmt::Debug for ByteReader<'r> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(0x{:02x})", Self::NAME, self.0[0])
    }
}

impl fmt::Display for Byte {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(0x{:02x})", Self::NAME, self.0[0])
    }
}

impl<'r> fmt::Display for ByteReader<'r> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(0x{:02x})", Self::NAME, self.0[0])
    }
}

impl Default for Byte {
    #[inline]
    fn default() -> Self {
        Self::new(0)
    }
}

// As Entity
impl Byte {
    #[inline]
    pub fn new(v: u8) -> Self {
        Byte([v; 1])
    }

    #[inline]
    pub fn new_unchecked(data: Bytes) -> Self {
        Byte::new(data[0])
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    #[inline]
    pub fn as_bytes(self) -> Bytes {
        self.as_slice().into()
    }

    #[inline]
    pub fn from_slice(slice: &[u8]) -> VerificationResult<Self> {
        ByteReader::verify(slice, false).map(|_| Self::new(slice[0]))
    }

    #[inline]
    pub fn from_compatible_slice(slice: &[u8]) -> VerificationResult<Self> {
        ByteReader::verify(slice, true).map(|_| Self::new(slice[0]))
    }
}

// As Reader
impl<'r> ByteReader<'r> {
    #[inline]
    pub fn to_entity(self) -> Byte {
        Byte::new(self.0[0])
    }

    #[inline]
    pub fn new_unchecked(slice: &'r [u8]) -> Self {
        ByteReader(slice)
    }

    #[inline]
    pub fn as_slice(&self) -> &'r [u8] {
        self.0
    }

    #[inline]
    pub fn verify(slice: &[u8], _compatible: bool) -> VerificationResult<()> {
        let slice_len = slice.len();
        if slice_len != 1 {
            verification_error!(Self, TotalSizeNotMatch, 1, slice_len)?;
        }
        Ok(())
    }

    #[inline]
    pub fn from_slice(slice: &'r [u8]) -> VerificationResult<Self> {
        Self::verify(slice, false).map(|_| Self::new_unchecked(slice))
    }

    #[inline]
    pub fn from_compatible_slice(slice: &'r [u8]) -> VerificationResult<Self> {
        Self::verify(slice, true).map(|_| Self::new_unchecked(slice))
    }
}

impl Byte {
    pub const NAME: &'static str = "Byte";

    #[inline]
    pub fn new_unchecked_with_padding(data: Bytes) -> Self {
        Self::new(data[0])
    }

    #[inline]
    pub fn as_reader(&self) -> ByteReader<'_> {
        ByteReader::new_unchecked(self.as_slice())
    }
}

impl<'r> ByteReader<'r> {
    pub const NAME: &'r str = "ByteReader";

    pub const fn peek_length_unchecked(_slice: &'r [u8]) -> usize {
        1
    }

    pub const fn peek_length(slice: &'r [u8]) -> VerificationResult<usize> {
        Ok(Self::peek_length_unchecked(slice))
    }

    #[inline]
    pub fn new_unchecked_with_padding(slice: &'r [u8]) -> Self {
        let len = Self::peek_length_unchecked(slice);
        Self::new_unchecked(&slice[..len])
    }
}

impl From<u8> for Byte {
    #[inline]
    fn from(v: u8) -> Self {
        Byte::new(v)
    }
}

impl From<Byte> for u8 {
    #[inline]
    fn from(v: Byte) -> Self {
        v.0[0]
    }
}

impl From<&Byte> for u8 {
    #[inline]
    fn from(v: &Byte) -> Self {
        v.0[0]
    }
}

impl From<ByteReader<'_>> for u8 {
    #[inline]
    fn from(v: ByteReader<'_>) -> Self {
        v.0[0]
    }
}

impl From<&ByteReader<'_>> for u8 {
    #[inline]
    fn from(v: &ByteReader<'_>) -> Self {
        v.0[0]
    }
}
