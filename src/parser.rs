use crate::error::InvalidSyntaxError;
use crate::nodes::*;
use crate::parse_result::ParseResult;
use crate::tokens::{Token, TokenType::*};

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub token_index: isize,
    pub current_tok: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Self {
            tokens,
            token_index: -1,
            current_tok: None,
        };
        parser.advance();
        parser
    }

    pub fn advance(&mut self) -> Option<Token> {
        todo!()
    }

    pub fn reverse(&mut self, amount: usize) -> Option<Token> {
        todo!()
    }

    fn update_current_tok(&mut self) {
        todo!()
    }

    pub fn parse(&mut self) -> ParseResult {
        todo!()
    }

    // ================================
    // Parsing entry points
    // ================================
    fn statements(&mut self) -> ParseResult {
        todo!()
    }

    fn block(&mut self) -> ParseResult {
        todo!()
    }

    fn statement(&mut self) -> ParseResult {
        todo!()
    }

    fn expr(&mut self) -> ParseResult {
        todo!()
    }

    fn comp_expr(&mut self) -> ParseResult {
        todo!()
    }

    fn arith_expr(&mut self) -> ParseResult {
        todo!()
    }

    fn term(&mut self) -> ParseResult {
        todo!()
    }

    fn factor(&mut self) -> ParseResult {
        todo!()
    }

    fn power(&mut self) -> ParseResult {
        todo!()
    }

    fn call(&mut self) -> ParseResult {
        todo!()
    }

    fn atom(&mut self) -> ParseResult {
        todo!()
    }

    fn list_expr(&mut self) -> ParseResult {
        todo!()
    }

    fn if_expr(&mut self) -> ParseResult {
        todo!()
    }

    fn ternary_expr(&mut self) -> ParseResult {
        todo!()
    }

    fn for_expr(&mut self) -> ParseResult {
        todo!()
    }

    fn while_expr(&mut self) -> ParseResult {
        todo!()
    }

    fn func_def(&mut self) -> ParseResult {
        todo!()
    }

    fn bin_op<F>(&mut self, func_a: F, ops: Vec<TokenType>, func_b: Option<F>) -> ParseResult
    where
        F: Fn(&mut Self) -> ParseResult,
    {
        todo!()
    }
}

// ================================
// Helper enums / structs
// ================================

#[derive(Debug)]
pub enum BinOpFunc {
    // Placeholder for functions passed into bin_op
    Func,
}

// Optionally, you can define more enums if needed for internal parser state
