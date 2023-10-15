#![allow(dead_code)]

use vodca::AsRefln;

#[derive(AsRefln)]
pub struct TupleStructI32(i32);


#[derive(AsRefln)]
pub struct TupleStructString(String);

#[derive(AsRefln)]
pub struct TupleStructVec(Vec<i32>);

fn main() {
    let ts_i32 = TupleStructI32(42);
    let i32 = ts_i32.as_ref();
    assert_eq!(i32, &42);

    let ts_string = TupleStructString("Hello, world!".to_string());
    let str: &str = ts_string.as_ref();
    assert_eq!(str, "Hello, world!");

    let ts_vec = TupleStructVec(vec![1, 2, 3]);
    let vec: &[i32] = ts_vec.as_ref();
    assert_eq!(vec, &[1, 2, 3]);
}
