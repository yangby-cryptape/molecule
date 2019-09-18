use std::io;

use crate::{Ast, Parser};

mod languages;

pub use languages::Language;

#[derive(Debug)]
pub(crate) struct Generator {
    ast: Ast,
}

impl Generator {
    pub(crate) fn new(input: &str) -> Self {
        let ast = Parser::parse(input);
        Self { ast }
    }

    pub(crate) fn generate<W>(&self, lang: Language, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        lang.generate(writer, &self.ast)
    }
}
