/*
 *
 *  * Created: 2026-3-8 11:54:22
 *  * File: lib.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */

pub use mcproto_derive::{ClientboundPacket, ServerboundPacket};
pub use mcproto_utils as utils;

#[cfg(feature = "network")]
pub use mcproto_network as network;

#[cfg(feature = "http")]
pub use mcproto_http as http;
