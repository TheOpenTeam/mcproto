/*
 *
 *  * Created: 2026-3-8 0:26:40
 *  * File: status.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use rand::RngExt;
use mcproto_utils::{ServerboundPacketTrait, ClientboundPacketTrait};
use mcproto_derive::{ClientboundPacket, ServerboundPacket};

use crate::packet::TextComponent;

// 按照正常登录流程排序
// todo: 老SLP（1.6前）
#[derive(ServerboundPacket)]
#[packet(id = 0x00)]
pub struct StatusRequest; // 空的

#[derive(ClientboundPacket)]
#[packet(id = 0x00)]
pub struct StatusResponse {
    pub json_response: TextComponent
}
#[derive(ServerboundPacket)]
#[packet(id = 0x01)]
pub struct PingRequest {
    pub payload: i64,
}
impl PingRequest {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        PingRequest {
            payload: rng.random()
        }
    }
}

#[derive(ClientboundPacket)]
#[packet(id = 0x01)]
pub struct PongResponse {
    pub payload: i64,
}

