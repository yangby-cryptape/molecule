use proc_macro2 as m4;
use quote::quote;

use super::super::utilities::{entity_name, entity_union_name, field_name, func_name, usize_lit};
use crate::ast::verified::{self as ast, HasName};

pub(in super::super) trait ImplSetters {
    fn impl_setters(&self) -> m4::TokenStream;
}

impl ImplSetters for ast::Option_ {
    fn impl_setters(&self) -> m4::TokenStream {
        let inner = entity_name(self.typ.name());
        quote!(
            pub fn set(mut self, v: Option<#inner>) -> Self {
                self.0 = v;
                self
            }
        )
    }
}

impl ImplSetters for ast::Union {
    fn impl_setters(&self) -> m4::TokenStream {
        let entity_union = entity_union_name(self.name());
        quote!(
            pub fn set<I>(mut self, v: I) -> Self
            where
                I: ::std::convert::Into<#entity_union>
            {
                self.0 = v.into();
                self
            }
        )
    }
}

impl ImplSetters for ast::Array {
    fn impl_setters(&self) -> m4::TokenStream {
        let inner = entity_name(self.typ.name());
        let item_count = usize_lit(self.item_count);
        let entire_setter = quote!(
            pub fn set(mut self, v: [#inner; #item_count]) -> Self {
                self.0 = v;
                self
            }
        );
        let each_setter = (0..self.item_count)
            .map(|idx| {
                let index = usize_lit(idx);
                let func = func_name(&format!("nth{}", idx));
                quote!(
                    pub fn #func(mut self, v: #inner) -> Self {
                        self.0[#index] = v;
                        self
                    }
                )
            })
            .collect::<Vec<_>>();
        quote!(
            #entire_setter
            #( #each_setter )*
        )
    }
}

impl ImplSetters for ast::Struct {
    fn impl_setters(&self) -> m4::TokenStream {
        impl_setters_for_struct_or_table(&self.inner[..])
    }
}

impl ImplSetters for ast::FixVec {
    fn impl_setters(&self) -> m4::TokenStream {
        impl_setters_for_vector(self.typ.name())
    }
}

impl ImplSetters for ast::DynVec {
    fn impl_setters(&self) -> m4::TokenStream {
        impl_setters_for_vector(self.typ.name())
    }
}

impl ImplSetters for ast::Table {
    fn impl_setters(&self) -> m4::TokenStream {
        impl_setters_for_struct_or_table(&self.inner[..])
    }
}

fn impl_setters_for_struct_or_table(inner: &[ast::FieldDecl]) -> m4::TokenStream {
    let each_setter = inner
        .iter()
        .map(|f| {
            let field_name = field_name(&f.name);
            let field_type = entity_name(f.typ.name());
            quote!(
                pub fn #field_name(mut self, v: #field_type) -> Self {
                    self.#field_name = v;
                    self
                }
            )
        })
        .collect::<Vec<_>>();
    quote!(
        #( #each_setter )*
    )
}

fn impl_setters_for_vector(inner_name: &str) -> m4::TokenStream {
    let inner = entity_name(&inner_name);
    quote!(
        pub fn set(mut self, v: Vec<#inner>) -> Self {
            self.0 = v;
            self
        }
        pub fn push(mut self, v: #inner) -> Self {
            self.0.push(v);
            self
        }
        pub fn extend<T: ::std::iter::IntoIterator<Item=#inner>>(mut self, iter: T) -> Self {
            for elem in iter {
                self.0.push(elem);
            }
            self
        }
    )
}
