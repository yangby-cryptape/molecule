use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use super::raw::{Ast as RawAst, TopDecl as RawTopDecl};

use molecule::Alignment;

mod complete;
mod default_content;
mod has_name;

use complete::CompleteRawDecl;
pub(crate) use default_content::DefaultContent;
pub(crate) use has_name::HasName;

pub(crate) const ATOM_NAME: &str = "byte";
pub(crate) const ATOM_SIZE: usize = 1;
pub(crate) const ATOM_PRIMITIVE_NAME: &str = "Byte";

#[derive(Debug)]
pub(crate) struct Ast {
    pub(crate) decls: Vec<Rc<TopDecl>>,
}

#[derive(Debug)]
pub(crate) enum TopDecl {
    Atom(Atom),
    Option_(Option_),
    Union(Union),
    Array(Array),
    Struct(Struct),
    FixVec(FixVec),
    DynVec(DynVec),
    Table(Table),
}

#[derive(Debug)]
pub(crate) struct Atom {
    pub(crate) name: String,
    pub(crate) size: usize,
    pub(crate) alignment: Alignment,
}

#[derive(Debug)]
pub(crate) struct Option_ {
    pub(crate) name: String,
    pub(crate) alignment: Alignment,
    pub(crate) typ: Rc<TopDecl>,
}

#[derive(Debug)]
pub(crate) struct Union {
    pub(crate) name: String,
    pub(crate) header_full_size: usize,
    pub(crate) alignment: Alignment,
    pub(crate) inner: Vec<ItemDecl>,
}

#[derive(Debug)]
pub(crate) struct Array {
    pub(crate) name: String,
    pub(crate) item_size: usize,
    pub(crate) item_padding: usize,
    pub(crate) item_count: usize,
    pub(crate) item_alignment: Alignment,
    pub(crate) alignment: Alignment,
    pub(crate) typ: Rc<TopDecl>,
}

#[derive(Debug)]
pub(crate) struct Struct {
    pub(crate) name: String,
    pub(crate) field_size: Vec<usize>,
    pub(crate) field_padding: Vec<usize>,
    pub(crate) field_alignment: Vec<Alignment>,
    pub(crate) alignment: Alignment,
    pub(crate) inner: Vec<FieldDecl>,
}

#[derive(Debug)]
pub(crate) struct FixVec {
    pub(crate) name: String,
    pub(crate) item_size: usize,
    pub(crate) item_padding: usize,
    pub(crate) header_size: usize,
    pub(crate) header_padding: usize,
    pub(crate) item_alignment: Alignment,
    pub(crate) alignment: Alignment,
    pub(crate) typ: Rc<TopDecl>,
}

#[derive(Debug)]
pub(crate) struct DynVec {
    pub(crate) name: String,
    pub(crate) header_base_size: usize,
    pub(crate) item_alignment: Alignment,
    pub(crate) alignment: Alignment,
    pub(crate) typ: Rc<TopDecl>,
}

#[derive(Debug)]
pub(crate) struct Table {
    pub(crate) name: String,
    pub(crate) header_size: usize,
    pub(crate) field_alignment: Vec<Alignment>,
    pub(crate) alignment: Alignment,
    pub(crate) inner: Vec<FieldDecl>,
}

#[derive(Debug)]
pub(crate) struct ItemDecl {
    pub(crate) typ: Rc<TopDecl>,
}

#[derive(Debug)]
pub(crate) struct FieldDecl {
    pub(crate) name: String,
    pub(crate) typ: Rc<TopDecl>,
}

impl Array {
    pub(crate) fn total_size(&self) -> usize {
        (self.item_size + self.item_padding) * self.item_count - self.item_padding
    }
}

impl Struct {
    pub(crate) fn total_size(&self) -> usize {
        self.field_size.iter().sum::<usize>() + self.field_padding.iter().sum::<usize>()
    }
}

impl TopDecl {
    fn atom() -> Self {
        let atom = Atom {
            name: ATOM_NAME.to_owned(),
            size: ATOM_SIZE,
            alignment: Alignment::Byte1,
        };
        TopDecl::Atom(atom)
    }

