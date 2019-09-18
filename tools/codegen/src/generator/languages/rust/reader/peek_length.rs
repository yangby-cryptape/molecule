use proc_macro2 as m4;
use quote::quote;

use super::super::utilities::reader_name;
use crate::ast::verified::{self as ast, HasName};

pub(in super::super) trait DefPeekLength {
    fn def_peek_length(&self) -> m4::TokenStream;
}

impl DefPeekLength for ast::Option_ {
    fn def_peek_length(&self) -> m4::TokenStream {
        let inner = reader_name(self.typ.name());
        quote!(
            pub fn peek_length_unchecked(slice: &'r [u8]) -> usize {
                if slice.is_empty() {
                    0
                } else {
                    #inner::peek_length_unchecked(slice)
                }
            }
            pub fn peek_length(slice: &'r [u8]) -> molecule::error::VerificationResult<usize> {
                if slice.is_empty() {
                    Ok(0)
                } else {
                    #inner::peek_length(slice)
                }
            }
        )
    }
}

impl DefPeekLength for ast::Union {
    fn def_peek_length(&self) -> m4::TokenStream {
        quote!(
            pub fn peek_length_unchecked(slice: &'r [u8]) -> usize {
                let reader = Self::new_unchecked(slice);
                reader.total_size()
            }
            pub fn peek_length(slice: &'r [u8]) -> molecule::error::VerificationResult<usize> {
                use molecule::verification_error as ve;
                let slice_len = slice.len();
                if slice_len < Self::HEADER_FULL_SIZE {
                    ve!(Self, HeaderIsBroken, Self::HEADER_FULL_SIZE, slice_len)?;
                }
                Ok(Self::peek_length_unchecked(slice))
            }
        )
    }
}

impl DefPeekLength for ast::Array {
    fn def_peek_length(&self) -> m4::TokenStream {
        quote!(
            pub fn peek_length_unchecked(_slice: &'r [u8]) -> usize {
                Self::TOTAL_SIZE
            }
            pub fn peek_length(_slice: &'r [u8]) -> molecule::error::VerificationResult<usize> {
                Ok(Self::peek_length_unchecked(_slice))
            }
        )
    }
}

impl DefPeekLength for ast::Struct {
    fn def_peek_length(&self) -> m4::TokenStream {
        quote!(
            pub fn peek_length_unchecked(_slice: &'r [u8]) -> usize {
                Self::TOTAL_SIZE
            }
            pub fn peek_length(_slice: &'r [u8]) -> molecule::error::VerificationResult<usize> {
                Ok(Self::peek_length_unchecked(_slice))
            }
        )
    }
}

impl DefPeekLength for ast::FixVec {
    fn def_peek_length(&self) -> m4::TokenStream {
        quote!(
            pub fn peek_length_unchecked(slice: &'r [u8]) -> usize {
                Self::new_unchecked(slice).total_size()
            }
            pub fn peek_length(slice: &'r [u8]) -> molecule::error::VerificationResult<usize> {
                use molecule::verification_error as ve;
                let slice_len = slice.len();
                if slice_len < molecule::NUMBER_SIZE {
                    ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len)?;
                }
                Ok(Self::peek_length_unchecked(slice))
            }
        )
    }
}

impl DefPeekLength for ast::DynVec {
    fn def_peek_length(&self) -> m4::TokenStream {
        quote!(
            pub fn peek_length_unchecked(slice: &'r [u8]) -> usize {
                Self::new_unchecked(slice).total_size()
            }
            pub fn peek_length(slice: &'r [u8]) -> molecule::error::VerificationResult<usize> {
                use molecule::verification_error as ve;
                let slice_len = slice.len();
                if slice_len < molecule::NUMBER_SIZE {
                    ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len)?;
                }
                Ok(Self::peek_length_unchecked(slice))
            }
        )
    }
}

impl DefPeekLength for ast::Table {
    fn def_peek_length(&self) -> m4::TokenStream {
        quote!(
            pub fn peek_length_unchecked(slice: &'r [u8]) -> usize {
                Self::new_unchecked(slice).total_size()
            }
            pub fn peek_length(slice: &'r [u8]) -> molecule::error::VerificationResult<usize> {
                use molecule::verification_error as ve;
                let slice_len = slice.len();
                if slice_len < molecule::NUMBER_SIZE {
                    ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len)?;
                }
                Ok(Self::peek_length_unchecked(slice))
            }
        )
    }
}
