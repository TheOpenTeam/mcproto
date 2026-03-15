use std::io::{Read, Write};

use mcproto_derive::{ClientboundPacket, ServerboundPacket};
use mcproto_utils::{ClientboundPacketTrait, CodecError, Identifier, Int, Long, PacketCodec, ServerboundPacketTrait};
use uuid::Uuid;
#[derive(Debug, Clone, Copy, PartialEq)]
// 实现基本类型
// 角度 mc中角度是 真实1度 = 1*256/360 度存储

pub struct Angle(pub u8);
impl PacketCodec for Angle {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.0.encode(buf)
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(Self(u8::decode(buf)?))
    }
}
// 转角度
impl Angle {
    pub fn to_degrees(self) -> f32 {
        (self.0 as f32) * 360.0 / 256.0
    }
    pub fn from_degrees(deg: f32) -> Self {
        Self(((deg * 256.0 / 360.0) as i32 & 0xFF) as u8)
    }
}

// LpVec3 理解为就是三维坐标
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LpVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl PacketCodec for LpVec3 {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.x.encode(buf)?;
        self.y.encode(buf)?;
        self.z.encode(buf)?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(Self {
            x: f64::decode(buf)?,
            y: f64::decode(buf)?,
            z: f64::decode(buf)?,
        })
    }
}
// 坐标
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl PacketCodec for Position {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        let packed =
            ((self.x as i64 & 0x3FFFFFF) << 38) |
            ((self.z as i64 & 0x3FFFFFF) << 12) |
            (self.y as i64 & 0xFFF);

        packed.encode(buf)
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let val = i64::decode(buf)?;

        let x = (val >> 38) as i32;
        let y = (val & 0xFFF) as i32;
        let z = ((val >> 12) & 0x3FFFFFF) as i32;

        Ok(Self { x, y, z })
    }
}




// clientbound

#[derive(ClientboundPacket)]
#[packet(id = 0x00)]
pub struct BundleDelimiter;

#[derive(ClientboundPacket)]
#[packet(id = 0x01)]
pub struct SpawnEntity {
    pub entity_id: i32,
    pub entity_uuid: Uuid,
    pub entity_type: i32,
    pub x: f64, // double
    pub y: f64,
    pub z: f64,
    pub velocity: LpVec3,
    pub pitch: Angle,
    pub yaw: Angle,
    pub head_yaw: Angle,
    pub data: i32,
}
#[derive(ClientboundPacket)]
#[packet(id = 0x02)]
pub struct EntityAnimation {
    pub entity_id: i32,
    pub animation: EntityAnimationType,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityAnimationType {
    SwingMainArm = 0,
    TakeDamage = 1,
    LeaveBed = 2,
    SwingOffHand = 3,
    CriticalEffect = 4,
    MagicCriticalEffect = 5,
}
impl PacketCodec for EntityAnimationType {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (*self as u8).encode(buf)
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let value = u8::decode(buf)?;

        match value {
            0 => Ok(EntityAnimationType::SwingMainArm),
            // 1 => Ok(EntityAnimationType::TakeDamage), // 666 wiki没有1
            2 => Ok(EntityAnimationType::LeaveBed),
            3 => Ok(EntityAnimationType::SwingOffHand),
            4 => Ok(EntityAnimationType::CriticalEffect),
            5 => Ok(EntityAnimationType::MagicCriticalEffect),
            _ => Err(CodecError::InvalidEnumValue {
                enum_name: "EntityAnimationType",
                value: value as i32,
                expected: "0, 2, 3, 4 or 5",
            }),
        }
    }
}

pub struct Statistic {
    pub category_id: StatisticCategory,
    pub statistic_id: i32,
    pub value: i32,
}
impl PacketCodec for Statistic {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.category_id.encode(buf)?;
        self.statistic_id.encode(buf)?;
        self.value.encode(buf)?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(Self {
            category_id: StatisticCategory::decode(buf)?,
            statistic_id: i32::decode(buf)?,
            value: i32::decode(buf)?,
        })
    }
}

pub struct Statistics(pub Vec<Statistic>);
impl PacketCodec for Statistics {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (self.0.len() as i32).encode(buf)?;
        for stat in &self.0 {
            stat.encode(buf)?;
        }
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let len = i32::decode(buf)? as usize;
        let mut stats = Vec::with_capacity(len);
        for _ in 0..len {
            stats.push(Statistic::decode(buf)?);
        }
        Ok(Statistics(stats))
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatisticCategory {
    Mined = 0,
    Crafted = 1,
    Used = 2,
    Broken = 3,
    PickedUp = 4,
    Dropped = 5,
    Killed = 6,
    KilledBy = 7,
    Custom = 8,
}

impl PacketCodec for StatisticCategory {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (*self as i32).encode(buf)
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let value = i32::decode(buf)?;

        match value {
            0 => Ok(StatisticCategory::Mined),
            1 => Ok(StatisticCategory::Crafted),
            2 => Ok(StatisticCategory::Used),
            3 => Ok(StatisticCategory::Broken),
            4 => Ok(StatisticCategory::PickedUp),
            5 => Ok(StatisticCategory::Dropped),
            6 => Ok(StatisticCategory::Killed),
            7 => Ok(StatisticCategory::KilledBy),
            8 => Ok(StatisticCategory::Custom),
            _ => Err(CodecError::InvalidEnumValue {
                enum_name: "StatisticCategory",
                value,
                expected: "0-8",
            }),
        }
    }
}

#[derive(ClientboundPacket)]
#[packet(id = 0x03)]
pub struct AwardStatistics {
    pub statistics: Statistics
}

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
    pub destroy_stage: u8
}

#[derive(ClientboundPacket)]
#[packet(id = 0x06)]
pub struct BlockEntityData {
    pub location: Position,
    pub r#type: BlockEntityType, // 关键字
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
#[derive(ClientboundPacket)]
#[packet(id = 0x07)]
pub struct BlockAction {
    pub location: Position,
    pub action: BlockActionData, // 为了安全，其次是懒，懒得搞enum了
    pub block_type: i32,
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
#[packet(id = 0x08)]
pub struct BlockUpdate {
    pub location: Position,
    pub block_id: i32
}

