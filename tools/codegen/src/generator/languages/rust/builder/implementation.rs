use proc_macro2 as m4;
use quote::quote;

use super::super::utilities::{builder_name, entity_name, field_name, usize_lit};
use crate::ast::verified::{self as ast, HasName};

pub(in super::super) trait ImplBuilder: HasName {
    fn impl_builder_internal(&self) -> m4::TokenStream;

    fn impl_builder(&self) -> m4::TokenStream {
        let builder = builder_name(self.name());
        let builder_string = builder.to_string();
        let entity = entity_name(self.name());
        let internal = self.impl_builder_internal();
        quote!(
            impl molecule::prelude::Builder for #builder {
                type Entity = #entity;
                const NAME: &'static str = #builder_string;
                #internal
                fn build(&self) -> Self::Entity {
                    let mut inner = molecule::create_content(self.expected_length());
                    self.write(&mut inner)
                        .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
                    #entity::new_unchecked(inner.into())
                }
            }
        )
    }
}

impl ImplBuilder for ast::Option_ {
    fn impl_builder_internal(&self) -> m4::TokenStream {
        quote!(
            fn expected_length(&self) -> usize {
                self.0.as_ref().map(|ref inner| inner.as_slice().len()).unwrap_or(0)
            }
            fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
                self.0.as_ref().map(|ref inner| writer.write_all(inner.as_slice())).unwrap_or(Ok(()))
            }
        )
    }
}

impl ImplBuilder for ast::Union {
    fn impl_builder_internal(&self) -> m4::TokenStream {
        quote!(
            fn expected_length(&self) -> usize {
                Self::HEADER_FULL_SIZE + self.0.as_slice().len()
            }
            fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
                let total_size = Self::HEADER_FULL_SIZE + self.0.as_slice().len();
                writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
                writer.write_all(&molecule::pack_number(self.0.item_id()))?;
                writer.write_all(self.0.as_slice())
            }
        )
    }
}

