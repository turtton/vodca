use vodca::Nameln;

#[derive(Nameln)]
pub enum Enum<T> {
    I64 { id: i64 },
    Option { option: Option<i64> },
    Generics(T),
    Deleted,
    TwoWords,
}

#[derive(Nameln)]
#[vodca(prefix = "prefix", snake_case)]
pub enum Prefix {
    I64 { id: i64 },
    Option { option: Option<i64> },
    Generics(String),
    Deleted,
    TwoWords,
}

fn main() {
    let i64: Enum<i64> = Enum::I64 { id: 42 };
    let name = i64.name();
    assert_eq!(name, "I64");

    let generics = Enum::Generics("Hello, world!".to_string());
    let name = generics.name();
    assert_eq!(name, "Generics");

    let deleted: Enum<&str> = Enum::Deleted;
    let name = deleted.name();
    assert_eq!(name, "Deleted");

    let two_words: Enum<&str> = Enum::TwoWords;
    let name = two_words.name();
    assert_eq!(name, "TwoWords");

    let i64: Prefix = Prefix::I64 { id: 42 };
    let name = i64.name();
    assert_eq!(name, "prefix_i64");

    let generics = Prefix::Generics("Hello, world!".to_string());
    let name = generics.name();
    assert_eq!(name, "prefix_generics");

    let deleted: Prefix = Prefix::Deleted;
    let name = deleted.name();
    assert_eq!(name, "prefix_deleted");
}
