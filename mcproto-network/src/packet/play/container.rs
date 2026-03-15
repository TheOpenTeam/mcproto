use std::io::{Read, Write};

use mcproto_derive::ClientboundPacket;
use mcproto_utils::{CodecError, PacketCodec};
use mcproto_utils::{ClientboundPacketTrait, ServerboundPacketTrait};
#[derive(ClientboundPacket)]
#[packet(id = 0x11)]
pub struct CloseContainer {
    pub window_id: u8,
}