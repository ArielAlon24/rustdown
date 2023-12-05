use crate::tag::Tag;
use crate::token::Token;
use crate::tokenizer::Tokenizer;
use std::iter::Peekable;
use std::{mem, unreachable};

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

impl<'a> Iterator for Parser<'a> {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.tokenizer.peek() {
            return match token {
                Token::Header(_) => Some(self.parse_header()),
                Token::Newline => Some(self.parse_newline()),
                Token::Dash => Some(self.parse_ul()),
                _ => Some(Tag::Paragraph(self.parse_line())),
            };
        }
        None
    }
}

impl Parser<'_> {
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
                style if Token::STYLES.contains(&style) => {
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

    fn parse_li(&mut self, initiataor: Token) -> (Tag, usize) {
        let mut level = 0;
        while self
            .tokenizer
            .peek()
            .is_some_and(|token| token == &initiataor)
        {
            self.tokenizer.next();
            level += 1;
        }
        (Tag::ListItem(self.parse_line()), level)
    }

    fn parse_ul(&mut self) -> Tag {
        let mut ul = Tag::UnorderedList(vec![]);
        let mut current = 1;
        let mut last_ul: &mut Tag = &mut ul;

        while let Some(Token::Dash) = self.tokenizer.peek() {
            let (li, level) = self.parse_li(Token::Dash);

            if level > current {
                for _ in 0..level - current {
                    let Tag::UnorderedList(tags) = last_ul else {
                        unreachable!()
                    };
                    tags.push(Tag::UnorderedList(vec![]));
                    last_ul = tags.last_mut().unwrap();
                }
            } else if current > level {
                last_ul = &mut ul;
                for _ in 0..level - 1 {
                    let Tag::UnorderedList(tags) = last_ul else {
                        unreachable!()
                    };
                    last_ul = tags.last_mut().unwrap();
                }
            }

            current = level;
            let Tag::UnorderedList(tags) = last_ul else {
                unreachable!()
            };
            tags.push(li);
        }

        ul
    }
}
