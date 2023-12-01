use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum Tag {
    Newline,
    Text(String),
    Bold(Vec<Tag>),
    Italic(Vec<Tag>),
    Paragraph(Vec<Tag>),
    Header(usize, Vec<Tag>),
    // OrederedList(Vec<Tag>),
    // UnorederedList(Vec<Tag>),
    // Link(String),
    // InlineCode(String),
    // Code(String),
    // Blockquote(String),
}

impl Tag {
    pub fn style(inner: Vec<Tag>, style_token: Token) -> Self {
        match style_token {
            Token::Bold => Tag::Bold(inner),
            Token::Italic => Tag::Italic(inner),
            Token::Both => Tag::Bold(vec![Tag::Italic(inner)]),
            _ => panic!("Unknown Style Token!"),
        }
    }
}
