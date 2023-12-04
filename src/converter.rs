use crate::Tag;

pub fn convert<'a>(iterator: impl Iterator<Item = Tag>) -> String {
    iterator
        .map(|tag| format!("{}", tag))
        .collect::<Vec<_>>()
        .join("")
}
