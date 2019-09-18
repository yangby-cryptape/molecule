use proc_macro2 as m4;
use quote::quote;

use crate::ast::verified as ast;

pub(super) trait DefProperties {
    fn def_properties(&self) -> m4::TokenStream;
}

impl DefProperties for ast::Option_ {
    fn def_properties(&self) -> m4::TokenStream {
        quote!(
            pub fn is_none(&self) -> bool {
                self.0.is_empty()
            }

            pub fn is_some(&self) -> bool {
                !self.0.is_empty()
            }
        )
    }
}

impl DefProperties for ast::Union {
    fn def_properties(&self) -> m4::TokenStream {
        quote!(
            pub fn total_size(&self) -> usize {
                molecule::unpack_number(self.as_slice()) as usize
            }
            pub fn item_id(&self) -> molecule::Number {
                molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..])
            }
        )
    }
}

impl DefProperties for ast::Array {
    fn def_properties(&self) -> m4::TokenStream {
        quote!()
    }
}

impl DefProperties for ast::Struct {
    fn def_properties(&self) -> m4::TokenStream {
        quote!()
    }
}

impl DefProperties for ast::FixVec {
    fn def_properties(&self) -> m4::TokenStream {
        quote!(
            pub fn total_size(&self) -> usize {
                let item_count = self.item_count();
                if item_count == 0 {
                    Self::HEADER_SIZE
                } else {
                    Self::HEADER_SIZE
                        + Self::HEADER_PADDING
                        + (Self::ITEM_SIZE + Self::ITEM_PADDING) * self.item_count()
                        - Self::ITEM_PADDING
                }
            }
            pub fn item_count(&self) -> usize {
                molecule::unpack_number(self.as_slice()) as usize
            }

            pub fn len(&self) -> usize {
                self.item_count()
            }
            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }
        )
    }
}

impl DefProperties for ast::DynVec {
    fn def_properties(&self) -> m4::TokenStream {
        quote!(
            pub fn total_size(&self) -> usize {
                molecule::unpack_number(self.as_slice()) as usize
            }
            pub fn item_count(&self) -> usize {
                molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize
            }
            pub fn item_offsets(&self) ->  &[[u8; 4]] {
                molecule::unpack_number_vec(&self.as_slice()[(molecule::NUMBER_SIZE*2)..])
            }
            pub fn header_size(&self) -> usize {
                molecule::NUMBER_SIZE * (2 + self.item_count())
            }

            pub fn len(&self) -> usize {
                self.item_count()
            }
            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }
        )
    }
}

impl DefProperties for ast::Table {
    fn def_properties(&self) -> m4::TokenStream {
        quote!(
            pub fn total_size(&self) -> usize {
                molecule::unpack_number(self.as_slice()) as usize
            }
            pub fn field_count(&self) -> usize {
                molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize
            }
            pub fn field_offsets(&self) ->  &[[u8; 4]] {
                molecule::unpack_number_vec(&self.as_slice()[(molecule::NUMBER_SIZE * 2)..])
            }
            pub fn header_size(&self) -> usize {
                molecule::NUMBER_SIZE * (2 + self.field_count())
            }

            pub fn count_extra_fields(&self) -> usize {
                self.field_count() - Self::FIELD_COUNT
            }
            pub fn has_extra_fields(&self) -> bool {
                Self::FIELD_COUNT != self.field_count()
            }
        )
    }
}
