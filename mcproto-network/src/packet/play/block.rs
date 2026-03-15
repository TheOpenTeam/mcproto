use std::io::{Read, Write};

use mcproto_derive::ClientboundPacket;
use mcproto_utils::{CodecError, PacketCodec, ClientboundPacketTrait, ServerboundPacketTrait};

use super::Position;

#[derive(ClientboundPacket)]
#[packet(id = 0x04)]
pub struct AcknowledgeBlockChange {
    pub sequence_id: i32,
}

#[derive(ClientboundPacket)]
#[packet(id = 0x05)]
pub struct SetBlockDestroyStage {
    pub entity_id: i32,
    pub location: Position,
    pub destroy_stage: u8,
}

#[derive(ClientboundPacket)]
#[packet(id = 0x06)]
pub struct BlockEntityData {
    pub location: Position,
    pub r#type: BlockEntityType,
    pub nbt_data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockEntityType {
    MobSpawner = 0,
    CommandBlock = 1,
    Beacon = 2,
    Skull = 3,
    Conduit = 4,
    Banner = 5,
    StructureBlock = 6,
    EndGateway = 7,
    Sign = 8,
    HangingSign = 9,
    Bed = 10,
    Jigsaw = 11,
    Campfire = 12,
    Beehive = 13,
    SculkSensor = 14,
    CalibratedSculkSensor = 15,
    SculkCatalyst = 16,
    SculkShrieker = 17,
    DecoratedPot = 18,
    Crafter = 19,
}

impl PacketCodec for BlockEntityType {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (*self as i32).encode(buf)
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let value = i32::decode(buf)?;

        match value {
            0 => Ok(BlockEntityType::MobSpawner),
            1 => Ok(BlockEntityType::CommandBlock),
            2 => Ok(BlockEntityType::Beacon),
            3 => Ok(BlockEntityType::Skull),
            4 => Ok(BlockEntityType::Conduit),
            5 => Ok(BlockEntityType::Banner),
            6 => Ok(BlockEntityType::StructureBlock),
            7 => Ok(BlockEntityType::EndGateway),
            8 => Ok(BlockEntityType::Sign),
            9 => Ok(BlockEntityType::HangingSign),
            10 => Ok(BlockEntityType::Bed),
            11 => Ok(BlockEntityType::Jigsaw),
            12 => Ok(BlockEntityType::Campfire),
            13 => Ok(BlockEntityType::Beehive),
            14 => Ok(BlockEntityType::SculkSensor),
            15 => Ok(BlockEntityType::CalibratedSculkSensor),
            16 => Ok(BlockEntityType::SculkCatalyst),
            17 => Ok(BlockEntityType::SculkShrieker),
            18 => Ok(BlockEntityType::DecoratedPot),
            19 => Ok(BlockEntityType::Crafter),
            _ => Err(CodecError::InvalidEnumValue {
                enum_name: "BlockEntityType",
                value,
                expected: "0..19",
            }),
        }
    }
}

pub struct BlockActionData {
    pub action_id: u8,
    pub action_param: u8,
}

impl PacketCodec for BlockActionData {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.action_id.encode(buf)?;
        self.action_param.encode(buf)
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(Self {
            action_id: u8::decode(buf)?,
            action_param: u8::decode(buf)?,
        })
    }
}

#[derive(ClientboundPacket)]
#[packet(id = 0x07)]
pub struct BlockAction {
    pub location: Position,
    pub action: BlockActionData,
    pub block_type: i32,
}

#[derive(ClientboundPacket)]
#[packet(id = 0x08)]
pub struct BlockUpdate {
    pub location: Position,
    pub block_id: i32,
}