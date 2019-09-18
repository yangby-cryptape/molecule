#[derive(Debug)]
pub(crate) struct ItemDecl {
    pub(crate) typ: String,
}

#[derive(Debug)]
pub(crate) struct FieldDecl {
    pub(crate) name: String,
    pub(crate) typ: String,
}

#[derive(Debug)]
pub(crate) struct OptionDecl {
    pub(crate) name: String,
    pub(crate) typ: String,
}

#[derive(Debug)]
pub(crate) struct UnionDecl {
    pub(crate) name: String,
    pub(crate) inner: Vec<ItemDecl>,
}

#[derive(Debug)]
pub(crate) struct ArrayDecl {
    pub(crate) name: String,
    pub(crate) typ: String,
    pub(crate) length: usize,
}

#[derive(Debug)]
pub(crate) struct StructDecl {
    pub(crate) name: String,
    pub(crate) inner: Vec<FieldDecl>,
}

#[derive(Debug)]
pub(crate) struct VectorDecl {
    pub(crate) name: String,
    pub(crate) typ: String,
}

#[derive(Debug)]
pub(crate) struct TableDecl {
    pub(crate) name: String,
    pub(crate) inner: Vec<FieldDecl>,
}

#[derive(Debug)]
pub(crate) enum TopDecl {
    Option_(OptionDecl),
    Union(UnionDecl),
    Array(ArrayDecl),
    Struct(StructDecl),
    Vector(VectorDecl),
    Table(TableDecl),
}

#[derive(Debug)]
pub(crate) struct ImportStmt {
    pub(crate) name: String,
    pub(crate) path: Vec<String>,
    pub(crate) depth: usize,
}

#[derive(Debug, Default)]
pub(crate) struct Ast {
    pub(crate) imports: Vec<ImportStmt>,
    pub(crate) decls: Vec<TopDecl>,
}

macro_rules! impl_into_top_decl_for {
    ($item:ident, $decl:ident) => {
        impl From<$decl> for TopDecl {
            fn from(decl: $decl) -> Self {
                TopDecl::$item(decl)
            }
        }
    };
}

impl_into_top_decl_for!(Option_, OptionDecl);
impl_into_top_decl_for!(Union, UnionDecl);
impl_into_top_decl_for!(Array, ArrayDecl);
impl_into_top_decl_for!(Struct, StructDecl);
impl_into_top_decl_for!(Vector, VectorDecl);
impl_into_top_decl_for!(Table, TableDecl);

macro_rules! impl_methods_for_decl {
    ($decl:ident) => {
        impl $decl {
            pub(crate) fn name(&self) -> &str {
                &self.name
            }
        }
    };
}

impl_methods_for_decl!(FieldDecl);
impl_methods_for_decl!(OptionDecl);
impl_methods_for_decl!(UnionDecl);
impl_methods_for_decl!(ArrayDecl);
impl_methods_for_decl!(StructDecl);
impl_methods_for_decl!(VectorDecl);
impl_methods_for_decl!(TableDecl);

impl TopDecl {
    pub(crate) fn name(&self) -> &str {
        match self {
            TopDecl::Option_(inner) => inner.name(),
            TopDecl::Union(inner) => inner.name(),
            TopDecl::Array(inner) => inner.name(),
            TopDecl::Struct(inner) => inner.name(),
            TopDecl::Vector(inner) => inner.name(),
            TopDecl::Table(inner) => inner.name(),
        }
    }
}

impl Ast {
    pub(crate) fn add_import(&mut self, stmt: ImportStmt) {
        self.imports.push(stmt);
    }

    pub(crate) fn add_decl(&mut self, decl: impl Into<TopDecl>) {
        self.decls.push(decl.into());
    }
}
