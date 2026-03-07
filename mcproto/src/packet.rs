/*
 *
 *  * Created: 2026-3-7 2:44:56
 *  * File: packet.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use crate::CodecError;

pub trait ServerboundPacket {
    fn packet_id(&self) -> i32;
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), CodecError>;
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, CodecError> where Self: Sized;
}