use crate::tag::Tag;
use crate::token::Token;
use crate::tokenizer::Tokenizer;
use std::iter::Peekable;
use std::mem;

pub struct Parser<'a> {
    tokenizer: Peekable<Box<dyn Iterator<Item = Token> + 'a>>,
}

impl<'a> From<Tokenizer<'a>> for Parser<'a> {
    fn from(tokenizer: Tokenizer<'a>) -> Self {
        Parser {
            tokenizer: (Box::new(tokenizer.into_iter()) as Box<dyn Iterator<Item = Token> + 'a>)
                .peekable(),
        }
    }
}

impl<'a> From<Vec<Token>> for Parser<'a> {
    fn from(tokens: Vec<Token>) -> Self {
        Parser {
            tokenizer: (Box::new(tokens.into_iter()) as Box<dyn Iterator<Item = Token> + 'a>)
                .peekable(),
        }
    }
}

impl Parser<'_> {
    pub fn parse(&mut self) -> Vec<Tag> {
        let mut tags = Vec::new();

        while let Some(token) = self.tokenizer.peek() {
            let tag = match token {
                Token::Header(_) => self.parse_header(),
                Token::Newline => self.parse_newline(),
                _ => Tag::Paragraph(self.parse_line()),
            };
            tags.push(tag)
        }
        tags
    }

    fn parse_newline(&mut self) -> Tag {
        self.tokenizer.next();
        Tag::Newline
    }

    fn parse_header(&mut self) -> Tag {
        let Token::Header(size) = self.tokenizer.next().unwrap() else {
            unreachable!()
        };
        Tag::Header(size, self.parse_line())
    }

    fn parse_line(&mut self) -> Vec<Tag> {
        let mut inner = Vec::new();
        let mut current = Vec::new();
        let mut style_stack = Vec::new();

        while let Some(token) = self.tokenizer.next() {
            match token {
                Token::Text(value) => {
                    if style_stack.is_empty() {
                        inner.push(Tag::Text(value))
                    } else {
                        current.push(Tag::Text(value));
                    }
                }
                style if [Token::Bold, Token::Italic, Token::Both].contains(&style) => {
                    if style_stack
                        .last()
                        .is_some_and(|last: &Token| last == &style)
                    {
                        style_stack.pop();
                        if style_stack.is_empty() {
                            inner.push(Tag::style(mem::take(&mut current), style));
                        } else {
                            current = vec![Tag::style(current, style)];
                        }
                    } else {
                        style_stack.push(style)
                    }
                }
                Token::Newline => break,
                _ => todo!(),
            }
        }

        inner
    }
}
