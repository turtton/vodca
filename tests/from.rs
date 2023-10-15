#![allow(dead_code)]

use vodca::Fromln;

#[derive(Fromln)]
pub struct TupleStruct(String);

fn main() {
    let ts = TupleStruct("Hello, world!".to_string());
    let parsed: String = ts.into();
    assert_eq!(parsed, "Hello, world!");
}