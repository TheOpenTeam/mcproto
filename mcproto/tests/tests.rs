/*
 *
 *  * Created: 2026-3-7 4:38:59
 *  * File: tests.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */

#[cfg(test)]
mod tests {
    use rand::RngExt;
    use mcproto::utils::{varint, varlong};

    #[test]
    fn test_encode() {
        println!("Encode varint test started");
        let mut rng = rand::rng();
        let mut buf = Vec::new();
        let iterations = 100_000_000; // 一亿次

        let start = std::time::Instant::now();

        for _ in 0..iterations {
            let n: i32 = rng.random();
            varint::encode(n, &mut buf).expect("encode failed"); // 6666 Rust发力了
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
            varint::encode(n, &mut buf).expect("encode failed");
            varint::decode(&mut buf.as_slice()).expect("decode failed");
            buf.clear();
        }

        let elapsed = start.elapsed();
        let ns = elapsed.as_nanos();
        let secs = ns as f64 / 1_000_000_000.0;
        let speed = iterations as f64 / secs;

        println!("Time: {:.3}s", secs);
        println!("Encode & Decode speed: {:.0}/s", speed);
    }
    #[test]
    fn test_encode_varlong() {
        println!("Encode varlong test started");
        let mut rng = rand::rng();
        let mut buf = Vec::new();
        let iterations = 100_000_000; // 一亿次

        let start = std::time::Instant::now();

        for _ in 0..iterations {
            let n: i64 = rng.random(); // 从varint改的 awa
            varlong::encode(n, &mut buf).expect("encode failed");
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
            varlong::encode(n, &mut buf).expect("encode failed");
            varlong::decode(&mut buf.as_slice()).expect("decode failed");
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
