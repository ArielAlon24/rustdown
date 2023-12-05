use crate::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl Tokenizer<'_> {
    pub fn consume_text(&mut self, start: char) -> String {
        let mut result = String::from(start);
        while let Some(c) = self.chars.peek() {
            match c {
                '\n' => break,
                c if Token::STYLE_CHARS.contains(&c) => break,
                _ => result.push(self.chars.next().unwrap()),
            }
        }
        result
    }

    fn consume_header(&mut self) -> Token {
        let mut size = 1;
        while let Some('#') = self.chars.peek() {
            size += 1;
            self.chars.next();
        }

        if self.chars.next().is_some_and(|peeked| peeked != ' ') {
            panic!("Header must contain a space after intialaization.")
        }

        Token::Header(size)
    }

    fn next_style(&mut self, c1: char) -> Token {
        if self.chars.peek().is_some_and(|c2| c1 == *c2) {
            self.chars.next();
            if self
                .chars
                .peek()
                .is_some_and(|c3| Token::STYLE_CHARS.contains(&c3))
            {
                self.chars.next();
                return Token::Both;
            }
            return Token::Bold;
        }
        Token::Italic
    }

    fn handle_dash(&mut self) -> Token {
        if let Some(' ') = self.chars.peek() {
            self.chars.next();
            return Token::Dash;
        }
        Token::Text(self.consume_text('-'))
    }
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(value: &'a str) -> Self {
        Tokenizer {
            chars: value.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next()? {
            '\n' => Some(Token::Newline),
            '\t' => Some(Token::Tab),
            '#' => Some(self.consume_header()),
            '-' => Some(self.handle_dash()),
            c if Token::STYLE_CHARS.contains(&c) => Some(self.next_style(c)),
            c => Some(Token::Text(self.consume_text(c))),
        }
    }
}
