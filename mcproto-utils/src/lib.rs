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
use uuid::Uuid;

pub mod utils;
use crate::utils::{varint, varlong};
pub type Identifier = String;
pub trait ServerboundPacketTrait {
    fn packet_id(&self) -> i32;
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError>;
}
pub trait ClientboundPacketTrait {
    fn packet_id(&self) -> i32;
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> where Self: Sized;
}
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
    #[error("Invalid enum value {value} for {enum_name}, expected {expected}")]
    InvalidEnumValue {
        value: i32,
        enum_name: &'static str,
        expected: &'static str,
    },
}
pub trait PacketCodec: Sized { // 给基础类型都实现encode和decode
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError>;
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError>;
}
impl PacketCodec for i32 {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        varint::encode(*self, buf)?;
        Ok(())
    }
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(varint::decode(buf)?)
    }
}
impl PacketCodec for i64 {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        varlong::encode(*self, buf)?;
        Ok(())
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
impl PacketCodec for bool {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        // true = 0x01, false = 0x00
        buf.write_all(&[*self as u8])?;
        Ok(())
    }
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let mut byte = [0u8; 1];
        buf.read_exact(&mut byte)?;
        match byte[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(CodecError::DecodeError),
        }
    }
}
// u8和i8完全一样,大端序，但是只有1个字节就明文
impl PacketCodec for u8 {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        buf.write_all(&[*self])?;
        Ok(())
    }
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let mut byte = [0u8; 1];
        buf.read_exact(&mut byte)?;
        Ok(byte[0])
    }
}
impl PacketCodec for i8 {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        buf.write_all(&[*self as u8])?;
        Ok(())
    }
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let mut byte = [0u8; 1];
        buf.read_exact(&mut byte)?;
        Ok(byte[0] as i8)
    }
}
// i16和u16是端口的大端序
impl PacketCodec for u16 {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let mut bytes = [0u8; 2];
        buf.read_exact(&mut bytes)?;
        Ok(u16::from_be_bytes(bytes))
    }
}

impl PacketCodec for i16 {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let mut bytes = [0u8; 2];
        buf.read_exact(&mut bytes)?;
        Ok(i16::from_be_bytes(bytes))
    }
}

impl PacketCodec for u32 {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let mut bytes = [0u8; 4];
        buf.read_exact(&mut bytes)?;
        Ok(u32::from_be_bytes(bytes))
    }
}


impl PacketCodec for u64 {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        buf.write_all(&self.to_be_bytes())?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let mut bytes = [0u8; 8];
        buf.read_exact(&mut bytes)?;
        Ok(u64::from_be_bytes(bytes))
    }
}
impl PacketCodec for f32 {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        buf.write_all(&self.to_bits().to_be_bytes())?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let mut bytes = [0u8; 4];
        buf.read_exact(&mut bytes)?;
        Ok(f32::from_bits(u32::from_be_bytes(bytes)))
    }
}

impl PacketCodec for f64 {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        buf.write_all(&self.to_bits().to_be_bytes())?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let mut bytes = [0u8; 8];
        buf.read_exact(&mut bytes)?;
        Ok(f64::from_bits(u64::from_be_bytes(bytes)))
    }
}

impl PacketCodec for Uuid {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        buf.write_all(self.as_bytes())?;
        Ok(())
    }
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let mut bytes = [0u8; 16];
        buf.read_exact(&mut bytes)?;
        Uuid::from_slice(&bytes).map_err(|_| CodecError::DecodeError)
    }
}


pub struct Int(pub i32); // 大端序 Int
impl PacketCodec for Int {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        buf.write_all(&self.0.to_be_bytes())?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let mut bytes = [0u8; 4];
        buf.read_exact(&mut bytes)?;
        Ok(Int(i32::from_be_bytes(bytes)))
    }
}
pub struct Long(pub i64); // 大端序 Long
impl PacketCodec for Long {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        buf.write_all(&self.0.to_be_bytes())?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let mut bytes = [0u8; 8];
        buf.read_exact(&mut bytes)?;
        Ok(Long(i64::from_be_bytes(bytes)))
    }
}
// 所有 Bool-prefixed Optional X
impl<T: PacketCodec> PacketCodec for Option<T> {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        match self {
            Some(value) => {
                true.encode(buf)?;
                value.encode(buf)
            }
            None => {
                false.encode(buf)
            }
        }
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let present = bool::decode(buf)?;

        if present {
            Ok(Some(T::decode(buf)?))
        } else {
            Ok(None)
        }
    }
}
// Length-Prefixed Optional X
#[derive(Debug, Clone)]
pub struct PrefixedString(pub Option<String>);

impl PacketCodec for PrefixedString {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        match &self.0 {
            Some(v) => v.encode(buf),
            None => 0i32.encode(buf),
        }
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let len = i32::decode(buf)?;

        if len == 0 {
            Ok(Self(None))
        } else {
            let mut bytes = vec![0u8; len as usize];
            buf.read_exact(&mut bytes)?;
            Ok(Self(Some(String::from_utf8(bytes).map_err(|_| CodecError::DecodeError)?)))
        }
    }
}



// Array of X
impl<T: PacketCodec> PacketCodec for Vec<T> {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (self.len() as i32).encode(buf)?;

        for item in self {
            item.encode(buf)?;
        }

        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let len = i32::decode(buf)? as usize;

        let mut values = Vec::with_capacity(len);

        for _ in 0..len {
            values.push(T::decode(buf)?);
        }

        Ok(values)
    }
}
