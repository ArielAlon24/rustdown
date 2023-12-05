#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Text(String),
    Bold,
    Italic,
    Both,
    Header(usize),
    Newline,
    Dash,
    Tab,
}

impl Token {
    pub const STYLE_CHARS: [char; 2] = ['_', '*'];
    pub const STYLES: [Self; 3] = [Self::Bold, Self::Italic, Self::Both];

    pub fn text(content: &str) -> Self {
        Self::Text(content.into())
    }
}
