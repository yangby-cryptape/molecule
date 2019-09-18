use proc_macro2 as m4;
use quote::quote;

use super::utilities::builder_name;
use crate::ast::verified::HasName;

mod definition;
mod implementation;
mod setters;

pub(super) trait GenBuilder {
    fn gen_builder(&self) -> m4::TokenStream;
}

impl<T> GenBuilder for T
where
    T: HasName
        + definition::DefBuilder
        + super::constants::DefConstants
        + setters::ImplSetters
        + implementation::ImplBuilder,
{
    fn gen_builder(&self) -> m4::TokenStream {
        let builder = builder_name(self.name());
        let definition = self.def_builder();
        let constants = self.def_constants();
        let setters = self.impl_setters();
        let implementation = self.impl_builder();
        quote!(
            #definition

            impl #builder {
                #( #constants )*
                #( #setters )*
            }

            #implementation
        )
    }
}
