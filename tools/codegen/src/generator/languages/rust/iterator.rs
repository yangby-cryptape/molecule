use proc_macro2 as m4;
use quote::quote;

use super::utilities::{entity_iterator_name, entity_name, reader_iterator_name, reader_name};
use crate::ast::verified::{self as ast, HasName};

pub(super) trait GenIterator {
    fn gen_iterator(&self) -> m4::TokenStream;
}

impl GenIterator for ast::FixVec {
    fn gen_iterator(&self) -> m4::TokenStream {
        gen_iterator_for_vector(self.name(), self.typ.name(), self.typ.is_atom())
    }
}

impl GenIterator for ast::DynVec {
    fn gen_iterator(&self) -> m4::TokenStream {
        gen_iterator_for_vector(self.name(), self.typ.name(), self.typ.is_atom())
    }
}

fn gen_iterator_for_vector(self_name: &str, inner_name: &str, is_atom: bool) -> m4::TokenStream {
    let entity_iterator = entity_iterator_name(self_name);
    let entity = entity_name(self_name);
    let entity_inner = entity_name(inner_name);
    let reader_iterator = reader_iterator_name(self_name);
    let reader = reader_name(self_name);
    let reader_inner = reader_name(inner_name);
    let common_part = quote!(
        pub struct #entity_iterator (#entity, usize, usize);
        impl ::std::iter::Iterator for #entity_iterator {
            type Item = #entity_inner;
            fn next(&mut self) -> Option<Self::Item> {
                if self.1 >= self.2 {
                    None
                } else {
                    let ret = self.0.get_unchecked(self.1);
                    self.1 += 1;
                    Some(ret)
                }
            }
        }
        impl ::std::iter::ExactSizeIterator for #entity_iterator {
            fn len(&self) -> usize {
                self.2 - self.1
            }
        }
        impl ::std::iter::IntoIterator for #entity {
            type Item = #entity_inner;
            type IntoIter = #entity_iterator;
            fn into_iter(self) -> Self::IntoIter {
                let len = self.len();
                #entity_iterator(self, 0, len)
            }
        }
    );
    if is_atom {
        common_part
    } else {
        quote!(
            #common_part

            impl<'r> #reader<'r> {
                pub fn iter<'t>(&'t self) -> #reader_iterator<'t, 'r> {
                    #reader_iterator(&self, 0, self.len())
                }
            }
            pub struct #reader_iterator<'t, 'r> (&'t #reader<'r>, usize, usize);
            impl<'t: 'r, 'r> ::std::iter::Iterator for #reader_iterator<'t, 'r> {
                type Item = #reader_inner<'t>;
                fn next(&mut self) -> Option<Self::Item> {
                    if self.1 >= self.2 {
                        None
                    } else {
                        let ret = self.0.get_unchecked(self.1);
                        self.1 += 1;
                        Some(ret)
                    }
                }
            }
            impl<'t: 'r, 'r> ::std::iter::ExactSizeIterator for #reader_iterator<'t, 'r> {
                fn len(&self) -> usize {
                    self.2 - self.1
                }
            }
        )
    }
}
