use std::{convert::TryFrom, io};

use crate::ast::verified as ast;

mod c;
mod rust;

#[derive(Debug, Clone, Copy)]
pub enum Language {
    C,
    Rust,
}

pub(super) trait LanguageGenerator {
    fn generate<W>(writer: &mut W, ast: &ast::Ast) -> io::Result<()>
    where
        W: io::Write;
}

impl TryFrom<&str> for Language {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "c" => Ok(Language::C),
            "rust" => Ok(Language::Rust),
            lang => Err(format!("unsupport language: [{}]", lang)),
        }
    }
}

impl Language {
    pub(crate) fn extension(&self) -> &str {
        match *self {
            Language::C => "h",
            Language::Rust => "rs",
        }
    }

    pub(crate) fn generate<W>(self, writer: &mut W, ast: &ast::Ast) -> io::Result<()>
    where
        W: io::Write,
    {
        match self {
            Language::C => c::Generator::generate(writer, ast),
            Language::Rust => rust::Generator::generate(writer, ast),
        }
    }
}
