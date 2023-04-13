use std::{collections::HashMap, path::Path};

use tree::{Symbols, TokenExpression, Tree};

use crate::tree::Token;

/// This module contains the language logic. Operators, functions, et cetera.
pub mod logic;
/// This module holds the syntax that composes the language, such as its parsed tree.
pub mod tree;

pub struct Parser {
    tree: Tree,
    contents: String,
    current_depth: usize,
    // vector with last index used in this depth
    // represents how many arguments (tokens) have been provided in each depth. As the vector
    // is ordered, to get the arguments of an specific depth, just depth_argument_len.get(depth)
    n_expressions_in_depth: Vec<usize>,
    // etc.
}

impl Parser {
    // create a new parser instance
    pub fn new() -> Self {
        Parser {
            tree: Tree::new(),
            contents: String::new(),
            current_depth: 0,
            n_expressions_in_depth: Vec::new(),
        }
    }
    //
    pub fn reduce_all(&mut self) {
        // get the lowest depth
        // HashMap<(depth, index), Token>
        let mut results_map: HashMap<(usize, usize), Token> = HashMap::new();
        // sort the expressions by depth (descending)
        self.tree
            .expressions
            .sort_unstable_by(|exp1, exp2| exp1.depth.cmp(&exp2.depth).reverse());
        // println!("{:?}", self.tree);
        // iterate the re-ordered vector of expressions, and store their result in a hash map.
        for exp in self.tree.expressions.iter_mut() {
            if !exp.args.is_empty() {
                for arg in exp.args.iter_mut() {
                    match arg {
                        Token::Expression((depth, index)) => {
                            if let Some(val_to_update) = results_map.get_mut(&(*depth + 1, *index))
                            {
                                // print!("Replaced {:?}", arg);
                                let _ = std::mem::replace(arg, val_to_update.clone());
                                // println!(" with {:?}", arg);
                            }
                        }
                        _ => continue,
                    }
                }
            }
            // println!("-------------");
            // println!("Reducing: {:?}", exp);
            let result = exp.reduce();
            println!("Result: {:?}", result);
            results_map.insert((exp.depth, exp.index), result);
        }
    }
    // get an expression from depth and index.
    pub fn get_from_depth_and_idx(&mut self, depth: usize, idx: usize) -> &TokenExpression {
        let old_depth = self.current_depth;
        self.current_depth = depth;
        let te = &self.tree.expressions[self.calculate_index(idx)];
        self.current_depth = old_depth;
        te
    }

    fn get_last(&self) -> &TokenExpression {
        let depth = self.n_expressions_in_depth[self.current_depth];
        if depth == 0 {
            self.tree
                .peek(self.calculate_index(self.n_expressions_in_depth[self.current_depth]))
        } else {
            self.tree
                .peek(self.calculate_index(self.n_expressions_in_depth[self.current_depth] - 1))
        }
    }
    fn get_last_mut(&mut self) -> &mut TokenExpression {
        let depth = self.n_expressions_in_depth[self.current_depth];
        if depth == 0 {
            self.tree
                .peek_mut(self.calculate_index(self.n_expressions_in_depth[self.current_depth]))
        } else {
            self.tree
                .peek_mut(self.calculate_index(self.n_expressions_in_depth[self.current_depth] - 1))
        }
    }
    // This function calculates the access index for an element at (depth, index). This is neccessary
    // because for optimization reasons, we store our expressions in a single Vec<> instead of Vec<Vec<>>, which
    // would consist of a vector of depths with each depth having multiple TokenExpressions. As it is stored in a single
    // vec, we could have something like this: [Depth0Idx0, Depth0Idx1, Depth1Idx0], which means that we need to calculate
    // the correct index to access Depth1Idx0, instead of using index 1.
    // This function gets the number of arguments per depth, which is stored
    // in n_expressions_in_depth as a vector. Each index corresponds to a depth and the value corresponds to the amount
    // of TokenExpressions that are in that depth. So, to get Depth1Idx0 we should access (Depth0args + Depth1Idx) - 1 =
    // (2 + 1) - 1 = 2, the third element in the self.tree.expressions vector.
    fn calculate_index(&self, index: usize) -> usize {
        let mut idx_sum: usize = 0;
        for (cur_depth, lens) in self.n_expressions_in_depth.clone().into_iter().enumerate() {
            if cur_depth == self.current_depth {
                // if we are in the desired depth, only add up to the required index, and stop adding
                idx_sum += index;
                break;
            } else {
                // else if we are in the previous depths, we need to sum the entire length of the vector.
                idx_sum += lens;
            }
        }
        // index starts at 0
        /*         if idx_sum > 0 {
            idx_sum -= 1;
        } */
        idx_sum
    }

