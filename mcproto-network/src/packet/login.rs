/*
 *
 *  * Created: 2026-3-8 1:57:10
 *  * File: login.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use std::io::{Read, Write};
use rand::Rng;
use mcproto_utils::{ServerboundPacketTrait, ClientboundPacketTrait, PacketCodec, CodecError};
use mcproto_derive::{ClientboundPacket, ServerboundPacket};
use uuid::Uuid;

#[derive(ServerboundPacket)]
#[packet(id = 0x00)]
pub struct LoginStart {
    pub name: String,
    pub uuid: Uuid,
}
#[derive(ClientboundPacket)]
#[packet(id = 0x00)]
pub struct Disconnect {
    pub reason_json : String,
}
#[derive(ClientboundPacket)]
#[packet(id = 0x01)]
pub struct EncryptionRequest {
    pub server_id: String, // 通常空的
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
    pub should_authenticate: bool,

}
#[derive(ServerboundPacket)]
#[packet(id = 0x01)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

impl EncryptionResponse {
    pub fn generate_shared_secret() -> [u8; 16] {
        let mut secret = [0u8; 16];
        rand::rng().fill_bytes(secret.as_mut());
        secret
    }
    pub fn new(verify_token: Vec<u8>) -> Self {
        EncryptionResponse {
            shared_secret: EncryptionResponse::generate_shared_secret().to_vec(),
            verify_token,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}
impl PacketCodec for Property { // Array（无长度）
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.name.encode(buf)?;
        self.value.encode(buf)?;
        self.signature.encode(buf)?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(Property {
            name: String::decode(buf)?,
            value: String::decode(buf)?,
            signature: Option::<String>::decode(buf)?, // Prefixed
        })
    }
}
pub struct PropertyList(pub Vec<Property>); // 用于封装 Vec<Property>
impl PacketCodec for PropertyList { // PrefixedArray
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (self.0.len() as i32).encode(buf)?;

        for property in &self.0 {
            property.encode(buf)?;
        }
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let len = i32::decode(buf)? as usize;
        let mut properties = Vec::with_capacity(len);
        for _ in 0..len {
            properties.push(Property::decode(buf)?);
        }
        Ok(PropertyList(properties))
    }
}
#[derive(ClientboundPacket)]
#[packet(id = 0x02)]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    pub properties: PropertyList,
}
// todo LoginPluginResponse
#[derive(ServerboundPacket)]
#[packet(id = 0x03)]
pub struct LoginAcknowledged; //空包

#[derive(ClientboundPacket)]
#[packet(id = 0x03)]
pub struct SetCompression {
    pub threshold: i32,
}
#[derive(ClientboundPacket)]
#[packet(id = 0x05)]
pub struct CookieRequest {
    pub key: String
}
#[derive(ServerboundPacket)]
#[packet(id = 0x05)]
pub struct CookieResponse {
    pub key: String,
    pub value: Option<Vec<u8>>
}