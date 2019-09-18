use std::io;

use super::{
    builder::GenBuilder, entity::GenEntity, enumerator::GenEnumerator, iterator::GenIterator,
    reader::GenReader,
};
use crate::ast::verified::{self as ast};

pub(super) trait Generator {
    fn generate<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write;
}

impl Generator for ast::Option_ {
    fn generate<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writeln!(writer, "{}", self.gen_entity())?;
        writeln!(writer, "{}", self.gen_reader())?;
        writeln!(writer, "{}", self.gen_builder())?;
        Ok(())
    }
}

impl Generator for ast::Union {
    fn generate<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writeln!(writer, "{}", self.gen_entity())?;
        writeln!(writer, "{}", self.gen_reader())?;
        writeln!(writer, "{}", self.gen_builder())?;
        writeln!(writer, "{}", self.gen_enumerator())?;
        Ok(())
    }
}

impl Generator for ast::Array {
    fn generate<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writeln!(writer, "{}", self.gen_entity())?;
        writeln!(writer, "{}", self.gen_reader())?;
        writeln!(writer, "{}", self.gen_builder())?;
        Ok(())
    }
}

impl Generator for ast::Struct {
    fn generate<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writeln!(writer, "{}", self.gen_entity())?;
        writeln!(writer, "{}", self.gen_reader())?;
        writeln!(writer, "{}", self.gen_builder())?;
        Ok(())
    }
}

impl Generator for ast::FixVec {
    fn generate<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writeln!(writer, "{}", self.gen_entity())?;
        writeln!(writer, "{}", self.gen_reader())?;
        writeln!(writer, "{}", self.gen_builder())?;
        writeln!(writer, "{}", self.gen_iterator())?;
        Ok(())
    }
}

impl Generator for ast::DynVec {
    fn generate<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writeln!(writer, "{}", self.gen_entity())?;
        writeln!(writer, "{}", self.gen_reader())?;
        writeln!(writer, "{}", self.gen_builder())?;
        writeln!(writer, "{}", self.gen_iterator())?;
        Ok(())
    }
}

impl Generator for ast::Table {
    fn generate<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writeln!(writer, "{}", self.gen_entity())?;
        writeln!(writer, "{}", self.gen_reader())?;
        writeln!(writer, "{}", self.gen_builder())?;
        Ok(())
    }
}
