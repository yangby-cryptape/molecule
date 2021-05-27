use std::{ffi, fs, io::Read as _, path::Path, str::FromStr};

use pest::{error::Error as PestError, iterators::Pairs, Parser as _};
use same_file::is_same_file;

use crate::{
    ast::raw as ast,
    parser,
    utils::{self, PairsUtils as _},
};

impl<'i> utils::PairsUtils for Pairs<'i, parser::Rule> {
    fn next_string(&mut self) -> String {
        self.next().unwrap().as_str().to_owned()
    }

    fn next_usize(&mut self) -> usize {
        usize::from_str(self.next().unwrap().as_str()).unwrap()
    }

    fn next_item(&mut self) -> ast::ItemDecl {
        ast::ItemDecl {
            typ: self.next_string(),
        }
    }

    fn next_items(&mut self) -> Vec<ast::ItemDecl> {
        let mut ret = Vec::new();
        for item in self {
            if item.as_rule() != parser::Rule::item_decl {
                unreachable!()
            }
            let mut pair = item.into_inner();
            let node = ast::ItemDecl {
                typ: pair.next_string(),
            };
            pair.next_should_be_none();
            ret.push(node);
        }
        ret
    }

    fn next_fields(&mut self) -> Vec<ast::FieldDecl> {
        let mut ret = Vec::new();
        for field in self {
            if field.as_rule() != parser::Rule::field_decl {
                unreachable!()
            }
            let mut pair = field.into_inner();
            let node = ast::FieldDecl {
                name: pair.next_string(),
                typ: pair.next_string(),
            };
            pair.next_should_be_none();
            ret.push(node);
        }
        ret
    }

    fn next_import<P: AsRef<Path>>(
        &mut self,
        imported_base: &P,
        imported_depth: usize,
    ) -> ast::ImportStmt {
        let mut paths = Vec::new();
        let mut path_supers = 0;
        if let Some(inner) = self.next() {
            if inner.as_rule() != parser::Rule::path {
                unreachable!()
            }
            let mut pair = inner.into_inner();
            loop {
                if let Some(inner) = pair.peek() {
                    if inner.as_rule() == parser::Rule::path_super {
                        pair.next();
                        path_supers += 1;
                        continue;
                    }
                }
                break;
            }
            for inner in pair {
                paths.push(inner.as_str().to_owned())
            }
        }
        ast::ImportStmt {
            name: paths.pop().unwrap(),
            paths,
            path_supers,
            imported_base: imported_base.as_ref().to_path_buf(),
            imported_depth,
        }
    }

    fn next_should_be_none(mut self) {
        if self.next().is_some() {
            unreachable!()
        }
    }
}

impl utils::ParserUtils for parser::Parser {
    fn preprocess<P: AsRef<Path>>(path: &P) -> Result<ast::Ast, PestError<parser::Rule>> {
        let namespace = path
            .as_ref()
            .file_stem()
            .and_then(ffi::OsStr::to_str)
            .unwrap()
            .to_owned();

        let mut ast = ast::Ast {
            namespace,
            ..Default::default()
        };

        let mut imported_depth = 0;

        Self::preprocess_single(&mut ast, path, imported_depth)?;

        let mut path_bufs = Vec::new();

        let mut imports = Vec::new();

        while !ast.imports.is_empty() {
            imported_depth += 1;
            while !ast.imports.is_empty() {
                let stmt = ast.imports.remove(0);
                let mut path_buf = stmt.imported_base().clone();
                path_buf.pop();
                for _ in 0..stmt.path_supers() {
                    path_buf.push("..");
                }
                for p in stmt.paths() {
                    path_buf.push(p);
                }
                path_buf.push(stmt.name());
                path_buf.set_extension("mol");
                let path_new = path_buf.as_path();
                if is_same_file(path, &path_new).unwrap() {
                    panic!("found cyclic dependencie");
                }

                if path_bufs
                    .iter()
                    .any(|ref path_old| is_same_file(&path_old, &path_new).unwrap())
                {
                    continue;
                } else {
                    imports.push(stmt);
                    Self::preprocess_single(&mut ast, &path_new, imported_depth)?;
                    path_bufs.push(path_buf);
                }
            }
        }

        ast.imports = imports;

        Ok(ast)
    }
}

impl parser::Parser {
    fn preprocess_single<P: AsRef<Path>>(
        ast: &mut ast::Ast,
        path: &P,
        imported_depth: usize,
    ) -> Result<(), PestError<parser::Rule>> {
        let buffer = {
            let mut buffer = String::new();
            let mut file_in = fs::OpenOptions::new().read(true).open(&path).unwrap();
            file_in.read_to_string(&mut buffer).unwrap();
            buffer
        };
        let mut file_content = parser::InnerParser::parse(parser::Rule::grammar, &buffer)?;
        let grammar = file_content
            .next()
            .unwrap_or_else(|| panic!("grammar should only have one pair"));
        if file_content.peek().is_some() {
            panic!("grammar should only have only one pair");
        }
        let mut eoi = false;
        for pair in grammar.into_inner() {
            if eoi {
                panic!("grammar should have only one EOI");
            }
            match pair.as_rule() {
                parser::Rule::import_stmt => {
                    let mut pair = pair.into_inner();
                    let node = pair.next_import(path, imported_depth);
                    pair.next_should_be_none();
                    ast.add_import(node);
                }
                parser::Rule::option_decl => {
                    let mut pair = pair.into_inner();
                    let node = ast::OptionDecl {
                        name: pair.next_string(),
                        item: pair.next_item(),
                        imported_depth,
                    };
                    pair.next_should_be_none();
                    ast.add_decl(node);
                }
                parser::Rule::union_decl => {
                    let mut pair = pair.into_inner();
                    let node = ast::UnionDecl {
                        name: pair.next_string(),
                        items: pair.next_items(),
                        imported_depth,
                    };
                    pair.next_should_be_none();
                    ast.add_decl(node);
                }
                parser::Rule::array_decl => {
                    let mut pair = pair.into_inner();
                    let node = ast::ArrayDecl {
                        name: pair.next_string(),
                        item: pair.next_item(),
                        item_count: pair.next_usize(),
                        imported_depth,
                    };
                    pair.next_should_be_none();
                    ast.add_decl(node);
                }
                parser::Rule::struct_decl => {
                    let mut pair = pair.into_inner();
                    let node = ast::StructDecl {
                        name: pair.next_string(),
                        fields: pair.next_fields(),
                        imported_depth,
                    };
                    pair.next_should_be_none();
                    ast.add_decl(node);
                }
                parser::Rule::vector_decl => {
                    let mut pair = pair.into_inner();
                    let node = ast::VectorDecl {
                        name: pair.next_string(),
                        item: pair.next_item(),
                        imported_depth,
                    };
                    pair.next_should_be_none();
                    ast.add_decl(node);
                }
                parser::Rule::table_decl => {
                    let mut pair = pair.into_inner();
                    let node = ast::TableDecl {
                        name: pair.next_string(),
                        fields: pair.next_fields(),
                        imported_depth,
                    };
                    pair.next_should_be_none();
                    ast.add_decl(node);
                }
                parser::Rule::EOI => {
                    if eoi {
                        panic!("grammar could not have more than one EOI");
                    }
                    eoi = true;
                }
                _ => {
                    unreachable!();
                }
            }
        }
        if !eoi {
            panic!("grammar should have only one EOI");
        }
        Ok(())
    }
}
