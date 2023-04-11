//! This module holds the syntax tree

use std::borrow::Borrow;

use crate::logic::BuiltinFuncs;

#[derive(Debug)]
pub struct Tree {
    /// Holds an array of expressions
    pub(crate) expressions: Vec<TokenExpression>,
}

impl Tree {
    /// Create a new syntax tree.
    pub fn new() -> Self {
        Tree {
            expressions: Vec::new(),
        }
    }
    /// push a new expression to the tree vec
    pub fn push(&mut self, expr: TokenExpression) {
        self.expressions.push(expr);
    }
    /// get expression from index (mutable)
    pub fn peek_mut(&mut self, index: usize) -> &mut TokenExpression {
        self.expressions.get_mut(index).unwrap()
    }
    /// get expression from index
    pub fn peek(&self, index: usize) -> &TokenExpression {
        match self.expressions.get(index) {
            Some(expression) => expression,
            None => self.expressions.get(index - 1).unwrap(),
        }
    }
}

// Holds a logical token, like a function keyword, a variable, a literal or another expression.
// We make sure the token, for example if it is an argument (fun (fun2 args))
//                                                                ---------
//                                                                Token  ⤶
// Will have the same lifetime as the rest of the expression.
#[derive(Debug)]
pub enum Token {
    // Function keywords.
    // Keyword(String),
    // A string literal (unquoted)
    Literal(String),
    // a number literal
    Number(f64),
    // TODO
    Variable(String), //(PhantomData<&'a ()>),
    // Might hold a reference to another expression to eval. (depth, index)
    Expression((usize, usize)),
    Result,
}
/// try to find a keyword for this string else return a variable if it is unquoted
/// , a literal if it has "" quotes or a number if it is a number.
impl From<String> for Token {
    fn from(parsed: String) -> Token {
        // quoted literal
        if parsed.starts_with('"') && parsed.ends_with('"') {
            // get rid of the quotes and return a string literal
            Token::Literal(parsed.replace('"', "").to_string())
        // number literal as f64 representation
        } else if let Ok(digit) = parsed.parse() {
            Token::Number(digit)
        } else {
            Token::Variable(parsed.to_string())
        }
    }
}

/// Special symbols for the syntax
pub enum Symbols {
    LPAREN,
    RPAREN,
}

impl TryFrom<char> for Symbols {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::LPAREN),
            ')' => Ok(Self::RPAREN),
            _ => Err("Could not convert char to a known symbol"),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
/// A group of tokens and arguments.
pub struct TokenExpression {
    // Keyword for this expression
    pub keyword: Option<Token>,
    // Nesting level
    pub depth: usize,
    // Function arguments.
    pub args: Vec<Token>,
    // holds the location of both of this expression's delimiters
    pub delimiters: (Option<usize>, Option<usize>),
    pub index: usize,
}

impl TokenExpression {
    /// Create a new expression
    pub fn new() -> Self {
        TokenExpression {
            keyword: None,
            depth: 0,
            args: Vec::new(),
            delimiters: (None, None),
            index: 0,
        }
    }
    /// Reduce this expression to a Token result
    pub fn reduce(&mut self) -> Token {
        let result = BuiltinFuncs::exec(self.borrow());
        match result {
            Ok(result) => result,
            Err(err) => {
                // TEMP: don't panic
                panic!("Reducing failed with error: {}", err)
            }
        }
    }
    /// Checks if the current expression has both its delimiters. Note, it does not mean it's empty,
    /// this function only returns true if the left delimiter "(" is present, but not the closing delimiter.
    pub fn is_unclosed(&self) -> bool {
        self.delimiters.0.is_some() && self.delimiters.1.is_none()
    }
    /// Insert the index of this expression's opening paren
    pub fn insert_opening(&mut self, index: usize) {
        self.delimiters.0 = Some(index);
    }
    /// Get the index of this expression's opening paren
    pub fn get_opening(&self) -> Option<usize> {
        self.delimiters.0
    }
    /// Insert the index of this expression's closing paren
    pub fn insert_closing(&mut self, index: usize) {
        self.delimiters.1 = Some(index);
    }
    /// Get the index of this expression's opening paren
    pub fn get_closing(&self) -> Option<usize> {
        self.delimiters.1
    }
}
