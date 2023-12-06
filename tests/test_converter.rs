use rustdown::converter;
use rustdown::Tag;

#[test]
fn test_paragraph() {
    let actual =
        converter::from(vec![Tag::Paragraph(vec![Tag::text("Hello, world!")])].into_iter());
    let expected = "<p>Hello, world!</p>".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn test_italic_bold_both() {
    let actual = converter::from(
        vec![Tag::Paragraph(vec![
            Tag::Italic(vec![Tag::text("italic")]),
            Tag::text(" or "),
            Tag::Bold(vec![Tag::text("bold")]),
            Tag::text(" or "),
            Tag::Bold(vec![Tag::Italic(vec![Tag::text("both")])]),
        ])]
        .into_iter(),
    );
    let expected = "<p><i>italic</i> or <b>bold</b> or <b><i>both</i></b></p>".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn test_newlines() {
    let actual = converter::from(
        vec![
            Tag::Paragraph(vec![Tag::text("Line one ")]),
            Tag::Newline,
            Tag::Paragraph(vec![Tag::Bold(vec![Tag::text("Bold Line")])]),
            Tag::Newline,
            Tag::Newline,
        ]
        .into_iter(),
    );
    let expected = "<p>Line one </p><br><p><b>Bold Line</b></p><br><br>".to_string();
    assert_eq!(actual, expected);
}

#[test]
fn test_header() {
    let actual = converter::from(
        vec![
            Tag::Header(1, vec![Tag::text("This is an header!")]),
            Tag::Paragraph(vec![
                Tag::text("And this is a paragraph with a "),
                Tag::Bold(vec![Tag::text("bold part")]),
                Tag::text("."),
            ]),
        ]
        .into_iter(),
    );
    let expected =
        "<h1>This is an header!</h1><p>And this is a paragraph with a <b>bold part</b>.</p>"
            .to_string();
    assert_eq!(actual, expected);
}

#[test]
fn test_unordered_list() {
    let actual = converter::from(
        vec![
            Tag::Header(1, vec![Tag::text("Unordered List")]),
            Tag::UnorderedList(vec![
                Tag::ListItem(vec![Tag::text("Entry 1.")]),
                Tag::UnorderedList(vec![Tag::ListItem(vec![Tag::text("Sub Entry 1.")])]),
            ]),
        ]
        .into_iter(),
    );
    let expected =
        "<h1>Unordered List</h1><ul><li>Entry 1.</li><ul><li>Sub Entry 1.</li></ul></ul>"
            .to_string();
    assert_eq!(actual, expected);
}
