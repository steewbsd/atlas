use std::path::Path;

use tree::{Symbols, TokenExpression, Tree};

/// This module contains the language logic. Operators, functions, et cetera.
pub mod logic;
/// This module holds the syntax that composes the language, such as its parsed tree.
pub mod tree;

pub struct Parser<'a> {
    pub tree: Tree<'a>,
    pub contents: String,
    pub current_depth: usize,
    // etc.
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser {
            tree: Tree::new(),
            contents: String::new(),
            current_depth: 0,
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
        println!("Parsing: {}", self.contents);
        // start simple, by finding the first TokenExpression
        // add first TokenExpression
        self.tree.push(TokenExpression::new());
        for (index, char) in char_iterator.into_iter().enumerate() {
            // try to get a known symboindex l from this char
            if let Ok(sym) = Symbols::try_from(char) {
                match sym {
                    Symbols::LPAREN if self.tree.peek(self.current_depth).get_opening() == None => {
                        let current_expr = self.tree.peek(self.current_depth);
                        current_expr.insert_opening(index);
                    }
                    Symbols::RPAREN if self.tree.peek(self.current_depth).is_unclosed() == true => {
                        let current_expr = self.tree.peek(self.current_depth);
                        current_expr.insert_closing(index);
                        if current_expr.depth > 0 {
                           // go one expression up
                           self.current_depth -= 1;
                        }
                    }
                    Symbols::LPAREN if self.tree.peek(self.current_depth).is_unclosed() == true => {
                        // create a new expression and leave the other in the stack
                        self.tree.push(TokenExpression::new());
                        self.current_depth += 1;
                        // update the reference
                        let current_expr = self.tree.peek(self.current_depth);
                        current_expr.insert_opening(index);
                        // increase the depth
                        current_expr.depth = self.current_depth;
                        //panic!("TODO: nested expression")
                    }
                    _ => (),
                }
            }
        }
        // finished iterating, check if it's closed
        if self.tree.peek(self.current_depth).is_unclosed() {
            panic!("The given expression has not been closed.");
        }
        println!("{:?}", self.tree);
    }
}
