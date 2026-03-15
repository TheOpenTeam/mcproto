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
        varlong::encode(*self, buf);
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
// prefixed optional uuid
impl PacketCodec for Option<Uuid> {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        match self {
            Some(uuid) => {
                true.encode(buf)?;
                buf.write_all(uuid.as_bytes())?;
            }
            None => {
                false.encode(buf)?;
            }
        }
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let has_uuid = bool::decode(buf)?;

        if has_uuid {
            let mut bytes = [0u8; 16];
            buf.read_exact(&mut bytes)?;
            Ok(Some(Uuid::from_bytes(bytes)))
        } else {
            Ok(None)
        }
    }
}

// prefixed array
impl PacketCodec for Vec<u8> {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (self.len() as i32).encode(buf)?;
        buf.write_all(self)?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let len = i32::decode(buf)? as usize;
        let mut data = vec![0u8; len];
        buf.read_exact(&mut data)?;
        Ok(data)
    }
}
// Prefixed Optional str
impl PacketCodec for Option<String> {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        if let Some(s) = self {
            true.encode(buf)?;
            s.encode(buf)?;
            Ok(())
        } else {
            (-1_i32).encode(buf)?;
            Ok(())
        }
    }
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        if bool::decode(buf)? {  // 读标志位
            Ok(Some(String::decode(buf)?))
        } else {
            Ok(None)
        }
    }
}
// Prefixed Optional Prefixed array of bytes
impl PacketCodec for Option<Vec<u8>> {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        match self {
            Some(bytes) => {
                true.encode(buf)?;      // 有内容，写 true
                bytes.encode(buf)?;      // 内层 Vec<u8> 自己处理长度
            }
            None => {
                false.encode(buf)?;      // 无内容，只写 false
            }
        }
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        if bool::decode(buf)? {          // 读标志位
            Ok(Some(Vec::<u8>::decode(buf)?))  // 有内容就读 Vec<u8>
        } else {
            Ok(None)                     // 没内容直接返回 None
        }
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
// Prefixed array of varint
impl PacketCodec for Vec<i32> {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (self.len() as i32).encode(buf)?; // VarInt length

        for v in self {
            v.encode(buf)?;
        }

        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let len = i32::decode(buf)? as usize;

        let mut values = Vec::with_capacity(len);

        for _ in 0..len {
            values.push(i32::decode(buf)?);
        }

        Ok(values)
    }
}
