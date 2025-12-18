use crate::phi_base::*;
use bitvec::prelude::*;
use shua_struct::field::{BinaryField, Options};
use shua_struct_macro::binary_struct;
use std::cell::Cell;

#[derive(Debug, Default)]
#[binary_struct]
struct TestHeader {
    v: u8,
}

#[derive(Debug, Default)]
#[binary_struct]
struct TestStruct {
    h: TestHeader,
    a: u8,
    b: u16,
    c: u32,
    d: f32,
    e: bool,
    f: VarInt,
    g: PhiString,
}

#[test]
fn test_array() {
    let arr: [u8; 3] = [1, 2, 3];
    let bits = arr.build(&Some(Options::default())).unwrap();

    let parsed = <[u8; 3]>::parse(&bits, &Some(Options::default()))
        .unwrap()
        .0;

    assert_eq!(arr, parsed);
}

#[test]
fn test_struct_roundtrip() {
    let original = TestHeader { v: 99 };
    let bits = original.build(&None).unwrap();

    let parsed = TestHeader::parse(&bits, &None).unwrap().0;

    assert_eq!(original.v, parsed.v);
}

#[test]
fn test_nested_struct() {
    let original = TestStruct {
        h: TestHeader { v: 114 },
        a: 42,
        b: 0x1234,
        c: 0x87654321,
        d: 3.14,
        e: true,
        f: VarInt(200),
        g: "hi".into(),
    };

    let bits = original.build(&None).unwrap();
    let parsed = TestStruct::parse(&bits, &None).unwrap().0;

    assert_eq!(parsed.h.v, original.h.v);
    assert_eq!(parsed.a, original.a);
    assert_eq!(parsed.b, original.b);
    assert_eq!(parsed.c, original.c);
    assert!((parsed.d - original.d).abs() < f32::EPSILON);
    assert_eq!(parsed.e, original.e);
    assert_eq!(parsed.f.0, original.f.0);
    assert_eq!(parsed.g.0, original.g.0);
}
