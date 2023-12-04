pub mod converter;
pub mod parser;
pub mod tag;
pub mod token;
pub mod tokenizer;

pub use parser::Parser;
pub use tag::Tag;
pub use token::Token;
pub use tokenizer::Tokenizer;

pub fn tokenize(content: &str) -> Vec<Token> {
    Tokenizer::from(content).into_iter().collect()
}

pub fn parse(content: &str) -> Vec<Tag> {
    Parser::from(Tokenizer::from(content)).into_iter().collect()
}

pub fn convert<'a>(content: &'a str) -> String {
    converter::convert(Parser::from(Tokenizer::from(content)).into_iter())
}
