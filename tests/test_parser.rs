use rustdown::parser::Parser;
use rustdown::tag::Tag;
use rustdown::tokenizer::Token;

#[test]
fn test_text() {
    let tags = Parser::from(vec![Token::text("Hello, world!")]).parse();
    let expected = vec![Tag::Paragraph(vec![Tag::text("Hello, world!")])];

    assert_eq!(tags, expected);
}

#[test]
fn test_italic_bold_both() {
    let tags = Parser::from(vec![
        Token::Italic,
        Token::text("italic"),
        Token::Italic,
        Token::text(" or "),
        Token::Bold,
        Token::text("bold"),
        Token::Bold,
        Token::text(" or "),
        Token::Both,
        Token::text("both"),
        Token::Both,
    ])
    .parse();

    let expected = vec![Tag::Paragraph(vec![
        Tag::Italic(vec![Tag::text("italic")]),
        Tag::text(" or "),
        Tag::Bold(vec![Tag::text("bold")]),
        Tag::text(" or "),
        Tag::Bold(vec![Tag::Italic(vec![Tag::text("both")])]),
    ])];

    assert_eq!(tags, expected);
}

#[test]
fn test_newlines() {
    let tags = Parser::from(vec![
        Token::text("Line one "),
        Token::Newline,
        Token::Newline,
        Token::Bold,
        Token::text("Bold Line"),
        Token::Bold,
        Token::Newline,
        Token::Newline,
        Token::Newline,
    ])
    .parse();

    let expected = vec![
        Tag::Paragraph(vec![Tag::text("Line one ")]),
        Tag::Newline,
        Tag::Paragraph(vec![Tag::Bold(vec![Tag::text("Bold Line")])]),
        Tag::Newline,
        Tag::Newline,
    ];

    assert_eq!(tags, expected);
}

#[test]
fn test_header() {
    let tags = Parser::from(vec![
        Token::Header(1),
        Token::text("This is an header!"),
        Token::Newline,
        Token::text("And this is a paragraph with a "),
        Token::Bold,
        Token::text("bold part"),
        Token::Bold,
        Token::text("."),
    ])
    .parse();

    let expected = vec![
        Tag::Header(1, vec![Tag::text("This is an header!")]),
        Tag::Paragraph(vec![
            Tag::text("And this is a paragraph with a "),
            Tag::Bold(vec![Tag::text("bold part")]),
            Tag::text("."),
        ]),
    ];

    assert_eq!(tags, expected);
}
