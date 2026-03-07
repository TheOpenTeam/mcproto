/*
 *
 *  * Created: 2026-3-7 0:30:25
 *  * File: varint.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use std::io::{Read, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VarIntError {
    #[error("VarInt too large, max is 2^31-1")]
    TooLarge, // VarInt 太大了，超过2^31-1
    #[error("io error: {0}")]
    Io(#[from] std::io::Error), // IO错误强转用的
}
#[inline]
pub fn encode(mut value: i32, buf: &mut impl Write) -> Result<(), VarIntError> {
    let mut value = value as u32; // 强转
    for i in 0..5 {
        let byte = (value & 0x7F) as u8;
        value >>= 7;
        let has_next = value != 0 && i < 4;
        let byte = if has_next { byte | 0x80 } else { byte };

        buf.write_all(&[byte])?;

        if !has_next {
            return Ok(());
        }
    }
    Err(VarIntError::TooLarge)
}
#[inline]
pub fn decode(reader: &mut impl Read) -> Result<i32, VarIntError> {
    let mut result = 0u32;
    let mut shift = 0;

    for _ in 0..5 {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        let byte = buf[0];

        let value = (byte & 0x7F) as u32;
        result |= value << shift;

        if (byte & 0x80) == 0 {
            return Ok(result as i32);
        }

        shift += 7;
    }
    Err(VarIntError::TooLarge)
}
// 测试 cargo test test_encode --release -- --nocapture
#[cfg(test)]
mod tests {
    use rand::RngExt;
    use super::*;

    #[test]
    fn test_encode() {
        println!("Encode varint test started");
        let mut rng = rand::rng();
        let mut buf = Vec::new();
        let iterations = 100_000_000; // 一亿次

        let start = std::time::Instant::now();

        for _ in 0..iterations {
            let n: i32 = rng.random();
            encode(n, &mut buf).expect("encode failed"); // 6666 Rust发力了
            buf.clear();
        }

        let elapsed = start.elapsed();
        let ns = elapsed.as_nanos();
        let secs = ns as f64 / 1_000_000_000.0; // 转换成秒，其实有点诗山味道
        let speed = iterations as f64 / secs;

        println!("Time: {:.3}s", secs);
        println!("Encode speed: {:.0}/s", speed);
    }
    #[test]
    fn test_both() {
        println!("Encode & Decode(Both) varint test started");
        let mut rng = rand::rng();
        let mut buf = Vec::new();
        let iterations = 100_000_000; // 一亿次

        let start = std::time::Instant::now();

        for _ in 0..iterations {
            let n: i32 = rng.random();
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