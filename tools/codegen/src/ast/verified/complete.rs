use std::{collections::HashMap, rc::Rc};

use molecule::{Alignment, NUMBER_SIZE};

use super::{super::raw, TopDecl};

macro_rules! impl_into_top_decl_for {
    ($type:ident) => {
        impl From<super::$type> for TopDecl {
            fn from(typ: super::$type) -> Self {
                TopDecl::$type(typ)
            }
        }
    };
}

impl_into_top_decl_for!(Atom);
impl_into_top_decl_for!(Option_);
impl_into_top_decl_for!(Union);
impl_into_top_decl_for!(Array);
impl_into_top_decl_for!(Struct);
impl_into_top_decl_for!(FixVec);
impl_into_top_decl_for!(DynVec);
impl_into_top_decl_for!(Table);

pub(super) trait CompleteRawDecl {
    fn complete(&self, deps: &HashMap<&str, Rc<TopDecl>>) -> Option<TopDecl>;
}

impl CompleteRawDecl for raw::OptionDecl {
    fn complete(&self, deps: &HashMap<&str, Rc<TopDecl>>) -> Option<TopDecl> {
        deps.get(self.typ.as_str()).map(|dep| {
            let name = self.name().to_owned();
            let typ = Rc::clone(dep);
            let alignment = typ.alignment();
            super::Option_ {
                name,
                alignment,
                typ,
            }
            .into()
        })
    }
}

impl CompleteRawDecl for raw::UnionDecl {
    fn complete(&self, deps: &HashMap<&str, Rc<TopDecl>>) -> Option<TopDecl> {
        if self.inner.is_empty() {
            panic!("the union ({}) is empty", self.name());
        }
        self.inner
            .iter()
            .map(|raw_item| {
                deps.get(raw_item.typ.as_str()).map(|dep| super::ItemDecl {
                    typ: Rc::clone(dep),
                })
            })
            .collect::<Option<Vec<_>>>()
            .map(|inner| {
                let name = self.name().to_owned();
                let alignment = Alignment::Byte8;
                let header_full_size = NUMBER_SIZE * 2;
                assert!(alignment.calc_padding(header_full_size) == 0);
                super::Union {
                    name,
                    header_full_size,
                    alignment,
                    inner,
                }
                .into()
            })
    }
}

impl CompleteRawDecl for raw::ArrayDecl {
    fn complete(&self, deps: &HashMap<&str, Rc<TopDecl>>) -> Option<TopDecl> {
        deps.get(self.typ.as_str()).map(|dep| {
            let item_size = dep.total_size().unwrap_or_else(|| {
                panic!(
                    "the inner type ({}) of array ({}) doesn't have fixed size",
                    self.typ,
                    self.name(),
                );
            });
            if item_size == 0 {
                panic!("the array ({}) has no size", self.name());
            }
            let name = self.name().to_owned();
            let typ = Rc::clone(dep);
            let item_count = self.length;
            let (item_alignment, alignment) = if dep.is_atom() {
                (Alignment::Byte1, Alignment::for_size(item_count))
            } else {
                (dep.alignment(), dep.alignment())
            };
            let item_padding = item_alignment.calc_padding(item_size);
            super::Array {
                name,
                item_size,
                item_padding,
                item_count,
                item_alignment,
                alignment,
                typ,
            }
            .into()
        })
    }
}

impl CompleteRawDecl for raw::StructDecl {
    fn complete(&self, deps: &HashMap<&str, Rc<TopDecl>>) -> Option<TopDecl> {
        let mut inner = Vec::with_capacity(self.inner.len());
        let mut field_size = Vec::with_capacity(self.inner.len());
        for raw_field in &self.inner[..] {
            let field_name = raw_field.name().to_owned();
            if let Some(dep) = deps.get(raw_field.typ.as_str()) {
                if let Some(item_size) = dep.total_size() {
                    field_size.push(item_size);
                } else {
                    panic!(
                        "the inner type ({}) in struct ({}) doesn't have fixed size",
                        field_name,
                        self.name(),
                    );
                }
                let field = super::FieldDecl {
                    name: field_name,
                    typ: Rc::clone(dep),
                };
                inner.push(field);
            } else {
                break;
            }
        }
        if inner.len() != self.inner.len() {
            return None;
        }
        if field_size.iter().sum::<usize>() == 0 {
            panic!("the struct ({}) has no size", self.name());
        }
        let field_alignment = inner
            .iter()
            .map(|field| field.typ.alignment())
            .collect::<Vec<_>>();
        let alignment = *field_alignment.iter().max().unwrap();
        let (_, field_padding) = field_size.iter().zip(field_alignment.iter()).fold(
            (0, Vec::with_capacity(field_size.len())),
            |(current_offset, mut padding), (size, align)| {
                let current_padding = align.calc_padding(current_offset);
                let next_offset = current_offset + size + current_padding;
                padding.push(current_padding);
                (next_offset, padding)
            },
        );
        let name = self.name().to_owned();
        Some(
            super::Struct {
                name,
                field_size,
                field_padding,
                field_alignment,
                alignment,
                inner,
            }
            .into(),
        )
    }
}

impl CompleteRawDecl for raw::VectorDecl {
    fn complete(&self, deps: &HashMap<&str, Rc<TopDecl>>) -> Option<TopDecl> {
        deps.get(self.typ.as_str()).map(|dep| {
            let name = self.name().to_owned();
            let typ = Rc::clone(dep);
            if let Some(item_size) = dep.total_size() {
                let header_size = NUMBER_SIZE;
                let item_alignment = typ.alignment();
                let item_padding = item_alignment.calc_padding(item_size);
                let header_padding = item_alignment.calc_padding(header_size);
                let header_alignment = Alignment::for_size(header_size);
                let alignment = if item_alignment > header_alignment {
                    item_alignment
                } else {
                    header_alignment
                };
                super::FixVec {
                    name,
                    item_size,
                    item_padding,
                    header_size,
                    header_padding,
                    item_alignment,
                    alignment,
                    typ,
                }
                .into()
            } else {
                let header_base_size = NUMBER_SIZE * 2;
                let item_alignment = typ.alignment();
                let header_alignment = Alignment::for_size(NUMBER_SIZE);
                let alignment = if item_alignment > header_alignment {
                    item_alignment
                } else {
                    header_alignment
                };
                super::DynVec {
                    name,
                    header_base_size,
                    item_alignment,
                    alignment,
                    typ,
                }
                .into()
            }
        })
    }
}

impl CompleteRawDecl for raw::TableDecl {
    fn complete(&self, deps: &HashMap<&str, Rc<TopDecl>>) -> Option<TopDecl> {
        self.inner
            .iter()
            .map(|raw_field| {
                let field_name = raw_field.name().to_owned();
                deps.get(raw_field.typ.as_str())
                    .map(|dep| super::FieldDecl {
                        name: field_name,
                        typ: Rc::clone(dep),
                    })
            })
            .collect::<Option<Vec<_>>>()
            .map(|inner| {
                let name = self.name().to_owned();
                let header_size = NUMBER_SIZE * (2 + inner.len());
                let field_alignment = inner
                    .iter()
                    .map(|field| field.typ.alignment())
                    .collect::<Vec<_>>();
                let alignment = Alignment::Byte8;
                super::Table {
                    name,
                    header_size,
                    field_alignment,
                    alignment,
                    inner,
                }
                .into()
            })
    }
}
