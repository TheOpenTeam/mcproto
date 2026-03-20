use std::collections::HashMap;
use std::fs;

use mcproto_nbt::nbt::{Nbt, NbtValue};

#[test]
fn test_nbt_all_types_roundtrip() {
    let mut root = HashMap::new();

    // primitives
    root.insert("byte".into(), NbtValue::Byte(1));
    root.insert("short".into(), NbtValue::Short(2));
    root.insert("int".into(), NbtValue::Int(3));
    root.insert("long".into(), NbtValue::Long(4));
    root.insert("float".into(), NbtValue::Float(5.5));
    root.insert("double".into(), NbtValue::Double(6.6));

    // arrays
    root.insert("byte_array".into(), NbtValue::ByteArray(vec![1, 2, 3]));
    root.insert("int_array".into(), NbtValue::IntArray(vec![10, 20, 30]));
    root.insert("long_array".into(), NbtValue::LongArray(vec![100, 200, 300]));

    // string
    root.insert("string".into(), NbtValue::String("hello".into()));

    // list
    root.insert(
        "list".into(),
        NbtValue::List(vec![
            NbtValue::Int(1),
            NbtValue::Int(2),
            NbtValue::Int(3),
        ]),
    );

    // compound
    let mut inner = HashMap::new();
    inner.insert("inner_key".into(), NbtValue::String("value".into()));
    root.insert("compound".into(), NbtValue::Compound(inner));

    let nbt = Nbt { root };

    let bytes = nbt.to_bytes().expect("to_bytes failed");

    // 写入当前目录
    fs::write("test.nbt", &bytes).expect("write file failed");

    let decoded = Nbt::from_bytes(&bytes).expect("from_bytes failed");

    // assert
    assert_eq!(nbt.root, decoded.root);

    
}