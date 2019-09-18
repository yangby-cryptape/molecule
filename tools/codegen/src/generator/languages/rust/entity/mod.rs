use proc_macro2 as m4;
use quote::quote;

use super::utilities::{entity_name, reader_name, usize_lit};
use crate::ast::verified::{DefaultContent, HasName};

mod implementation;

pub(super) trait GenEntity {
    fn gen_entity(&self) -> m4::TokenStream;
}

impl<T> GenEntity for T
where
    T: HasName
        + DefaultContent
        + super::display::ImplDisplay
        + super::constants::DefConstants
        + super::properties::DefProperties
        + super::getters::ImplGetters
        + implementation::ImplEntity,
{
    fn gen_entity(&self) -> m4::TokenStream {
        let entity = entity_name(self.name());
        let reader = reader_name(self.name());
        let default_content = self
            .default_content()
            .into_iter()
            .map(|b| usize_lit(b as usize));
        let display_stmts = self.impl_display();
        let constants = self.def_constants();
        let properties = self.def_properties();
        let getters = self.impl_getters_for_entity();
        let implementation = self.impl_entity();
        quote!(
            #[derive(Clone)]
            pub struct #entity(molecule::bytes::Bytes);

            impl ::std::fmt::Debug for #entity {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    use molecule::faster_hex::hex_string;
                    write!(f, "{}(0x{})", Self::NAME, hex_string(self.as_slice()).unwrap())
                }
            }

            impl ::std::fmt::Display for #entity {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    #display_stmts
                }
            }

            impl ::std::default::Default for #entity {
                fn default() -> Self {
                    let v: Vec<u8> = vec![#( #default_content, )*];
                    #entity::new_unchecked(v.into())
                }
            }

            impl #entity {
                #constants
                #properties
                #getters
                pub fn as_reader<'r>(&'r self) -> #reader<'r> {
                    #reader::new_unchecked(self.as_slice())
                }
            }

            #implementation
        )
    }
}
