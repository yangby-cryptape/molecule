pub(crate) trait HasName {
    const TYPE_NAME: &'static str;
    fn name(&self) -> &str;
    fn type_name(&self) -> &str;
}

macro_rules! impl_has_name_for_decl {
    ($decl:ident, $type_name:ident) => {
        impl HasName for super::$decl {
            const TYPE_NAME: &'static str = stringify!($type_name);
            fn name(&self) -> &str {
                &self.name
            }
            fn type_name(&self) -> &str {
                Self::TYPE_NAME
            }
        }
    };
}

impl_has_name_for_decl!(Atom, Atom);
impl_has_name_for_decl!(Option_, Option);
impl_has_name_for_decl!(Union, Union);
impl_has_name_for_decl!(Array, Array);
impl_has_name_for_decl!(Struct, Struct);
impl_has_name_for_decl!(FixVec, FixVec);
impl_has_name_for_decl!(DynVec, DynVec);
impl_has_name_for_decl!(Table, Table);

impl HasName for super::TopDecl {
    const TYPE_NAME: &'static str = "TopDecl";

    fn name(&self) -> &str {
        match self {
            super::TopDecl::Atom(inner) => inner.name(),
            super::TopDecl::Option_(inner) => inner.name(),
            super::TopDecl::Union(inner) => inner.name(),
            super::TopDecl::Array(inner) => inner.name(),
            super::TopDecl::Struct(inner) => inner.name(),
            super::TopDecl::FixVec(inner) => inner.name(),
            super::TopDecl::DynVec(inner) => inner.name(),
            super::TopDecl::Table(inner) => inner.name(),
        }
    }

    fn type_name(&self) -> &str {
        match self {
            super::TopDecl::Atom(inner) => inner.type_name(),
            super::TopDecl::Option_(inner) => inner.type_name(),
            super::TopDecl::Union(inner) => inner.type_name(),
            super::TopDecl::Array(inner) => inner.type_name(),
            super::TopDecl::Struct(inner) => inner.type_name(),
            super::TopDecl::FixVec(inner) => inner.type_name(),
            super::TopDecl::DynVec(inner) => inner.type_name(),
            super::TopDecl::Table(inner) => inner.type_name(),
        }
    }
}
