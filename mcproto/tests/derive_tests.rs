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
use mcproto_derive::ServerboundPacket;

#[test]
fn create_struct() {
    #[derive(ServerboundPacket)]
    #[packet(id = 0x00)]
    struct Test {
        a: i32,
        b: String,
        c: i64
    }
}

