//! This module holds all the language logic.

use crate::tree::{Token, TokenExpression};

/// Hold the language built in functions.
pub enum BuiltinFuncs {
    Print,
    Add,
    Substract,
    Multiply,
    Divide,
}

impl BuiltinFuncs {
    // reduce the token expression to a Token result.
    pub fn exec(exp: &TokenExpression) -> Result<Token, &'static str> {
        let keyword = match &exp.keyword {
            Some(kw) => kw,
            None => return Err("Could not get keyword"),
        };
        match BuiltinFuncs::try_from(keyword) {
            Ok(BuiltinFuncs::Print) => Ok(Token::Literal(String::from("TODO"))),
            Ok(BuiltinFuncs::Add) => {
                let mut result: f64 = 0.0;
                for arg in &exp.args {
                    if let Token::Number(n) = arg {
                        result += n;
                    // TODO: we should get the expression result, but currently it's becoming increasingly
                    // difficult due to ownership. I'll have to rethink this.
                    } else if let Token::Expression(exp) = arg {
                        
                        return Err("Some expression was not correctly reduced while performing an operation,
                        or an illegal argument has been passed.");
                    }
                }
                return Ok(Token::Number(result));
            }
            _ => Err("Unknown builtin function"),
        }
    }
}

impl TryFrom<&Token> for BuiltinFuncs {
    type Error = &'static str;
    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        // try to match the passed token to a function
        match value {
            Token::Variable(name) => {
                // match strings to known builtin functions
                match name.to_lowercase().trim() {
                    "print" => Ok(BuiltinFuncs::Print),
                    "+" => Ok(BuiltinFuncs::Add),
                    "-" => Ok(BuiltinFuncs::Substract),
                    "*" => Ok(BuiltinFuncs::Multiply),
                    "/" => Ok(BuiltinFuncs::Divide),
                    _ => Err("Unknown function passed."),
                }
            }
            // TODO: others, user defined functions and such
            _ => Err("Unknown function passed."),
        }
    }
}
