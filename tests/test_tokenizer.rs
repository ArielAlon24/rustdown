use rustdown::Token;
use rustdown::Tokenizer;

#[test]
fn test_text() {
    let tokenizer = Tokenizer::from("Hello, world!");
    let tokens: Vec<_> = tokenizer.into_iter().collect();
    assert_eq!(tokens, vec![Token::Text(String::from("Hello, world!"))]);
}

#[test]
fn test_italic_bold_both() {
    let underline: Vec<_> = Tokenizer::from("_italic_ or __bold__ or ___both___")
        .into_iter()
        .collect();
    let asteriks: Vec<_> = Tokenizer::from("*italic* or **bold** or ***both***")
        .into_iter()
        .collect();
    let expected = vec![
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
    ];

    assert_eq!(underline, expected);
    assert_eq!(asteriks, expected);
}

#[test]
fn test_newlines() {
    let actual: Vec<_> = Tokenizer::from("Line one \n__Bold Line__\n\n\n")
        .into_iter()
        .collect();
    let expected = vec![
        Token::text("Line one "),
        Token::Newline,
        Token::Bold,
        Token::text("Bold Line"),
        Token::Bold,
        Token::Newline,
        Token::Newline,
        Token::Newline,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn test_headers() {
    let actual: Vec<_> = Tokenizer::from("# H1 ## not a H2\n### H3")
        .into_iter()
        .collect();
    let expected = vec![
        Token::Header(1),
        Token::text("H1 ## not a H2"),
        Token::Newline,
        Token::Header(3),
        Token::text("H3"),
    ];
    assert_eq!(actual, expected);
}
