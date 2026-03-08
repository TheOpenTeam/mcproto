/*
 *
 *  * Created: 2026-3-8 10:59:37
 *  * File: handshake.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */

use mcproto_utils::ServerboundPacketTrait;
use std::io::{Read, Write};
use mcproto_derive::ServerboundPacket;
use mcproto_utils::{CodecError, PacketCodec};
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NextState {
    // 下一个状态 1 = status， 2 = login
    Status = 1,
    Login = 2,
    Transfer = 3,
}
impl PacketCodec for NextState {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (*self as i32).encode(buf)
    }
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let value = i32::decode(buf)?;
        match value {
            1 => Ok(NextState::Status),
            2 => Ok(NextState::Login),
            3 => Ok(NextState::Transfer),
            _ => Err(CodecError::InvalidEnumValue {enum_name: "NextState", value, expected: "1 or 2"}),
        }
    }
}
#[derive(ServerboundPacket)]
#[packet(id = 0x00)]
pub struct Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: NextState,
}