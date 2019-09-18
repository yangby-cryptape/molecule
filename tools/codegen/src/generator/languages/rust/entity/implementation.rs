use proc_macro2 as m4;
use quote::quote;

use super::super::utilities::{builder_name, entity_name, field_name, func_name, reader_name};
use crate::ast::verified::{self as ast, HasName};

pub(in super::super) trait ImplEntity: HasName {
    fn impl_entity_internal(&self) -> m4::TokenStream;

    fn impl_entity(&self) -> m4::TokenStream {
        let entity = entity_name(self.name());
        let entity_string = entity.to_string();
        let reader = reader_name(self.name());
        let builder = builder_name(self.name());
        let internal = self.impl_entity_internal();
        quote!(
            impl molecule::prelude::Entity for #entity {
                type Builder = #builder;
                const NAME: &'static str = #entity_string;
                fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
                    #entity(data)
                }
                fn as_bytes(&self) -> molecule::bytes::Bytes {
                    self.0.clone()
                }
                fn as_slice(&self) -> &[u8] {
                    &self.0[..]
                }
                fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
                    #reader::from_slice(slice).map(|reader| reader.to_entity())
                }
                fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
                    #reader::from_compatible_slice(slice).map(|reader| reader.to_entity())
                }
                fn new_builder() -> Self::Builder {
                    ::std::default::Default::default()
                }
                #internal
            }
        )
    }
}

impl ImplEntity for ast::Option_ {
    fn impl_entity_internal(&self) -> m4::TokenStream {
        quote!(
            fn as_builder(self) -> Self::Builder {
                Self::new_builder().set(self.to_opt())
            }
        )
    }
}

impl ImplEntity for ast::Union {
    fn impl_entity_internal(&self) -> m4::TokenStream {
        quote!(
            fn as_builder(self) -> Self::Builder {
                Self::new_builder().set(self.to_enum())
            }
        )
    }
}

impl ImplEntity for ast::Array {
    fn impl_entity_internal(&self) -> m4::TokenStream {
        let items = (0..self.item_count)
            .map(|idx| func_name(&format!("nth{}", idx)))
            .map(|func| quote!(self.#func()));
        quote!(
            fn as_builder(self) -> Self::Builder {
                Self::new_builder().set([ #( #items, )* ])
            }
        )
    }
}

impl ImplEntity for ast::Struct {
    fn impl_entity_internal(&self) -> m4::TokenStream {
        let fields = self.inner.iter().map(|f| field_name(&f.name));
        let fields_func = fields.clone();
        quote!(
            fn as_builder(self) -> Self::Builder {
                Self::new_builder()
                    #( .#fields(self.#fields_func()) )*
            }
        )
    }
}

impl ImplEntity for ast::FixVec {
    fn impl_entity_internal(&self) -> m4::TokenStream {
        quote!(
            fn as_builder(self) -> Self::Builder {
                Self::new_builder().extend(self.into_iter())
            }
        )
    }
}

impl ImplEntity for ast::DynVec {
    fn impl_entity_internal(&self) -> m4::TokenStream {
        quote!(
            fn as_builder(self) -> Self::Builder {
                Self::new_builder().extend(self.into_iter())
            }
        )
    }
}

impl ImplEntity for ast::Table {
    fn impl_entity_internal(&self) -> m4::TokenStream {
        let fields = self.inner.iter().map(|f| field_name(&f.name));
        let fields_func = fields.clone();
        quote!(
            fn as_builder(self) -> Self::Builder {
                Self::new_builder()
                    #( .#fields(self.#fields_func()) )*
            }
        )
    }
}
