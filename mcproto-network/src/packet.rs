use std::io::{Read, Write};

use mcproto_utils::{CodecError, PacketCodec};

/*
 *
 *  * Created: 2026-3-8 0:20:28
 *  * File: packet.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
pub mod handshake;
pub mod status;
pub mod login;
pub mod configuration;
pub mod play;

#[derive(Debug, Clone, PartialEq)]
pub struct TextComponent(pub String);
impl PacketCodec for TextComponent {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.0.encode(buf)
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(Self(String::decode(buf)?))
    }
}
