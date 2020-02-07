use structs_ser_deser_derive::{StructsDeser, StructsSer};
use structs_ser_deser_traits::{Deser, Ser};

#[derive(StructsSer, StructsDeser, Debug, PartialEq)]
struct MyStruct {
    a: u32,
    b: f64,
    c: String,
    d: bool,
    e: MyUnnamedStruct,
}

#[derive(StructsSer, StructsDeser, Debug, PartialEq)]
struct MyUnnamedStruct(u32, String);

#[test]
fn ser_deser_works_for_struct_with_inner_custom_struct() {
    let s = MyStruct {
        a: 10,
        b: 2.4,
        c: String::from("Hello, World!"),
        d: false,
        e: MyUnnamedStruct(12345, String::from("e!")),
    };
    let expected_serialized = [
        0xa, 0x0, 0x0, 0x0, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x3, 0x40, 0xd, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64,
        0x21, 0x0, 0x39, 0x30, 0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x65, 0x21,
    ];

    let mut serialized = bytes::BytesMut::with_capacity(s.ser_len());
    serialized.resize(s.ser_len(), 0x00);

    s.ser(serialized.as_mut()).expect("Failed to serialize");
    assert_eq!(expected_serialized.as_ref(), serialized.as_ref());

    let deserialized = MyStruct::deser(serialized.as_ref()).expect("Failed to deserialize");

    assert_eq!(s, deserialized);
}
