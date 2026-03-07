/*
 *
 *  * Created: 2026-3-7 4:48:7
 *  * File: derive_tests.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
#[cfg(test)]
mod tests;

use std::io::Cursor;
use mcproto::packet::ServerboundPacket;
use mcproto_derive::ServerboundPacket;

#[test]
fn serverbound_packet_test() {
    #[derive(ServerboundPacket, Debug)]
    #[packet(id = 0x00)]
    struct Test {
        a: i32,
        b: String,
        c: i64
    }
    println!("Successfully create a serverbound packet struct");
    let test = Test {
        a: 1,
        b: "test".to_string(),
        c: 2
    };
    let mut buf = Vec::new();
    test.encode(&mut buf).expect("Failed to encode packet");
    dbg!(&buf);
    Test::decode(&mut Cursor::new(buf)).expect("Failed to decode packet");
    dbg!(&test);
    println!("Passed the test including encode and decode packet.");

}

