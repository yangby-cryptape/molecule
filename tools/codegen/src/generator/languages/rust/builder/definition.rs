use proc_macro2 as m4;
use quote::quote;

use super::super::utilities::{
    builder_name, entity_name, entity_union_name, field_name, usize_lit,
};
use crate::ast::verified::{self as ast, HasName};

pub(in super::super) trait DefBuilder {
    fn def_builder(&self) -> m4::TokenStream;
}

impl DefBuilder for ast::Option_ {
    fn def_builder(&self) -> m4::TokenStream {
        let builder = builder_name(self.name());
        let inner = entity_name(self.typ.name());
        quote!(
            #[derive(Debug, Default)]
            pub struct #builder (pub(crate) Option<#inner>);
        )
    }
}

impl DefBuilder for ast::Union {
    fn def_builder(&self) -> m4::TokenStream {
        let builder = builder_name(self.name());
        let entity_union = entity_union_name(self.name());
        quote!(
            #[derive(Debug, Default)]
            pub struct #builder (pub(crate) #entity_union);
        )
    }
}

impl DefBuilder for ast::Array {
    fn def_builder(&self) -> m4::TokenStream {
        let builder = builder_name(self.name());
        let inner = entity_name(self.typ.name());
        let item_count = usize_lit(self.item_count);
        let inner_array = (0..self.item_count)
            .map(|_| inner.clone())
            .collect::<Vec<_>>();
        quote!(
            pub struct #builder (pub(crate) [#inner; #item_count]);

            impl ::std::fmt::Debug for #builder {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "{}({:?})", Self::NAME, &self.0[..])
                }
            }

            impl ::std::default::Default for #builder {
                fn default() -> Self {
                    #builder([#(#inner_array::default(), )*])
                }
            }
        )
    }
}

impl DefBuilder for ast::Struct {
    fn def_builder(&self) -> m4::TokenStream {
        def_builder_for_struct_or_table(self.name(), &self.inner[..])
    }
}

impl DefBuilder for ast::FixVec {
    fn def_builder(&self) -> m4::TokenStream {
        def_builder_for_vector(self.name(), self.typ.name())
    }
}

impl DefBuilder for ast::DynVec {
    fn def_builder(&self) -> m4::TokenStream {
        def_builder_for_vector(self.name(), self.typ.name())
    }
}

impl DefBuilder for ast::Table {
    fn def_builder(&self) -> m4::TokenStream {
        def_builder_for_struct_or_table(self.name(), &self.inner[..])
    }
}

fn def_builder_for_struct_or_table(self_name: &str, inner: &[ast::FieldDecl]) -> m4::TokenStream {
    let builder = builder_name(self_name);
    let fields = inner.iter().map(|f| {
        let field_name = field_name(&f.name);
        let field_type = entity_name(f.typ.name());
        quote!(#field_name: #field_type,)
    });
    quote!(
        #[derive(Debug, Default)]
        pub struct #builder { #( pub(crate) #fields )* }
    )
}

fn def_builder_for_vector(self_name: &str, inner_name: &str) -> m4::TokenStream {
    let builder = builder_name(self_name);
    let inner = entity_name(&inner_name);
    quote!(
        #[derive(Debug, Default)]
        pub struct #builder (pub(crate) Vec<#inner>);
    )
}
