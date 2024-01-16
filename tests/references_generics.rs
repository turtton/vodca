#![allow(dead_code)]

use vodca::References;

#[derive(References)]
pub struct StructGeneric<T> {
    field_vec: Vec<T>,
}

fn main() {
    let s = StructGeneric {
        field_vec: vec![1, 2, 3],
    };

    let vec: &[i32] = s.field_vec();
    assert_eq!(vec, &[1, 2, 3]);
}
