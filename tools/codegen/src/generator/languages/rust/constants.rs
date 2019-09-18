use proc_macro2 as m4;
use quote::quote;

use super::utilities::{alignment_name, usize_lit};
use crate::ast::verified as ast;

pub(super) trait DefConstants {
    fn def_constants(&self) -> m4::TokenStream;
}

impl DefConstants for ast::Option_ {
    fn def_constants(&self) -> m4::TokenStream {
        let alignment = alignment_name(self.alignment);
        quote!(
            pub const ALIGNMENT: molecule::Alignment = molecule::Alignment::#alignment;
        )
    }
}

impl DefConstants for ast::Union {
    fn def_constants(&self) -> m4::TokenStream {
        let item_count = usize_lit(self.inner.len());
        let header_full_size = usize_lit(self.header_full_size);
        let alignment = alignment_name(self.alignment);
        quote!(
            pub const ITEM_COUNT: usize = #item_count;
            pub const HEADER_FULL_SIZE: usize = #header_full_size;
            pub const ALIGNMENT: molecule::Alignment = molecule::Alignment::#alignment;
        )
    }
}

impl DefConstants for ast::Array {
    fn def_constants(&self) -> m4::TokenStream {
        let total_size = usize_lit(self.total_size());
        let item_padding = usize_lit(self.item_padding);
        let item_size = usize_lit(self.item_size);
        let item_count = usize_lit(self.item_count);
        let item_alignment = alignment_name(self.item_alignment);
        let alignment = alignment_name(self.alignment);
        quote!(
            pub const TOTAL_SIZE: usize = #total_size;
            pub const ITEM_PADDING: usize = #item_padding;
            pub const ITEM_SIZE: usize = #item_size;
            pub const ITEM_COUNT: usize = #item_count;
            pub const ITEM_ALIGNMENT: molecule::Alignment = molecule::Alignment::#item_alignment;
            pub const ALIGNMENT: molecule::Alignment = molecule::Alignment::#alignment;
        )
    }
}

impl DefConstants for ast::Struct {
    fn def_constants(&self) -> m4::TokenStream {
        let total_size = usize_lit(self.total_size());
        let field_size = self.field_size.iter().map(|x| usize_lit(*x));
        let field_padding = self.field_padding.iter().map(|x| usize_lit(*x));
        let field_count = usize_lit(self.inner.len());
        let field_alignment = self.field_alignment.iter().map(|x| alignment_name(*x));
        let alignment = alignment_name(self.alignment);
        quote!(
            pub const TOTAL_SIZE: usize = #total_size;
            pub const FIELD_SIZE: [usize; #field_count]= [ #( #field_size, )* ];
            pub const FIELD_PADDING: [usize; #field_count]= [ #( #field_padding, )* ];
            pub const FIELD_COUNT: usize = #field_count;
            pub const FIELD_ALIGNMENT: [molecule::Alignment; #field_count]= [
                #( molecule::Alignment::#field_alignment, )* ];
            pub const ALIGNMENT: molecule::Alignment = molecule::Alignment::#alignment;
        )
    }
}

impl DefConstants for ast::FixVec {
    fn def_constants(&self) -> m4::TokenStream {
        let item_size = usize_lit(self.item_size);
        let item_padding = usize_lit(self.item_padding);
        let header_size = usize_lit(self.header_size);
        let header_padding = usize_lit(self.header_padding);
        let item_alignment = alignment_name(self.item_alignment);
        let alignment = alignment_name(self.alignment);
        quote!(
            pub const ITEM_SIZE: usize = #item_size;
            pub const ITEM_PADDING: usize = #item_padding;
            pub const HEADER_SIZE: usize = #header_size;
            pub const HEADER_PADDING: usize = #header_padding;
            pub const ITEM_ALIGNMENT: molecule::Alignment = molecule::Alignment::#item_alignment;
            pub const ALIGNMENT: molecule::Alignment = molecule::Alignment::#alignment;
        )
    }
}

impl DefConstants for ast::DynVec {
    fn def_constants(&self) -> m4::TokenStream {
        let header_base_size = usize_lit(self.header_base_size);
        let item_alignment = alignment_name(self.item_alignment);
        let alignment = alignment_name(self.alignment);
        quote!(
            pub const HEADER_BASE_SIZE: usize = #header_base_size;
            pub const ITEM_ALIGNMENT: molecule::Alignment = molecule::Alignment::#item_alignment;
            pub const ALIGNMENT: molecule::Alignment = molecule::Alignment::#alignment;
        )
    }
}

impl DefConstants for ast::Table {
    fn def_constants(&self) -> m4::TokenStream {
        let field_count = usize_lit(self.inner.len());
        let header_size = usize_lit(self.header_size);
        let field_alignment = self.field_alignment.iter().map(|x| alignment_name(*x));
        let alignment = alignment_name(self.alignment);
        quote!(
            pub const FIELD_COUNT: usize = #field_count;
            pub const HEADER_SIZE: usize = #header_size;
            pub const FIELD_ALIGNMENT: [molecule::Alignment; #field_count]= [
                #( molecule::Alignment::#field_alignment, )* ];
            pub const ALIGNMENT: molecule::Alignment = molecule::Alignment::#alignment;
        )
    }
}
