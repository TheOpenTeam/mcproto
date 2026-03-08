/*
 *
 *  * Created: 2026-3-7 2:19:33
 *  * File: varlong.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use std::io::{Read, Write};
use crate::CodecError;

#[inline]
pub fn encode(value: i64, buf: &mut impl Write) -> Result<(), CodecError> {
    let mut value = value as u64;
    for _ in 0..10 {
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buf.write_all(&[byte])?;
        if value == 0 {
            return Ok(());
        }
    }
    Err(CodecError::VarTooLarge)
}

#[inline]
pub fn decode(reader: &mut impl Read) -> Result<i64, CodecError> {
    let mut result = 0u64;
    let mut shift = 0;

    for _ in 0..10 {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        let byte = buf[0];

        let value = (byte & 0x7F) as u64;
        result |= value << shift;

        if (byte & 0x80) == 0 {
            return Ok(result as i64);
        }

        shift += 7;
    }
    Err(CodecError::VarTooLarge)
}

