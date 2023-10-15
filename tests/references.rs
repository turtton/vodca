#![allow(dead_code)]

use vodca::References;

#[derive(References)]
pub struct Struct {
    field_i32: i32,
    field_string: String,
    field_vec: Vec<i32>
}

fn main() {
    let s = Struct {
        field_i32: 42,
        field_string: "Hello, world!".to_string(),
        field_vec: vec![1, 2, 3]
    };

    let i32 = s.field_i32();
    assert_eq!(i32, &42);

    let str: &str = s.field_string();
    assert_eq!(str, "Hello, world!");

    let vec: &[i32] = s.field_vec();
    assert_eq!(vec, &[1, 2, 3]);
}