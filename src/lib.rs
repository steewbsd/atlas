use std::{path::Path};

use tree::{Symbols, TokenExpression, Tree};

use crate::tree::{ExpressionLocation, Token};

/// This module contains the language logic. Operators, functions, et cetera.
pub mod logic;
/// This module holds the syntax that composes the language, such as its parsed tree.
pub mod tree;

pub struct Parser {
    pub tree: Tree,
    pub contents: String,
    pub current_depth: usize,
    // vector with last index used in this depth
    // represents how many arguments (tokens) have been provided in each depth. As the vector
    // is ordered, to get the arguments of an specific depth, just depth_argument_len.get(depth)
    pub depth_argument_len: Vec<usize>,
    // etc.
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            tree: Tree::new(),
            contents: String::new(),
            current_depth: 0,
            depth_argument_len: Vec::new(),
        }
    }

    pub fn calculate_index(&self, depth: usize, index: usize) -> usize {
        let mut idx_sum: usize = 0;
        for (cur_depth, lens) in self.depth_argument_len.clone().into_iter().enumerate() {
            if cur_depth == depth {
                idx_sum += index;
            } else {
                idx_sum += lens;
            }
        }
        idx_sum
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
                    Symbols::LPAREN
                        if self.tree.peek_mut(self.current_depth).get_opening() == None =>
                    {
                        let current_expr = self.tree.peek_mut(self.current_depth);
                        current_expr.insert_opening(index);
                    }
                    Symbols::RPAREN if self.tree.peek(self.current_depth).is_unclosed() == true => {
                        let current_expr = self.tree.peek_mut(self.current_depth);
                        current_expr.insert_closing(index);
                        if current_expr.depth > 0 {
                            // go one expression up
                            self.current_depth -= 1;
                        }
                    }
                    Symbols::LPAREN if self.tree.peek(self.current_depth).is_unclosed() == true => {
                        // create a new expression and leave the other in the stack
                        self.tree.push(TokenExpression::new());
                        // increment this depth's index, otherwise, push a new depth to the vector
                        match self.depth_argument_len.get(self.current_depth) {
                            Some(len) => {
                                self.depth_argument_len.insert(self.current_depth, len + 1)
                            }
                            None => self.depth_argument_len.push(1),
                        }
                        self.tree
                            .peek_mut(self.current_depth)
                            .args
                            .push(Token::Expression(ExpressionLocation::new(
                                self.current_depth,
                                *self.depth_argument_len.get(self.current_depth).unwrap(),
                            )));
                        self.current_depth += 1;

                        // update the reference
                        let current_expr = self.tree.peek_mut(self.current_depth);
                        current_expr.insert_opening(index);
                        // increase the depth
                        current_expr.depth = self.current_depth;
                        //panic!("TODO: nested expression")
                    }
                    _ => (),
                }
            } else {
                if char == ' ' {
                    continue;
                }
                let kw = &self
                    .tree
                    .peek_mut(self.calculate_index(
                        self.current_depth,
                        0,
                        //                        *self.depth_argument_len.get(self.current_depth).unwrap(),
                    ))
                    .keyword;
                let kw = match kw {
                    Some(Token::Keyword(current_keyword)) => {
                        let new_keyword = format!("{}{}", current_keyword, char);
                        Some(Token::Keyword(new_keyword))
                    }
                    _ => Some(Token::Keyword(String::from(char))),
                };
                self.tree
                    .peek_mut(self.calculate_index(
                        self.current_depth,
                        0,
                        //*self.depth_argument_len.get(self.current_depth).unwrap(),
                    ))
                    .keyword = kw;

                // = Some(Token::Keyword(String::from(char)));
            }
        }
        // finished iterating, check if it's closed
        if self.tree.peek(self.current_depth).is_unclosed() {
            panic!("The given expression has not been closed.");
        }
        println!("{:?}", self.tree);
    }
}
