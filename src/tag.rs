use core::fmt;

use crate::Token;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn text(content: &str) -> Self {
        Self::Text(content.into())
    }
}

fn format_tags(tags: &Vec<Tag>) -> String {
    tags.into_iter()
        .map(|tag| format!("{}", tag))
        .collect::<Vec<_>>()
        .join("")
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tag::Newline => write!(f, "<br>"),
            Tag::Text(text) => write!(f, "{text}"),
            Tag::Bold(tags) => write!(f, "<b>{}</b>", format_tags(tags)),
            Tag::Italic(tags) => write!(f, "<i>{}</i>", format_tags(tags)),
            Tag::Paragraph(tags) => write!(f, "<p>{}</p>", format_tags(tags)),
            Tag::Header(size, tags) => write!(f, "<h{size}>{}</h{size}>", format_tags(tags)),
        }
    }
}
