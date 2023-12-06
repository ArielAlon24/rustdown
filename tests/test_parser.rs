use rustdown::Parser;
use rustdown::Tag;
use rustdown::Token;

#[test]
fn test_text() {
    let actual: Vec<_> = Parser::from(vec![Token::text("Hello, world!")])
        .into_iter()
        .collect();
    let expected = vec![Tag::Paragraph(vec![Tag::text("Hello, world!")])];

    assert_eq!(actual, expected);
}

#[test]
fn test_italic_bold_both() {
    let actual: Vec<_> = Parser::from(vec![
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
    .into_iter()
    .collect();

    let expected = vec![Tag::Paragraph(vec![
        Tag::Italic(vec![Tag::text("italic")]),
        Tag::text(" or "),
        Tag::Bold(vec![Tag::text("bold")]),
        Tag::text(" or "),
        Tag::Bold(vec![Tag::Italic(vec![Tag::text("both")])]),
    ])];

    assert_eq!(actual, expected);
}

#[test]
fn test_newlines() {
    let actual: Vec<_> = Parser::from(vec![
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
    .into_iter()
    .collect();

    let expected = vec![
        Tag::Paragraph(vec![Tag::text("Line one ")]),
        Tag::Newline,
        Tag::Paragraph(vec![Tag::Bold(vec![Tag::text("Bold Line")])]),
        Tag::Newline,
        Tag::Newline,
    ];

    assert_eq!(actual, expected);
}

#[test]
fn test_header() {
    let actual: Vec<_> = Parser::from(vec![
        Token::Header(1),
        Token::text("This is an header!"),
        Token::Newline,
        Token::text("And this is a paragraph with a "),
        Token::Bold,
        Token::text("bold part"),
        Token::Bold,
        Token::text("."),
    ])
    .into_iter()
    .collect();

    let expected = vec![
        Tag::Header(1, vec![Tag::text("This is an header!")]),
        Tag::Paragraph(vec![
            Tag::text("And this is a paragraph with a "),
            Tag::Bold(vec![Tag::text("bold part")]),
            Tag::text("."),
        ]),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn test_unordered_list() {
    let actual: Vec<_> = Parser::from(vec![
        Token::Header(1),
        Token::text("Unordered List"),
        Token::Newline,
        Token::Dash,
        Token::text("Entry 1."),
        Token::Newline,
        Token::Dash,
        Token::Dash,
        Token::text("Sub Entry 1."),
    ])
    .into_iter()
    .collect();

    let expected = vec![
        Tag::Header(1, vec![Tag::text("Unordered List")]),
        Tag::UnorderedList(vec![
            Tag::ListItem(vec![Tag::text("Entry 1.")]),
            Tag::UnorderedList(vec![Tag::ListItem(vec![Tag::text("Sub Entry 1.")])]),
        ]),
    ];

    assert_eq!(actual, expected);
}
