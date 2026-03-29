use crate::position::Position;

#[derive(Debug, Clone)]
pub struct Lexer {
    pub file_name: String,
    pub text: String,
    pub position: Position,
    pub current_character: Option<char>,
}

impl Lexer {
    pub fn new(file_name: String, text: String) -> Self {
        let mut lexer = Self {
            position: Position::new(0, 0, 0, &file_name, &text),
            current_character: None,
            file_name,
            text,
        };

        lexer.advance(); // move to first char
        lexer
    }

    pub fn advance(&mut self) {
        self.position.advance(self.current_character);

        if self.position.index < self.text.len() {
            self.current_character = self.text.chars().nth(self.position.index);
        } else {
            self.current_character = None;
        }
    }
}

pub fn dummy() {}
