use std::path::Path;

use syntax_tree::Tree;

/// This module contains the language logic. Operators, functions, et cetera.
pub mod atlas_logic;
/// This module holds the syntax that composes the language, such as its parsed tree.
pub mod syntax_tree;

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
        for char in char_iterator.into_iter() {
            println!("Next char: {}", char);
        }
    }
}