impl ImplBuilder for ast::Array {
    fn impl_builder_internal(&self) -> m4::TokenStream {
        let write_inners = {
            let idx = (1..self.item_count).map(usize_lit).collect::<Vec<_>>();
            if self.item_padding == 0 {
                quote!(
                    writer.write_all(self.0[0].as_slice())?;
                    #(
                        writer.write_all(self.0[#idx].as_slice())?;
                    )*
                )
            } else {
                quote!(
                    writer.write_all(self.0[0].as_slice())?;
                    #(
                        writer.write_all(&[0u8; Self::ITEM_PADDING])?;
                        writer.write_all(self.0[#idx].as_slice())?;
                    )*
                )
            }
        };
        quote!(
            fn expected_length(&self) -> usize {
                Self::TOTAL_SIZE
            }
            fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
                #write_inners
                Ok(())
            }
        )
    }
}

impl ImplBuilder for ast::Struct {
    fn impl_builder_internal(&self) -> m4::TokenStream {
        let fields = self.inner.iter().enumerate().map(|(i, f)| {
            let field_name = field_name(&f.name);
            if self.field_padding[i] == 0 {
                quote!(
                    writer.write_all(self.#field_name.as_slice())?;
                )
            } else {
                quote!(
                    writer.write_all(&[0u8; Self::FIELD_PADDING[#i]])?;
                    writer.write_all(self.#field_name.as_slice())?;
                )
            }
        });
        quote!(
            fn expected_length(&self) -> usize {
                Self::TOTAL_SIZE
            }
            fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
                #( #fields )*
                Ok(())
            }
        )
    }
}

impl ImplBuilder for ast::FixVec {
    fn impl_builder_internal(&self) -> m4::TokenStream {
        let write_inners = if self.item_padding == 0 {
            quote!(
                writer.write_all(self.0[0].as_slice())?;
                for inner in &self.0[1..] {
                    writer.write_all(inner.as_slice())?;
                }
            )
        } else {
            quote!(
                writer.write_all(self.0[0].as_slice())?;
                for inner in &self.0[1..] {
                    writer.write_all(&[0u8; Self::ITEM_PADDING])?;
                    writer.write_all(inner.as_slice())?;
                }
            )
        };
        quote!(
            fn expected_length(&self) -> usize {
                let item_count = self.0.len();
                if item_count == 0 {
                    Self::HEADER_SIZE
                } else {
                    Self::HEADER_SIZE
                        + Self::HEADER_PADDING
                        + (Self::ITEM_SIZE + Self::ITEM_PADDING) * item_count
                        - Self::ITEM_PADDING
                }
            }
            fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
                let item_count = self.0.len();
                writer.write_all(&molecule::pack_number(item_count as molecule::Number))?;
                if item_count != 0 {
                    writer.write_all(&[0u8; Self::HEADER_PADDING])?;
                    #write_inners
                }
                Ok(())
            }
        )
    }
}

impl ImplBuilder for ast::DynVec {
    fn impl_builder_internal(&self) -> m4::TokenStream {
        quote!(
            fn expected_length(&self) -> usize {
                self.0
                    .iter()
                    .fold(
                        Self::HEADER_BASE_SIZE + molecule::NUMBER_SIZE * self.0.len(),
                        |total_size, inner| {
                            let padding_size = Self::ITEM_ALIGNMENT.calc_padding(total_size);
                            total_size + padding_size + inner.as_slice().len()
                        }
                    )
            }
            fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
                use molecule::write_padding;
                let item_count = self.0.len();
                if item_count == 0 {
                    let total_size = Self::HEADER_BASE_SIZE;
                    writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
                    writer.write_all(&molecule::pack_number(item_count as molecule::Number))?;
                } else {
                    let (total_size, offsets, paddings) = self.0.iter().fold(
                        (
                            Self::HEADER_BASE_SIZE + molecule::NUMBER_SIZE * item_count,
                            Vec::with_capacity(item_count),
                            Vec::with_capacity(item_count),
                        ),
                        |(prev_end, mut offsets, mut paddings), inner| {
                            let inner_slice = inner.as_slice();
                            let padding_size = if inner_slice.is_empty() {
                                0
                            } else {
                                Self::ITEM_ALIGNMENT.calc_padding(prev_end)
                            };
                            paddings.push(padding_size);
                            let start = prev_end + padding_size;
                            offsets.push(start);
                            let end = start + inner_slice.len();
                            (end, offsets, paddings)
                        },
                    );
                    writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
                    writer.write_all(&molecule::pack_number(item_count as molecule::Number))?;
                    for offset in offsets.into_iter() {
                        writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
                    }
                    for (padding_size, inner) in paddings.into_iter().zip(self.0.iter()) {
                        write_padding(writer, padding_size)?;
                        writer.write_all(inner.as_slice())?;
                    }
                }
                Ok(())
            }
        )
    }
}

impl ImplBuilder for ast::Table {
    fn impl_builder_internal(&self) -> m4::TokenStream {
        if self.inner.is_empty() {
            quote!(
                fn expected_length(&self) -> usize {
                    Self::HEADER_SIZE
                }
                fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
                    writer.write_all(&molecule::pack_number(Self::HEADER_SIZE as molecule::Number))?;
                    writer.write_all(&molecule::pack_number(Self::FIELD_COUNT as molecule::Number))?;
                    Ok(())
                }
            )
        } else {
            let field = &self
                .inner
                .iter()
                .map(|f| field_name(&f.name))
                .collect::<Vec<_>>();
            let index = &(0..(self.inner.len())).collect::<Vec<_>>();
            quote!(
                fn expected_length(&self) -> usize {
                    let mut total_size = Self::HEADER_SIZE;
                    #(
                        total_size += Self::FIELD_ALIGNMENT[#index].calc_padding(total_size)
                            + self.#field.as_slice().len();
                    )*
                    total_size
                }
                fn write<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<()> {
                    use molecule::write_padding;
                    let mut total_size = Self::HEADER_SIZE;
                    let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
                    let mut paddings = Vec::with_capacity(Self::FIELD_COUNT);
                    #(
                        let field_slice = self.#field.as_slice();
                        let padding_size = if field_slice.is_empty() {
                            0
                        } else {
                            Self::FIELD_ALIGNMENT[#index].calc_padding(total_size)
                        };
                        paddings.push(padding_size);
                        total_size += padding_size;
                        offsets.push(total_size);
                        total_size += field_slice.len();
                    )*
                    writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
                    writer.write_all(&molecule::pack_number(Self::FIELD_COUNT as molecule::Number))?;
                    for offset in offsets.into_iter() {
                        writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
                    }
                    #(
                        write_padding(writer, paddings[#index])?;
                        writer.write_all(self.#field.as_slice())?;
                    )*
                    Ok(())
                }
            )
        }
    }
}
