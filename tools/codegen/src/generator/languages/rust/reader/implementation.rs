use proc_macro2 as m4;
use quote::quote;

use super::super::utilities::{entity_name, reader_name, usize_lit};
use crate::ast::verified::{self as ast, HasName};

pub(in super::super) trait ImplReader: HasName {
    fn impl_reader_internal(&self) -> m4::TokenStream;

    fn impl_reader(&self) -> m4::TokenStream {
        let entity = entity_name(self.name());
        let reader = reader_name(self.name());
        let reader_string = reader.to_string();
        let internal = self.impl_reader_internal();
        quote!(
            impl<'r> molecule::prelude::Reader<'r> for #reader<'r> {
                type Entity = #entity;
                const NAME: &'static str = #reader_string;
                fn to_entity(&self) -> Self::Entity {
                    Self::Entity::new_unchecked(self.as_slice().into())
                }
                fn new_unchecked(slice: &'r [u8]) -> Self {
                    #reader(slice)
                }
                fn as_slice(&self) -> &'r [u8] {
                    self.0
                }
                #internal
            }
        )
    }
}

impl ImplReader for ast::Option_ {
    fn impl_reader_internal(&self) -> m4::TokenStream {
        let inner = reader_name(self.typ.name());
        quote!(
            fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
                if !slice.is_empty() {
                    #inner::verify(&slice[..], compatible)?;
                }
                Ok(())
            }
        )
    }
}

impl ImplReader for ast::Union {
    fn impl_reader_internal(&self) -> m4::TokenStream {
        let verify_inners = self.inner.iter().enumerate().map(|(index, inner)| {
            let item_id = usize_lit(index);
            let inner = reader_name(inner.typ.name());
            quote!(
                #item_id => #inner::verify(inner_slice, compatible),
            )
        });
        quote!(
            fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
                use molecule::verification_error as ve;
                let slice_len = slice.len();
                if slice_len < Self::HEADER_FULL_SIZE {
                    ve!(Self, HeaderIsBroken, Self::HEADER_FULL_SIZE, slice_len)?;
                }
                let total_size = molecule::unpack_number(slice) as usize;
                if slice_len != total_size {
                    ve!(Self, TotalSizeNotMatch, total_size, slice_len)?;
                }
                let item_id = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]);
                let inner_slice = &slice[Self::HEADER_FULL_SIZE..];
                match item_id {
                    #( #verify_inners )*
                    _ => ve!(Self, UnknownItem, Self::ITEM_COUNT, item_id),
                }?;
                Ok(())
            }
        )
    }
}

impl ImplReader for ast::Array {
    fn impl_reader_internal(&self) -> m4::TokenStream {
        let inner = reader_name(self.typ.name());
        let verify_inners = if self.typ.is_atom() {
            Vec::new()
        } else {
            (0..self.item_count)
                .map(|i| {
                    let start = usize_lit((self.item_size + self.item_padding) * i);
                    let end = usize_lit(self.item_size * (i + 1) + self.item_padding * i);
                    if i == 0 || self.item_padding == 0 {
                        quote!(
                            #inner::verify(&slice[#start..#end], compatible)?;
                        )
                    } else {
                        let padding_start =
                            usize_lit(self.item_size * i + self.item_padding * (i - 1));
                        quote!(
                            molecule::check_padding_result!(Self, slice, #padding_start, #start)?;
                            #inner::verify(&slice[#start..#end], compatible)?;
                        )
                    }
                })
                .collect()
        };
        quote!(
            fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
                use molecule::verification_error as ve;
                let slice_len = slice.len();
                if slice_len != Self::TOTAL_SIZE {
                    ve!(Self, TotalSizeNotMatch, Self::TOTAL_SIZE, slice_len)?;
                }
                #( #verify_inners )*
                let _ = compatible;
                Ok(())
            }
        )
    }
}

