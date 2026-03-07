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
pub fn encode(mut value: i64, buf: &mut impl Write) -> Result<(), CodecError> {
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
#[cfg(test)]
mod tests {
    use rand::RngExt;
    use super::*;

    #[test]
    fn test_encode_varlong() {
        println!("Encode varlong test started");
        let mut rng = rand::rng();
        let mut buf = Vec::new();
        let iterations = 100_000_000; // 一亿次

        let start = std::time::Instant::now();

        for _ in 0..iterations {
            let n: i64 = rng.random(); // 从varint改的 awa
            encode(n, &mut buf).expect("encode failed");
            buf.clear();
        }

        let elapsed = start.elapsed();
        let ns = elapsed.as_nanos();
        let secs = ns as f64 / 1_000_000_000.0;
        let speed = iterations as f64 / secs;

        println!("Time: {:.3}s", secs);
        println!("Encode speed: {:.0}/s", speed);
    }
    #[test]
    fn test_both_varlong() {
        println!("Encode & Decode(Both) varlong test started");
        let mut rng = rand::rng();
        let mut buf = Vec::new();
        let iterations = 100_000_000; // 一亿次

        let start = std::time::Instant::now();

        for _ in 0..iterations {
            let n: i64 = rng.random();
            encode(n, &mut buf).expect("encode failed");
            decode(&mut buf.as_slice()).expect("decode failed");
            buf.clear();
        }

        let elapsed = start.elapsed();
        let ns = elapsed.as_nanos();
        let secs = ns as f64 / 1_000_000_000.0;
        let speed = iterations as f64 / secs;

        println!("Time: {:.3}s", secs);
        println!("Encode & Decode speed: {:.0}/s", speed);
    }
}