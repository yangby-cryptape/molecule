use std::io;

use crate::ast::verified::{self as ast, HasName};

use molecule::Number;

fn append_number<W: io::Write>(writer: &mut W, value: Number) -> io::Result<()> {
    write!(writer, ", MolNum({:>3})", value)
}

fn append_type_name<W: io::Write, T: HasName>(writer: &mut W, typ: &T) -> io::Result<()> {
    let type_name = format!("Mol{}", typ.type_name());
    write!(writer, ", {:10} ", type_name)
}

pub(super) trait Generator: HasName {
    fn generate<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        self.generate_class(writer)?;
        Ok(())
    }

    fn generate_class<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        let cls_name = format!("MolCls{}", self.name());
        let type_name = format!("Mol{}", self.type_name());
        write!(writer, "#define {:32}    ( {:10}", cls_name, type_name)?;
        self.generate_class_internal(writer)?;
        writeln!(writer, " )")
    }

    fn generate_class_internal<W: io::Write>(&self, writer: &mut W) -> io::Result<()>;
}

impl Generator for ast::Option_ {
    fn generate_class_internal<W: io::Write>(&self, _writer: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl Generator for ast::Union {
    fn generate_class_internal<W: io::Write>(&self, _writer: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl Generator for ast::Array {
    fn generate_class_internal<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        append_number(writer, self.item_count as Number)?;
        append_number(writer, self.item_size as Number)?;
        append_number(writer, self.item_padding as Number)?;
        Ok(())
    }
}

impl Generator for ast::Struct {
    fn generate_class_internal<W: io::Write>(&self, _writer: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl Generator for ast::FixVec {
    fn generate_class_internal<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        append_number(writer, self.header_padding as Number)?;
        append_number(writer, self.item_size as Number)?;
        append_number(writer, self.item_padding as Number)?;
        Ok(())
    }
}

impl Generator for ast::DynVec {
    fn generate_class_internal<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        append_type_name(writer, self.typ.as_ref())?;
        Ok(())
    }
}

impl Generator for ast::Table {
    fn generate_class_internal<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        let field_count = self.inner.len();
        append_number(writer, field_count as Number)?;
        Ok(())
    }
}