    pub(crate) fn is_atom(&self) -> bool {
        match self {
            TopDecl::Atom(_) => true,
            _ => false,
        }
    }

    fn total_size(&self) -> Option<usize> {
        match self {
            TopDecl::Atom(ref typ) => Some(typ.size),
            TopDecl::Option_(_) => None,
            TopDecl::Union(_) => None,
            TopDecl::Array(ref typ) => Some(typ.total_size()),
            TopDecl::Struct(ref typ) => Some(typ.total_size()),
            TopDecl::FixVec(_) => None,
            TopDecl::DynVec(_) => None,
            TopDecl::Table(_) => None,
        }
    }

    fn alignment(&self) -> Alignment {
        match self {
            TopDecl::Atom(ref typ) => typ.alignment,
            TopDecl::Option_(ref typ) => typ.alignment,
            TopDecl::Union(ref typ) => typ.alignment,
            TopDecl::Array(ref typ) => typ.alignment,
            TopDecl::Struct(ref typ) => typ.alignment,
            TopDecl::FixVec(ref typ) => typ.alignment,
            TopDecl::DynVec(ref typ) => typ.alignment,
            TopDecl::Table(ref typ) => typ.alignment,
        }
    }

    fn default_content(&self) -> Vec<u8> {
        match self {
            TopDecl::Atom(_) => vec![0],
            TopDecl::Option_(ref typ) => typ.default_content(),
            TopDecl::Union(ref typ) => typ.default_content(),
            TopDecl::Array(ref typ) => typ.default_content(),
            TopDecl::Struct(ref typ) => typ.default_content(),
            TopDecl::FixVec(ref typ) => typ.default_content(),
            TopDecl::DynVec(ref typ) => typ.default_content(),
            TopDecl::Table(ref typ) => typ.default_content(),
        }
    }

    fn complete(raw: &RawTopDecl, deps: &HashMap<&str, Rc<Self>>) -> Option<Self> {
        match raw {
            RawTopDecl::Option_(raw_decl) => raw_decl.complete(deps),
            RawTopDecl::Union(raw_decl) => raw_decl.complete(deps),
            RawTopDecl::Array(raw_decl) => raw_decl.complete(deps),
            RawTopDecl::Struct(raw_decl) => raw_decl.complete(deps),
            RawTopDecl::Vector(raw_decl) => raw_decl.complete(deps),
            RawTopDecl::Table(raw_decl) => raw_decl.complete(deps),
        }
    }
}

impl Ast {
    pub(crate) fn new(raw: RawAst) -> Self {
        let mut decls_idx = HashMap::new();
        let mut decls_keys = HashSet::new();
        for decl in &raw.decls[..] {
            let name = decl.name();
            if name == ATOM_NAME || name == ATOM_PRIMITIVE_NAME {
                panic!("the name `{}` is reserved", name);
            }
            if decls_idx.insert(name, decl).is_some() || !decls_keys.insert(name) {
                panic!("the name `{}` is used more than once", name);
            };
        }
        let mut decls_result = HashMap::new();
        decls_result.insert(ATOM_NAME, Rc::new(TopDecl::atom()));
        loop {
            if decls_keys.is_empty() {
                break;
            }
            let incompleted = decls_keys.len();
            decls_keys.retain(|&name| {
                let decl_raw = decls_idx.get(name).unwrap();
                if let Some(decl) = TopDecl::complete(decl_raw, &decls_result) {
                    decls_result.insert(name, Rc::new(decl));
                    false
                } else {
                    true
                }
            });
            if decls_keys.len() == incompleted {
                panic!(
                    "there are {} types which are unable to be completed: {:?}",
                    incompleted, decls_keys
                );
            }
        }
        let mut decls = Vec::with_capacity(raw.decls.len());
        for decl in &raw.decls[..] {
            let result = decls_result.get(decl.name()).unwrap();
            decls.push(Rc::clone(result));
        }
        Self { decls }
    }
}