impl ImplReader for ast::Struct {
    fn impl_reader_internal(&self) -> m4::TokenStream {
        let verify_fields = {
            let mut offset = 0;
            let mut codes = Vec::with_capacity(self.field_size.len());
            for ((f, s), p) in self
                .inner
                .iter()
                .zip(self.field_size.iter())
                .zip(self.field_padding.iter())
            {
                let field = reader_name(f.typ.name());
                let padding_start = usize_lit(offset);
                offset += p;
                let start = usize_lit(offset);
                offset += s;
                let end = usize_lit(offset);
                let code = if *p == 0 {
                    quote!(
                        #field::verify(&slice[#start..#end], compatible)?;
                    )
                } else {
                    quote!(
                        molecule::check_padding_result!(Self, slice, #padding_start, #start)?;
                        #field::verify(&slice[#start..#end], compatible)?;
                    )
                };
                codes.push(code);
            }
            codes
        };
        quote!(
            fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
                use molecule::verification_error as ve;
                let slice_len = slice.len();
                if slice_len != Self::TOTAL_SIZE {
                    ve!(Self, TotalSizeNotMatch, Self::TOTAL_SIZE, slice_len)?;
                }
                #( #verify_fields )*
                let _ = compatible;
                Ok(())
            }
        )
    }
}

impl ImplReader for ast::FixVec {
    fn impl_reader_internal(&self) -> m4::TokenStream {
        let inner = reader_name(self.typ.name());
        let verify_inners = if self.typ.is_atom() {
            quote!(let _ = compatible;)
        } else if self.item_padding == 0 {
            quote!(
                use molecule::check_padding_result;
                let mut start = Self::HEADER_SIZE + Self::HEADER_PADDING;
                check_padding_result!(Self, slice, Self::HEADER_SIZE, start)?;
                for _ in 0..item_count {
                    let end = start + Self::ITEM_SIZE;
                    #inner::verify(&slice[start..end], compatible)?;
                    start = end;
                }
            )
        } else {
            quote!(
                use molecule::check_padding_result;
                let mut padding_start = Self::HEADER_SIZE;
                let mut start = padding_start + Self::HEADER_PADDING;
                let mut end = start + Self::ITEM_SIZE;
                check_padding_result!(Self, slice, padding_start, start)?;
                #inner::verify(&slice[start..end], compatible)?;
                for _ in 1..item_count {
                    padding_start = end;
                    start = padding_start + Self::ITEM_PADDING;
                    end = start + Self::ITEM_SIZE;
                    check_padding_result!(Self, slice, padding_start, start)?;
                    #inner::verify(&slice[start..end], compatible)?;
                }
            )
        };
        quote!(
            fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
                use molecule::verification_error as ve;
                let slice_len = slice.len();
                if slice_len < Self::HEADER_SIZE {
                    ve!(Self, HeaderIsBroken, Self::HEADER_SIZE, slice_len)?;
                }
                let item_count = molecule::unpack_number(slice) as usize;
                if item_count == 0 {
                    if slice_len != Self::HEADER_SIZE {
                        ve!(Self, TotalSizeNotMatch, Self::HEADER_SIZE, slice_len)?;
                    }
                    return Ok(());
                }
                let total_size = Self::HEADER_SIZE + Self::HEADER_PADDING
                    + (Self::ITEM_SIZE + Self::ITEM_PADDING) * item_count - Self::ITEM_PADDING;
                if slice_len != total_size {
                    ve!(Self, TotalSizeNotMatch, total_size, slice_len)?;
                }
                #verify_inners
                Ok(())
            }
        )
    }
}

