use std::path::Path;

use tree::{Symbols, TokenExpression, Tree};

/// This module contains the language logic. Operators, functions, et cetera.
pub mod logic;
/// This module holds the syntax that composes the language, such as its parsed tree.
pub mod tree;

pub struct Parser<'a> {
    pub tree: Tree<'a>,
    pub contents: String,
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
        let mut expressions: Vec<TokenExpression> = Vec::new();
        let mut current_expr = TokenExpression::new();
        for (index, char) in char_iterator.into_iter().enumerate() {
            // try to get a known symbol from this char
            if let Ok(sym) = Symbols::try_from(char) {
                match sym {
                    Symbols::LPAREN if current_expr.delimiters.0 == None => {
                        current_expr.insert_opening(index);
                    }
                    Symbols::RPAREN if current_expr.is_unclosed() == true => {
                        current_expr.insert_closing(index);
                    }
                    Symbols::LPAREN if current_expr.is_unclosed() == true => {
                        panic!("TODO: nested expression")
                    }
                    _ => (),
                }
            } else {
            }
        }
        // finished iterating, check if it's closed
        if current_expr.is_unclosed() {
            panic!("The given expression has not been closed.");
        }
        println!("{:?}", current_expr);
    }
}
