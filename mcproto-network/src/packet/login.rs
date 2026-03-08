/*
 *
 *  * Created: 2026-3-8 1:57:10
 *  * File: login.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use mcproto_utils::{ServerboundPacketTrait, ClientboundPacketTrait};
use mcproto_derive::{ClientboundPacket, ServerboundPacket};
use uuid::Uuid;

#[derive(ServerboundPacket)]
#[packet(id = 0x00)]
pub struct LoginStart {
    name: String,
    uuid: Uuid,
}
#[derive(ClientboundPacket)]
#[packet(id = 0x00)]
pub struct Disconnect {
    reason_json : String,
}
#[derive(ClientboundPacket)]
#[packet(id = 0x01)]
pub struct EncryptionRequest {
    server_id: String, // 通常空的
    public_key: Vec<u8>,

}