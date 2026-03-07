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
    #[error("VarInt too large: {0}, max is 2^31-1")]
    TooLarge(i32), // VarInt 太大了，超过2^31-1
    #[error("io error: {0}")]
    Io(#[from] std::io::Error), // IO错误强转用的
}

fn encode(mut value: i32, buf: &mut impl Write) -> Result<(), VarIntError> {
    for _ in 0..5 { // mc应该未来也不会有varlong吧，默认varint 5个字节最大
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buf.write_all(&[byte])?;
        if value == 0 { // 正常结束了
            return Ok(());
        }
    }
    Err(VarIntError::TooLarge(value))
}

pub fn decode(reader: &mut impl Read) -> Result<i32, VarIntError> {
    let mut result = 0u32;
    let mut shift = 0;
    let mut bytes_read = 0;

    loop {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        let byte = buf[0];
        bytes_read += 1;
        let value = (byte & 0x7F) as u32;
        result |= value << shift;

        // 如果最高位是0，说明这是最后一个字节
        if (byte & 0x80) == 0 {
            if result > i32::MAX as u32 {
                return Err(VarIntError::TooLarge(result as i32));
            }
            return Ok(result as i32);
        }

        shift += 7;

        if bytes_read == 5 {
            return Err(VarIntError::TooLarge(result as i32));
        }
    }
}

// 测试
#[cfg(test)]
mod tests {
    use rand::RngExt;
    use super::*;

    #[test]
    fn test_encode_speed() {
        println!("Encode varint test started");
        let mut rng = rand::rng();
        let mut buf = Vec::new();
        let iterations = 1_000_000; // 一百万次

        let start = std::time::Instant::now();

        for _ in 0..iterations {
            let n: i32 = rng.random();
            encode(n, &mut buf).unwrap(); // 结果直接unwrap，出错说明逻辑问题
            buf.clear();
        }

        let elapsed = start.elapsed();
        let ns = elapsed.as_nanos();
        let secs = ns as f64 / 1_000_000_000.0; // 转换成秒，其实有点诗山味道
        let speed = iterations as f64 / secs;

        println!("Time: {:.3}s", secs);
        println!("Encode speed: {:.0}/s", speed);
    }
}