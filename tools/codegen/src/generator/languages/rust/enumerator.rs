use proc_macro2 as m4;
use quote::quote;

use super::utilities::{
    entity_name, entity_union_name, reader_name, reader_union_name, union_item_name, usize_lit,
};
use crate::ast::verified::{self as ast, HasName};

pub(in super::super) trait GenEnumerator {
    fn gen_enumerator(&self) -> m4::TokenStream;
}

impl GenEnumerator for ast::Union {
    #[allow(clippy::cognitive_complexity)]
    fn gen_enumerator(&self) -> m4::TokenStream {
        let entity_union = entity_union_name(self.name());
        let reader_union = reader_union_name(self.name());
        let entity_union_string = entity_union.to_string();
        let reader_union_string = reader_union.to_string();
        let inner_len = self.inner.len();
        let (
            ref entity_inners,
            ref reader_inners,
            ref union_items,
            ref union_ids,
            ref entity_union_item_paths,
            ref reader_union_item_paths,
        ) = {
            self.inner.iter().enumerate().fold(
                (
                    Vec::with_capacity(inner_len),
                    Vec::with_capacity(inner_len),
                    Vec::with_capacity(inner_len),
                    Vec::with_capacity(inner_len),
                    Vec::with_capacity(inner_len),
                    Vec::with_capacity(inner_len),
                ),
                |(
                    mut entity_inners,
                    mut reader_inners,
                    mut union_items,
                    mut union_ids,
                    mut entity_union_item_paths,
                    mut reader_union_item_paths,
                ),
                 (index, inner)| {
                    let inner_name = inner.typ.name();
                    let entity_name = entity_name(inner_name);
                    let reader_name = reader_name(inner_name);
                    let item_name = union_item_name(inner_name);
                    let item_id = usize_lit(index);
                    let entity_union_item_path = quote!(#entity_union::#item_name);
                    let reader_union_item_path = quote!(#reader_union::#item_name);
                    entity_inners.push(entity_name);
                    reader_inners.push(reader_name);
                    union_items.push(item_name);
                    union_ids.push(item_id);
                    entity_union_item_paths.push(entity_union_item_path);
                    reader_union_item_paths.push(reader_union_item_path);
                    (
                        entity_inners,
                        reader_inners,
                        union_items,
                        union_ids,
                        entity_union_item_paths,
                        reader_union_item_paths,
                    )
                },
            )
        };
        let union_items_string = &union_items
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let entity_default = {
            let inner = &self.inner[0];
            let item_name = union_item_name(inner.typ.name());
            quote!(#item_name(::core::default::Default::default()))
        };
        let code_union_definitions_and_impl_traits = quote!(
            #[derive(Debug, Clone)]
            pub enum #entity_union {
                #( #union_items(#entity_inners), )*
            }
            #[derive(Debug, Clone, Copy)]
            pub enum #reader_union<'r> {
                #( #union_items(#reader_inners<'r>), )*
            }

            impl ::core::default::Default for #entity_union {
                fn default() -> Self {
                    #entity_union::#entity_default
                }
            }

            impl ::core::fmt::Display for #entity_union {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        #(
                            #entity_union_item_paths(ref item) => {
                                write!(f, "{}::{}({})", Self::NAME, #union_items::NAME, item)
                            }
                        )*
                    }
                }
            }
            impl<'r> ::core::fmt::Display for #reader_union<'r> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        #(
                            #reader_union_item_paths(ref item) => {
                                write!(f, "{}::{}({})", Self::NAME, #union_items::NAME, item)
                            }
                        )*
                    }
                }
            }

            impl #entity_union {
                pub(crate) fn display_inner(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        #( #entity_union_item_paths(ref item) => write!(f, "{}", item), )*
                    }
                }
            }
            impl<'r> #reader_union<'r> {
                pub(crate) fn display_inner(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        #( #reader_union_item_paths(ref item) => write!(f, "{}", item), )*
                    }
                }
            }
        );
        let code_entity_item_into_union = union_items
            .iter()
            .zip(entity_inners.iter())
            .map(|(item_name, entity_name)| {
                quote!(
                    impl ::core::convert::From<#entity_name> for #entity_union {
                        fn from(item: #entity_name) -> Self {
                            #entity_union::#item_name(item)
                        }
                    }
                )
            })
            .collect::<Vec<_>>();
        let code_reader_item_into_union = union_items
            .iter()
            .zip(reader_inners.iter())
            .map(|(item_name, reader_name)| {
                quote!(
                    impl<'r> ::core::convert::From<#reader_name<'r>> for #reader_union<'r> {
                        fn from(item: #reader_name<'r>) -> Self {
                            #reader_union::#item_name(item)
                        }
                    }
                )
            })
            .collect::<Vec<_>>();
        let code_impl_entity_union = {
            quote!(
                impl #entity_union {
                    pub const NAME: &'static str = #entity_union_string;
                    pub fn as_bytes(&self) -> molecule::bytes::Bytes {
                        match self {
                            #( #entity_union_item_paths(item) => item.as_bytes(), )*
                        }
                    }
                    pub fn as_slice(&self) -> &[u8] {
                        match self {
                            #( #entity_union_item_paths(item) => item.as_slice(), )*
                        }
                    }
                    pub fn item_id(&self) -> molecule::Number {
                        match self {
                            #( #entity_union_item_paths(_) => #union_ids, )*
                        }
                    }
                    pub fn item_name(&self) -> &str {
                        match self {
                            #( #entity_union_item_paths(_) => #union_items_string, )*
                        }
                    }
                    pub fn as_reader<'r>(&'r self) -> #reader_union<'r> {
                        match self {
                            #( #entity_union_item_paths(item) => item.as_reader().into(), )*
                        }
                    }
                }
            )
        };
        let code_impl_reader_union = {
            quote!(
                impl<'r> #reader_union<'r> {
                    pub const NAME: &'r str = #reader_union_string;
                    pub fn as_slice(&self) -> &'r [u8] {
                        match self {
                            #( #reader_union_item_paths(item) => item.as_slice(), )*
                        }
                    }
                    pub fn item_id(&self) -> molecule::Number {
                        match self {
                            #( #reader_union_item_paths(_) => #union_ids, )*
                        }
                    }
                    pub fn item_name(&self) -> &str {
                        match self {
                            #( #reader_union_item_paths(_) => #union_items_string, )*
                        }
                    }
                }
            )
        };
        quote!(
            #code_union_definitions_and_impl_traits
            #( #code_entity_item_into_union )*
            #( #code_reader_item_into_union )*
            #code_impl_entity_union
            #code_impl_reader_union
        )
    }
}