    /// Read file and send it to a string
    pub fn read_file(&mut self, path: &Path) {
        let bytes = std::fs::read(path).unwrap();
        let string = String::from_utf8(bytes).unwrap();
        self.contents = string;
    }

    pub fn parse(&mut self, contents: String) {
        self.contents = contents.clone();
        println!("Parsing: {}", self.contents);
        /* for (i, c) in self.contents.chars().into_iter().enumerate() {
            print!("{}:{} ", i, c);
        }
        println!(""); */
        println!("");
        let mut currently_parsing_token: String = String::new();
        let char_iterator = contents.chars();
        // start simple, by finding the first TokenExpression
        // add first TokenExpression
        self.tree.push(TokenExpression::new());
        self.n_expressions_in_depth.push(0);
        for (index, char) in char_iterator.into_iter().enumerate() {
            // println!("Parsing char: {} from depth: {}", char, self.current_depth);
            // try to get a known symboindex l from this char
            if let Ok(sym) = Symbols::try_from(char) {
                match sym {
                    // check for opening paren if we closed all previous expressions
                    Symbols::RPAREN => {
                        // special case where the expression ends, but we haven't stored the
                        // last parsed token yet. NOTE: we don't do this with LPAREN, cause the syntax
                        // token(expression), without a space between them, is considered incorrect.
                        if !currently_parsing_token.is_empty() {
                            self.get_last_mut()
                                .args
                                .push(Token::from(currently_parsing_token.clone()));
                            currently_parsing_token.clear();
                        }
                        // end token parsing
                        let current_expr = self.get_last_mut();
                        if current_expr.is_unclosed() {
                            current_expr.insert_closing(index);
                            if self.current_depth > 0 {
                                // go one expression up
                                self.current_depth -= 1;
                            }
                        } else {
                            if self.current_depth > 0 {
                                // go one expression up
                                self.current_depth -= 1;
                            }
                            let idx = self.calculate_index(0);
                            let current_expr = self.tree.peek_mut(idx);
                            current_expr.insert_closing(index);
                        }
                    }
                    //Symbols::LPAREN if self.get_last().is_unclosed() == true => {
                    Symbols::LPAREN => {
                        if self.get_last().get_opening().is_none() {
                            let current_expr = self.get_last_mut();
                            current_expr.insert_opening(index);
                            continue;
                        }
                        // create a new expression and leave the other in the stack
                        self.tree.push(TokenExpression::new());
                        // increment this depth's index, otherwise, push a new depth to the vector
                        match self.n_expressions_in_depth.get(self.current_depth) {
                            Some(len) => {
                                self.n_expressions_in_depth
                                    .insert(self.current_depth, len + 1);
                            }
                            None => {
                                self.n_expressions_in_depth.push(0);
                                //println!("Inserting index: 1 in depth: {}", self.current_depth);
                            }
                        }
                        let depth_index = self.n_expressions_in_depth[self.current_depth];
                        self.tree
                            .peek_mut(self.current_depth)
                            .args
                            .push(Token::Expression((
                                self.current_depth,
                                self.n_expressions_in_depth[self.current_depth],
                            )));
                        self.current_depth += 1;

                        // update the reference
                        let current_expr = self.get_last_mut();
                        current_expr.insert_opening(index);
                        current_expr.index = depth_index;
                        // HACK: workaround for mut & immutable references at the same time
                        #[allow(unused_variables)]
                        let current_expr = ();
                        let depth = self.current_depth;
                        let current_expr = self.get_last_mut();
                        current_expr.depth = depth;
                        // END HACK
                    }
                }
            } else {
                currently_parsing_token.push(char);
                // If we reached a space, push the parsed token to the current expression
                if char == ' ' && self.get_last().keyword.is_some() {
                    // remove the end space
                    currently_parsing_token.pop();
                    if !currently_parsing_token.is_empty() {
                        self.get_last_mut()
                            .args
                            .push(Token::from(currently_parsing_token.clone()));
                    }
                    currently_parsing_token.clear();
                // if we had no keyword, store it
                } else if char == ' ' && self.get_last().keyword.is_none() {
                    if !currently_parsing_token.is_empty() {
                        currently_parsing_token.pop();
                        self.get_last_mut().keyword =
                            Some(Token::from(currently_parsing_token.clone()));
                    }
                    currently_parsing_token.clear();
                }
            }
        }
        // finished iterating, check if it's closed
        //println!("{:?}", self.tree);
    }
}
