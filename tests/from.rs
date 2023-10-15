#![allow(dead_code)]

use std::marker::PhantomData;
use vodca::Fromln;

#[derive(Fromln)]
pub struct TupleStruct(String);

#[derive(Fromln)]
pub struct GenericTuple<T>(i32, PhantomData<T>);

fn main() {
    let ts = TupleStruct("Hello, world!".to_string());
    let parsed: String = ts.into();
    assert_eq!(parsed, "Hello, world!");

    let ts = GenericTuple::<()>(42, PhantomData);
    let parsed: i32 = ts.into();
    assert_eq!(parsed, 42);
}