/*
 *
 *  * Created: 2026-3-7 0:29:55
 *  * File: lib.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use std::io::{Read, Write};
use thiserror::Error;
use utils::{varint, varlong};
pub mod utils;
pub mod packet;
#[derive(Debug, Error)]
pub enum CodecError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("VarInt(long) error. Too large value, int max is 2^31 -1 and long max is 2^63-1")]
    VarTooLarge,
    #[error("Encode error")]
    EncodeError,
    #[error("Decode error")]
    DecodeError,
}
pub trait PacketCodec: Sized { // 给基础类型都实现encode和decode
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError>;
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError>;
}
impl PacketCodec for i32 {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        Ok(varint::encode(*self, buf)?)
    }
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(varint::decode(buf)?)
    }
}
impl PacketCodec for i64 {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        Ok(varlong::encode(*self, buf)?)
    }
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(varlong::decode(buf)?)
    }
}
impl PacketCodec for String {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        // 先写长度（VarInt），再写内容
        (self.len() as i32).encode(buf)?;
        buf.write_all(self.as_bytes())?;
        Ok(())
    }
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let len = i32::decode(buf)? as usize;
        let mut bytes = vec![0u8; len];
        buf.read_exact(&mut bytes)?;
        String::from_utf8(bytes).map_err(|_| CodecError::DecodeError)
    }
}