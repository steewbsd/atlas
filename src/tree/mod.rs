//! This module holds the syntax tree

use std::marker::PhantomData;

#[derive(Debug)]
pub struct Tree<'a> {
    /// Holds an array of expressions
    _marker: PhantomData<&'a ()>,
    expressions: Vec<TokenExpression<'a>>,
}

impl<'a> Tree<'a> {
    /// Create a new syntax tree.
    pub fn new() -> Self {
        Tree {
            _marker: PhantomData::default(),
            expressions: Vec::new(),
        }
    }
    /// push a new expression to the tree vec
    pub fn push(&mut self, expr: TokenExpression<'a>) {
        self.expressions.push(expr);
    }
    /// get expression from index (mutable)
    pub fn peek_mut(&mut self, index: usize) -> &mut TokenExpression<'a> {
        self.expressions.get_mut(index).unwrap()
    }
    /// get expression from index
    pub fn peek(&self, index: usize) -> &TokenExpression<'a> {
        self.expressions.get(index).unwrap()
    }
}
#[derive(Debug)]
#[allow(dead_code)]
pub struct ExpressionLocation {
    depth: usize,
    index: usize,
}

impl ExpressionLocation {
    pub fn new(depth: usize, index: usize) -> Self {
        ExpressionLocation { depth, index }
    }
}

// Holds a logical token, like a function keyword, a variable, a literal or another expression.
// We make sure the token, for example if it is an argument (fun (fun2 args))
//                                                                ---------
//                                                                Token  ⤶
// Will have the same lifetime as the rest of the expression.
#[derive(Debug)]
pub enum Token<'a> {
    // Function keywords.
    Keyword(String),
    // A string or number literal.
    Literal(String),
    // TODO
    Variable(PhantomData<&'a ()>),
    // Might hold a reference to another expression to eval. (depth, index)
    Expression(ExpressionLocation),
}

impl<'a> From<&str> for Token<'a> {
    fn from(parsed: &str) -> Token<'a> {
        match parsed {
            _ => Token::Keyword(String::from("TODO")),
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
pub struct TokenExpression<'a> {
    // Keyword for this expression
    pub keyword: Option<Token<'a>>,
    // Nesting level
    pub depth: usize,
    // Function arguments.
    pub args: Vec<Token<'a>>,
    // holds the location of both of this expression's delimiters
    pub delimiters: (Option<usize>, Option<usize>),
}

impl<'a> TokenExpression<'a> {
    /// Create a new expression
    pub fn new() -> Self {
        TokenExpression {
            keyword: None,
            depth: 0,
            args: Vec::new(),
            delimiters: (None, None),
        }
    }
    /// Checks if the current expression has both its delimiters. Note, it does not mean it's empty,
    /// this function only returns true if the left delimiter "(" is present, but not the closing delimiter.
    pub fn is_unclosed(&self) -> bool {
        self.delimiters.0 != None && self.delimiters.1 == None
    }
    /// Insert the index of this expression's opening paren
    pub fn insert_opening(&mut self, index: usize) {
        self.delimiters.0 = Some(index);
    }
    /// Get the index of this expression's opening paren
    pub fn get_opening(&mut self) -> Option<usize> {
        self.delimiters.0
    }
    /// Insert the index of this expression's closing paren
    pub fn insert_closing(&mut self, index: usize) {
        self.delimiters.1 = Some(index);
    }
    /// Get the index of this expression's opening paren
    pub fn get_closing(&mut self) -> Option<usize> {
        self.delimiters.1
    }
}
