pub use alloc::{borrow::ToOwned, vec, vec::Vec};
use core::{clone::Clone, default::Default, fmt};

use crate::{bytes::Bytes, error::VerificationResult, io};

pub use crate::primitive::{Byte, ByteReader};

pub trait Entity: fmt::Debug + Default + Clone {
    type Builder: Builder;
    const NAME: &'static str;
    fn new_unchecked(data: Bytes) -> Self;
    fn as_bytes(&self) -> Bytes;
    fn as_slice(&self) -> &[u8];
    fn from_slice(slice: &[u8]) -> VerificationResult<Self>;
    fn from_compatible_slice(slice: &[u8]) -> VerificationResult<Self>;
    fn new_builder() -> Self::Builder;
    #[allow(clippy::wrong_self_convention)]
    fn as_builder(self) -> Self::Builder;
}

pub trait Reader<'r>: Sized + fmt::Debug + Clone + Copy {
    type Entity: Entity;
    const NAME: &'static str;
    fn verify(slice: &[u8], compatible: bool) -> VerificationResult<()>;
    fn new_unchecked(slice: &'r [u8]) -> Self;
    fn as_slice(&self) -> &'r [u8];
    fn from_slice(slice: &'r [u8]) -> VerificationResult<Self> {
        Self::verify(slice, false).map(|_| Self::new_unchecked(slice))
    }
    fn from_compatible_slice(slice: &'r [u8]) -> VerificationResult<Self> {
        Self::verify(slice, true).map(|_| Self::new_unchecked(slice))
    }
    fn to_entity(&self) -> Self::Entity;
}

pub trait Builder: Default {
    type Entity: Entity;
    const NAME: &'static str;
    fn expected_length(&self) -> usize;
    fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;
    fn build(&self) -> Self::Entity;
}
