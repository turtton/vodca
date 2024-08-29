#![allow(dead_code)]

use std::marker::PhantomData;
use vodca::Newln;

#[derive(Newln)]
pub struct TupleStructI32(i32);

#[derive(Newln)]
pub struct TupleStructString(String);

#[derive(Newln)]
pub struct TupleStructVec(Vec<i32>);

#[derive(Newln)]
pub struct TupleStructGenerics<T>(i32, PhantomData<T>);

#[derive(Newln)]
pub struct Struct<T> {
    field_i32: i32,
    field_string: String,
    field_vec: Vec<i32>,
    data: PhantomData<T>,
}

fn main() {
    let ts_i32 = TupleStructI32::new(42);
    assert_eq!(ts_i32.0, 42);

    let ts_string = TupleStructString::new("Hello, world!".to_string());
    assert_eq!(ts_string.0, "Hello, world!".to_string());

    let ts_vec = TupleStructVec::new(vec![1, 2, 3]);
    assert_eq!(ts_vec.0, vec![1, 2, 3]);

    let ts_generics = TupleStructGenerics::<()>::new(42, PhantomData);
    assert_eq!(ts_generics.0, 42);

    let s: Struct<String> =
        Struct::new(42, "Hello, world!".to_string(), vec![1, 2, 3], PhantomData);

    let i32 = s.field_i32;
    assert_eq!(i32, 42);

    let str = s.field_string;
    assert_eq!(str, "Hello, world!");

    let vec = s.field_vec;
    assert_eq!(vec, &[1, 2, 3]);
}
