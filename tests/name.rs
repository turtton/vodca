use vodca::Nameln;

#[derive(Nameln)]
pub enum Enum<T> {
    I64 { id: i64 },
    Generics(T),
    Deleted,
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
}
