/*
 *
 *  * Created: 2026-3-8 0:26:40
 *  * File: status.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use mcproto_derive::ServerboundPacket;

#[derive(ServerboundPacket)]
#[packet(id: 0x00)]
pub struct StatusRequest;