//! This module holds the syntax tree

pub struct Tree<'a> {
    /// Holds an array of expressions
    expressions: Vec<&'a TokenExpression<'a>>,
}

impl Tree<'_> {
    /// Create a new syntax tree.
    pub fn new() -> Self {
        Tree {
            expressions: Vec::new(),
        }
    }
}

// Holds a logical token, like a function keyword, a variable, a literal or another expression.
// We make sure the token, for example if it is an argument (fun (fun2 args))
//                                                                ---------
//                                                                Token  ⤶
// Will have the same lifetime as the rest of the expression.
pub enum Token<'a> {
    // Function keywords.
    Keyword(String),
    // A string or number literal.
    Literal(String),
    // TODO
    Variable,
    // Might hold a reference to another expression to eval.
    Expression(&'a TokenExpression<'a>),
}

impl<'a> From<&str> for Token<'a> {
    fn from(parsed: &str) -> Token<'a> {
        match parsed {
            _ => Token::Keyword(String::from("TODO")),
        }
    }
}

/// A group of tokens and arguments.
pub struct TokenExpression<'a> {
    // Keyword for this expression
    keyword: Token<'a>,
    // Nesting level
    level: usize,
    // Function arguments.
    args: Vec<Token<'a>>,
}

impl<'a> TokenExpression<'a> {
    /// Create a new expression from given keyword, indentation level, and arguments.
    pub fn new(keyword: Token<'a>, level: usize, args: Vec<Token<'a>>) -> Self {
        TokenExpression {
            keyword,
            level,
            args,
        }
    }
}

/// Parse a string
pub fn parse(contents: &str) -> Tree {
    let mut tree = Tree::new();

    tree
}
