use std::path::Path;

use tree::{Tree, Symbols};

/// This module contains the language logic. Operators, functions, et cetera.
pub mod logic;
/// This module holds the syntax that composes the language, such as its parsed tree.
pub mod tree;

pub struct Parser<'a> {
    tree: Tree<'a>,
    contents: String,
    // etc.
}

impl Parser<'_> {
    pub fn new() -> Self {
        Parser {
            tree: Tree::new(),
            contents: String::new(),
        }
    }

    /// Read file and send it to a string
    pub fn read_file(&mut self, path: &Path) {
        let bytes = std::fs::read(path).unwrap();
        let string = String::from_utf8(bytes).unwrap();
        self.contents = string;
    }

    pub fn parse(&mut self, contents: String) {
        self.contents = contents;
        let char_iterator = self.contents.chars();
        // start simple, by finding the first TokenExpression
        let mut unclosed_exp = false;
        for char in char_iterator.into_iter() {
            // try to get a known symbol from this char
            if let Ok(sym) = Symbols::try_from(char) {
                match sym {
                    Symbols::LPAREN if unclosed_exp == false => {
                        unclosed_exp = true;
                    },
                    Symbols::RPAREN if unclosed_exp == true => {
                        unclosed_exp = false;
                        // TEMP: check if we can get a closed sexp
                        panic!("Unfinished, but we got a full sexp.");
                    }
                    Symbols::RPAREN if unclosed_exp == false => {
                        panic!("Unexpected delimiter");
                    }
                    Symbols::LPAREN if unclosed_exp == true => {
                        panic!("TODO: nested expression")
                    }
                    _ => (),
                }
            } else {

            }
        }
    }
}
