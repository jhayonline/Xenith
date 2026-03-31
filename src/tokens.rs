use crate::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Int,
    Float,
    String,
    Identifier,
    Keyword,

    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    Eq,

    LParen,
    RParen,
    LSquare,
    RSquare,
    LBrace,
    RBrace,

    Question,
    Colon,
    Ee, // ==
    Ne, // !=
    Lt,
    Gt,
    Lte,
    Gte,
    Comma,
    Arrow, // ->
    Newline,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub value: Option<String>,
    pub position_start: Position,
    pub position_end: Position,
}

impl Token {
    pub fn new(
        kind: TokenType,
        value: Option<String>,
        position_start: Position,
        position_end: Option<Position>,
    ) -> Self {
        let pos_end = if let Some(end) = position_end {
            end
        } else {
            let mut copy = position_start.clone();
            copy.advance(None);
            copy
        };

        Self {
            kind,
            value,
            position_start,
            position_end: pos_end,
        }
    }
    pub fn matches(&self, kind: TokenType, value: Option<&str>) -> bool {
        self.kind == kind && self.value.as_deref() == value
    }
}

pub const KEYWORDS: &[&str] = &[
    "spawn",
    "&&",
    "||",
    "!",
    "when",
    "or when",
    "otherwise",
    "for",
    "to",
    "step",
    "while",
    "method",
    "release",
    "skip",
    "stop",
];
