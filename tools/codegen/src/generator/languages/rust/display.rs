use proc_macro2 as m4;
use quote::quote;

use super::utilities::func_name;
use crate::ast::verified as ast;

pub(super) trait ImplDisplay {
    fn impl_display(&self) -> m4::TokenStream;
}

impl ImplDisplay for ast::Option_ {
    fn impl_display(&self) -> m4::TokenStream {
        quote!(if let Some(v) = self.to_opt() {
            write!(f, "{}(Some({}))", Self::NAME, v)
        } else {
            write!(f, "{}(None)", Self::NAME)
        })
    }
}

impl ImplDisplay for ast::Union {
    fn impl_display(&self) -> m4::TokenStream {
        quote!(
            write!(f, "{}(", Self::NAME)?;
            self.to_enum().display_inner(f)?;
            write!(f, ")")
        )
    }
}

impl ImplDisplay for ast::Array {
    fn impl_display(&self) -> m4::TokenStream {
        if self.typ.is_atom() {
            quote!(
                use molecule::faster_hex::hex_string;
                let raw_data = hex_string(&self.raw_data()).unwrap();
                write!(f, "{}(0x{})", Self::NAME, raw_data)
            )
        } else {
            let display_items = (0..self.item_count).map(|idx| {
                let func = func_name(&format!("nth{}", idx));
                if idx == 0 {
                    quote!(write!(f, "{}", self.#func())?;)
                } else {
                    quote!(write!(f, ", {}", self.#func())?;)
                }
            });
            quote!(
                write!(f, "{} [", Self::NAME)?;
                #( #display_items )*
                write!(f, "]")
            )
        }
    }
}

impl ImplDisplay for ast::Struct {
    fn impl_display(&self) -> m4::TokenStream {
        let display_fields = self.inner.iter().enumerate().map(|(i, f)| {
            let field = f.name.clone();
            let func = func_name(&f.name);
            if i == 0 {
                quote!(write!(f, "{}: {}", #field, self.#func())?;)
            } else {
                quote!(write!(f, ", {}: {}", #field, self.#func())?;)
            }
        });
        quote!(
            write!(f, "{} {{ ", Self::NAME)?;
            #( #display_fields )*
            write!(f, " }}")
        )
    }
}

impl ImplDisplay for ast::FixVec {
    fn impl_display(&self) -> m4::TokenStream {
        if self.typ.is_atom() {
            quote!(
                use molecule::faster_hex::hex_string;
                let raw_data = hex_string(&self.raw_data()).unwrap();
                write!(f, "{}(0x{})", Self::NAME, raw_data)
            )
        } else {
            quote!(
                write!(f, "{} [", Self::NAME)?;
                for i in 0..self.len() {
                    if i == 0 {
                        write!(f, "{}", self.get_unchecked(i))?;
                    } else {
                        write!(f, ", {}", self.get_unchecked(i))?;
                    }
                }
                write!(f, "]")
            )
        }
    }
}

impl ImplDisplay for ast::DynVec {
    fn impl_display(&self) -> m4::TokenStream {
        quote!(
            write!(f, "{} [", Self::NAME)?;
            for i in 0..self.len() {
                if i == 0 {
                    write!(f, "{}", self.get_unchecked(i))?;
                } else {
                    write!(f, ", {}", self.get_unchecked(i))?;
                }
            }
            write!(f, "]")
        )
    }
}

impl ImplDisplay for ast::Table {
    fn impl_display(&self) -> m4::TokenStream {
        let display_fields = self.inner.iter().enumerate().map(|(i, f)| {
            let field = f.name.clone();
            let func = func_name(&f.name);
            if i == 0 {
                quote!(write!(f, "{}: {}", #field, self.#func())?;)
            } else {
                quote!(write!(f, ", {}: {}", #field, self.#func())?;)
            }
        });
        let display_unresolved = if self.inner.is_empty() {
            quote!(write!(f, ".. ({} fields)", extra_count)?;)
        } else {
            quote!(write!(f, ", .. ({} fields)", extra_count)?;)
        };
        quote!(
            write!(f, "{} {{ ", Self::NAME)?;
            #( #display_fields )*
            let extra_count = self.count_extra_fields();
            if extra_count != 0 {
                #display_unresolved
            }
            write!(f, " }}")
        )
    }
}
