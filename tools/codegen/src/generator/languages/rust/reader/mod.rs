use proc_macro2 as m4;
use quote::quote;

use super::utilities::reader_name;
use crate::ast::verified::HasName;

mod implementation;
mod peek_length;

pub(super) trait GenReader {
    fn gen_reader(&self) -> m4::TokenStream;
}

impl<T> GenReader for T
where
    T: HasName
        + super::display::ImplDisplay
        + super::constants::DefConstants
        + super::properties::DefProperties
        + super::getters::ImplGetters
        + peek_length::DefPeekLength
        + implementation::ImplReader,
{
    fn gen_reader(&self) -> m4::TokenStream {
        let reader = reader_name(self.name());
        let display_stmts = self.impl_display();
        let constants = self.def_constants();
        let properties = self.def_properties();
        let getters = self.impl_getters_for_reader();
        let peek_length = self.def_peek_length();
        let implementation = self.impl_reader();
        quote!(
            #[derive(Clone, Copy)]
            pub struct #reader<'r>(&'r [u8]);

            impl<'r> ::std::fmt::Debug for #reader<'r> {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    use molecule::faster_hex::hex_string;
                    write!(f, "{}(0x{})", Self::NAME, hex_string(self.as_slice()).unwrap())
                }
            }

            impl<'r> ::std::fmt::Display for #reader<'r> {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    #display_stmts
                }
            }

            impl<'r> #reader<'r> {
                #constants
                #properties
                #getters
                #peek_length
            }

            #implementation
        )
    }
}
