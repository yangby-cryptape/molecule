use crate::ast::{self, HasName as _};

pub(crate) trait ToIntermediate {
    type Ir;
    fn to_ir(&self) -> Self::Ir;
}

impl ToIntermediate for ast::Ast {
    type Ir = super::Ir;
    fn to_ir(&self) -> Self::Ir {
        Self::Ir {
            namespace: self.namespace().to_owned(),
            imports: self.imports().iter().map(ToIntermediate::to_ir).collect(),
            decls: self.decls().iter().map(|decl| decl.to_ir()).collect(),
        }
    }
}

impl ToIntermediate for ast::ImportStmt {
    type Ir = super::ImportStmt;
    fn to_ir(&self) -> Self::Ir {
        Self::Ir {
            name: self.name().to_owned(),
            paths: self.paths().to_owned(),
            path_supers: self.path_supers(),
        }
    }
}

impl ToIntermediate for ast::TopDecl {
    type Ir = super::TopDecl;
    fn to_ir(&self) -> Self::Ir {
        match self {
            Self::Primitive(_) => unreachable!(),
            Self::Option_(inner) => Self::Ir::Option_(inner.to_ir()),
            Self::Union(inner) => Self::Ir::Union(inner.to_ir()),
            Self::Array(inner) => Self::Ir::Array(inner.to_ir()),
            Self::Struct(inner) => Self::Ir::Struct(inner.to_ir()),
            Self::FixVec(inner) => Self::Ir::FixVec(inner.to_ir()),
            Self::DynVec(inner) => Self::Ir::DynVec(inner.to_ir()),
            Self::Table(inner) => Self::Ir::Table(inner.to_ir()),
        }
    }
}

impl ToIntermediate for ast::Option_ {
    type Ir = super::Option_;
    fn to_ir(&self) -> Self::Ir {
        Self::Ir {
            name: self.name().to_owned(),
            item: self.item().to_ir(),
            imported_depth: self.imported_depth(),
        }
    }
}

impl ToIntermediate for ast::Union {
    type Ir = super::Union;
    fn to_ir(&self) -> Self::Ir {
        Self::Ir {
            name: self.name().to_owned(),
            items: self.items().iter().map(ToIntermediate::to_ir).collect(),
            imported_depth: self.imported_depth(),
        }
    }
}

impl ToIntermediate for ast::Array {
    type Ir = super::Array;
    fn to_ir(&self) -> Self::Ir {
        Self::Ir {
            name: self.name().to_owned(),
            item: self.item().to_ir(),
            item_count: self.item_count(),
            imported_depth: self.imported_depth(),
        }
    }
}

impl ToIntermediate for ast::Struct {
    type Ir = super::Struct;
    fn to_ir(&self) -> Self::Ir {
        Self::Ir {
            name: self.name().to_owned(),
            fields: self.fields().iter().map(ToIntermediate::to_ir).collect(),
            imported_depth: self.imported_depth(),
        }
    }
}

impl ToIntermediate for ast::FixVec {
    type Ir = super::FixVec;
    fn to_ir(&self) -> Self::Ir {
        Self::Ir {
            name: self.name().to_owned(),
            item: self.item().to_ir(),
            imported_depth: self.imported_depth(),
        }
    }
}

impl ToIntermediate for ast::DynVec {
    type Ir = super::DynVec;
    fn to_ir(&self) -> Self::Ir {
        Self::Ir {
            name: self.name().to_owned(),
            item: self.item().to_ir(),
            imported_depth: self.imported_depth(),
        }
    }
}

impl ToIntermediate for ast::Table {
    type Ir = super::Table;
    fn to_ir(&self) -> Self::Ir {
        Self::Ir {
            name: self.name().to_owned(),
            fields: self.fields().iter().map(ToIntermediate::to_ir).collect(),
            imported_depth: self.imported_depth(),
        }
    }
}

impl ToIntermediate for ast::ItemDecl {
    type Ir = super::ItemDecl;
    fn to_ir(&self) -> Self::Ir {
        Self::Ir {
            typ: self.typ().name().to_owned(),
        }
    }
}

impl ToIntermediate for ast::FieldDecl {
    type Ir = super::FieldDecl;
    fn to_ir(&self) -> Self::Ir {
        Self::Ir {
            name: self.name().to_owned(),
            typ: self.typ().name().to_owned(),
        }
    }
}
