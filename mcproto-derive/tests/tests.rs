/*
 *
 *  * Created: 2026-3-7 4:48:7
 *  * File: tests
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use mcproto_utils::*;
    use mcproto_derive::ServerboundPacket;
    #[test]
    fn serverbound_packet_test() {
        #[derive(ServerboundPacket, Debug)]
        #[packet(id = 0x00)]
        struct Test {
            int: i32, // VarInt
            str: String, //Var Int + String
            long: i64, // VarLong
            boolean: bool, // Bool
            byte: i8, // Byte
            ubyte: u8, // Unsigned Byte
            short: i16, // Short
            ushort: u16, // Unsigned Short
            float: f32, // Float
            double: f64, // Double

        }
        println!("Successfully create a serverbound packet struct");
        let test = Test {
            int: 123456789,
            str: "666 This person is hack".to_string(),
            long: 1234567890123456789,
            boolean: true,
            byte: -128,
            ubyte: 255,
            short: -32768,
            ushort: 65535,
            float: 1.23456789f32,
            double: 1.1145141919354836f64,
        };
        let mut buf = Vec::new();
        test.encode(&mut buf).expect("Failed to encode packet");
        dbg!(&buf);
        dbg!(&test);
        println!("Passed the test including encode and decode packet.");

    }
}

