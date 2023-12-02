#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Text(String),
    Bold,
    Italic,
    Both,
    Header(usize),
    Newline,
}

impl Token {
    pub const STYLE_CHARS: [char; 2] = ['_', '*'];

    pub fn text(content: &str) -> Self {
        Self::Text(content.into())
    }
}