impl ImplReader for ast::DynVec {
    fn impl_reader_internal(&self) -> m4::TokenStream {
        let inner = reader_name(self.typ.name());
        quote!(
            fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
                use molecule::{check_padding_result, verification_error as ve};
                let slice_len = slice.len();
                if slice_len < Self::HEADER_BASE_SIZE {
                    ve!(Self, HeaderIsBroken, Self::HEADER_BASE_SIZE, slice_len)?;
                }
                let total_size = molecule::unpack_number(slice) as usize;
                if slice_len != total_size {
                    ve!(Self, TotalSizeNotMatch, total_size, slice_len)?;
                }
                let item_count = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
                if item_count == 0 {
                    if slice_len != Self::HEADER_BASE_SIZE {
                        ve!(Self, TotalSizeNotMatch, Self::HEADER_BASE_SIZE, slice_len)?;
                    }
                    return Ok(());
                }
                let header_size = Self::HEADER_BASE_SIZE + molecule::NUMBER_SIZE * item_count;
                if slice_len < header_size {
                    ve!(Self, HeaderIsBroken, header_size, slice_len)?;
                }
                let ptr = molecule::unpack_number_vec(&slice[(molecule::NUMBER_SIZE*2)..]);
                let mut offsets: Vec<usize> = ptr[..item_count]
                    .iter()
                    .map(|x| molecule::unpack_number(&x[..]) as usize)
                    .collect();
                offsets.push(total_size);
                if offsets.windows(2).any(|i| i[0] > i[1]) {
                    ve!(Self, OffsetsNotMatch)?;
                }
                let mut end = header_size;
                for pair in offsets.windows(2) {
                    let start = pair[0];
                    let next =  pair[1];
                    check_padding_result!(Self, slice, end, start)?;
                    end = start + #inner::peek_length(&slice[start..next])?;
                    #inner::verify(&slice[start..end], compatible)?;
                }
                Ok(())
            }
        )
    }
}

impl ImplReader for ast::Table {
    fn impl_reader_internal(&self) -> m4::TokenStream {
        if self.inner.is_empty() {
            quote!(
                fn verify(
                    slice: &[u8],
                    compatible: bool,
                ) -> molecule::error::VerificationResult<()> {
                    use molecule::verification_error as ve;
                    let slice_len = slice.len();
                    if slice_len < Self::HEADER_SIZE {
                        ve!(Self, HeaderIsBroken, Self::HEADER_SIZE, slice_len)?;
                    }
                    let total_size = molecule::unpack_number(slice) as usize;
                    if slice_len != total_size {
                        ve!(Self, TotalSizeNotMatch, total_size, slice_len)?;
                    }
                    let field_count =
                        molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
                    if field_count != Self::FIELD_COUNT && !compatible {
                        ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count)?;
                    };
                    Ok(())
                }
            )
        } else {
            let verify_fields = self.inner.iter().enumerate().map(|(i, f)| {
                let field = reader_name(f.typ.name());
                let curr = usize_lit(i);
                let next = usize_lit(i + 1);
                quote!(
                    let start = offsets[#curr];
                    check_padding_result!(Self, slice, end, start)?;
                    end = start + #field::peek_length(&slice[start..offsets[#next]])?;
                    #field::verify(&slice[start..end], compatible)?;
                )
            });
            quote!(
                fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
                    use molecule::{check_padding_result, verification_error as ve};
                    let slice_len = slice.len();
                    if slice_len < Self::HEADER_SIZE {
                        ve!(Self, HeaderIsBroken, Self::HEADER_SIZE, slice_len)?;
                    }
                    let total_size = molecule::unpack_number(slice) as usize;
                    if slice_len != total_size {
                        ve!(Self, TotalSizeNotMatch, total_size, slice_len)?;
                    }
                    let field_count = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
                    if field_count < Self::FIELD_COUNT {
                        ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count)?;
                    } else if field_count != Self::FIELD_COUNT && !compatible {
                        ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count)?;
                    };
                    let mut end = if field_count == Self::FIELD_COUNT {
                        Self::HEADER_SIZE
                    } else {
                        let header_size = molecule::NUMBER_SIZE * (2 + field_count);
                        if slice_len < header_size {
                            ve!(Self, HeaderIsBroken, header_size, slice_len)?;
                        }
                        header_size
                    };
                    let ptr = molecule::unpack_number_vec(&slice[(molecule::NUMBER_SIZE*2)..]);
                    let mut offsets: Vec<usize> = ptr[..field_count]
                        .iter()
                        .map(|x| molecule::unpack_number(&x[..]) as usize)
                        .collect();
                    offsets.push(total_size);
                    if offsets.windows(2).any(|i| i[0] > i[1]) {
                        ve!(Self, OffsetsNotMatch)?;
                    }
                    #( #verify_fields )*
                    Ok(())
                }
            )
        }
    }
}
