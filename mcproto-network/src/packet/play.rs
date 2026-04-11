use std::io::{Read, Write};

use mcproto_derive::{ClientboundPacket, ServerboundPacket};
use mcproto_utils::{
    ClientboundPacketTrait, CodecError, Identifier, Int, Long, PacketCodec, ServerboundPacketTrait,
};
use uuid::Uuid;

use crate::packet::TextComponent;

pub mod block;
pub mod bossbar;
pub mod container;
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

const LPVEC3_MAX_QUANTIZED_VALUE: f64 = 32766.0;
const LPVEC3_ZERO_THRESHOLD: f64 = 3.051944088384301e-5;

#[inline]
fn pack_lpvec3_component(value: f64) -> u64 {
    ((value * 0.5 + 0.5) * LPVEC3_MAX_QUANTIZED_VALUE).round() as u64
}

#[inline]
fn unpack_lpvec3_component(value: u64) -> f64 {
    value.min(LPVEC3_MAX_QUANTIZED_VALUE as u64) as f64 * 2.0 / LPVEC3_MAX_QUANTIZED_VALUE - 1.0
}

impl PacketCodec for LpVec3 {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        let max_coordinate = self.x.abs().max(self.y.abs()).max(self.z.abs());

        if max_coordinate < LPVEC3_ZERO_THRESHOLD {
            0u8.encode(buf)?;
            return Ok(());
        }

        let max_coordinate_i = max_coordinate as i64;
        let scale_factor = if max_coordinate > max_coordinate_i as f64 {
            (max_coordinate_i + 1) as u64
        } else {
            max_coordinate_i as u64
        };

        let need_continuation = (scale_factor & 3) != scale_factor;
        let packed_scale = if need_continuation {
            (scale_factor & 3) | 4
        } else {
            scale_factor
        };

        let packed_x = pack_lpvec3_component(self.x / scale_factor as f64) << 3;
        let packed_y = pack_lpvec3_component(self.y / scale_factor as f64) << 18;
        let packed_z = pack_lpvec3_component(self.z / scale_factor as f64) << 33;
        let packed = packed_z | packed_y | packed_x | packed_scale;

        (packed as u8).encode(buf)?;
        ((packed >> 8) as u8).encode(buf)?;
        ((packed >> 16) as u32).encode(buf)?;

        if need_continuation {
            ((scale_factor >> 2) as i32).encode(buf)?;
        }

        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let byte1 = u8::decode(buf)?;
        if byte1 == 0 {
            return Ok(Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            });
        }

        let byte2 = u8::decode(buf)?;
        let bytes3_to_6 = u32::decode(buf)?;
        let packed = ((bytes3_to_6 as u64) << 16) | ((byte2 as u64) << 8) | byte1 as u64;

        let mut scale_factor = packed & 3;
        if (packed & 4) == 4 {
            scale_factor |= (i32::decode(buf)? as u32 as u64) << 2;
        }

        let scale_factor = scale_factor as f64;

        Ok(Self {
            x: unpack_lpvec3_component((packed >> 3) & 0x7FFF) * scale_factor,
            y: unpack_lpvec3_component((packed >> 18) & 0x7FFF) * scale_factor,
            z: unpack_lpvec3_component((packed >> 33) & 0x7FFF) * scale_factor,
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
        let packed = ((self.x as i64 & 0x3FFFFFF) << 38)
            | ((self.z as i64 & 0x3FFFFFF) << 12)
            | (self.y as i64 & 0xFFF);

        Long(packed).encode(buf)
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let val = Long::decode(buf)?.0;

        let x = (val >> 38) as i32;
        let y = ((val << 52) >> 52) as i32;
        let z = ((val << 26) >> 38) as i32;

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
                expected: "0..5",
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
                expected: "0..8",
            }),
        }
    }
}

#[derive(ClientboundPacket)]
#[packet(id = 0x03)]
pub struct AwardStatistics {
    pub statistics: Statistics,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

impl PacketCodec for Difficulty {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (*self as u8).encode(buf)
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        match u8::decode(buf)? {
            0 => Ok(Difficulty::Peaceful),
            1 => Ok(Difficulty::Easy),
            2 => Ok(Difficulty::Normal),
            3 => Ok(Difficulty::Hard),
            v => Err(CodecError::InvalidEnumValue {
                enum_name: "Difficulty",
                value: v as i32,
                expected: "0..3",
            }),
        }
    }
}

#[derive(ClientboundPacket)]
#[packet(id = 0x0A)]
pub struct ChangeDifficulty {
    pub difficulty: Difficulty,
    pub difficulty_locked: bool,
}

#[derive(ClientboundPacket)]
#[packet(id = 0x0B)]
pub struct ChunkBatchFinished {
    pub batch_size: i32,
}

#[derive(ClientboundPacket)]
#[packet(id = 0x0C)]
pub struct ChunkBatchStart;

pub struct ChunkBiome {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub data: Vec<u8>,
}

impl PacketCodec for ChunkBiome {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.chunk_x.encode(buf)?;
        self.chunk_z.encode(buf)?;
        self.data.encode(buf)?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(Self {
            chunk_x: i32::decode(buf)?,
            chunk_z: i32::decode(buf)?,
            data: Vec::<u8>::decode(buf)?,
        })
    }
}

#[derive(ClientboundPacket)]
#[packet(id = 0x0D)]
pub struct ChunkBiomes {
    pub chunks_biome_data: Vec<ChunkBiome>,
}

#[derive(ClientboundPacket)]
#[packet(id = 0x0E)]
pub struct ClearTitles {
    pub reset: bool,
}

pub struct CommandSuggestion {
    pub match_str: String,
    pub tooltip: Option<TextComponent>,
}

impl PacketCodec for CommandSuggestion {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.match_str.encode(buf)?;
        self.tooltip.encode(buf)?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(Self {
            match_str: String::decode(buf)?,
            tooltip: Option::<TextComponent>::decode(buf)?,
        })
    }
}

#[derive(ClientboundPacket)]
#[packet(id = 0x0F)]
pub struct CommandSuggestionsResponse {
    pub transaction_id: i32,
    pub start: i32,
    pub length: i32,
    pub matches: Vec<CommandSuggestion>,
}

pub struct CommandNode {
    pub flags: u8,
    pub children: Vec<i32>,
    pub redirect_node: Option<i32>,
    pub name: Option<String>,
    pub parser: Option<Identifier>,
    pub properties: Option<Vec<u8>>,
    pub suggestions_type: Option<Identifier>,
}

impl PacketCodec for CommandNode {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.flags.encode(buf)?;
        self.children.encode(buf)?;

        if self.flags & 0x08 != 0 {
            if let Some(v) = self.redirect_node {
                v.encode(buf)?;
            }
        }

        let node_type = self.flags & 0x03;

        if node_type == 1 {
            if let Some(name) = &self.name {
                name.encode(buf)?;
            }
        }

        if node_type == 2 {
            if let Some(name) = &self.name {
                name.encode(buf)?;
            }
            if let Some(parser) = &self.parser {
                parser.encode(buf)?;
            }
            if let Some(props) = &self.properties {
                props.encode(buf)?;
            }
        }

        if self.flags & 0x10 != 0 {
            if let Some(s) = &self.suggestions_type {
                s.encode(buf)?;
            }
        }

        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let flags = u8::decode(buf)?;
        let children = Vec::<i32>::decode(buf)?;

        let redirect_node = if flags & 0x08 != 0 {
            Some(i32::decode(buf)?)
        } else {
            None
        };

        let node_type = flags & 0x03;

        let mut name = None;
        let mut parser = None;
        let mut properties = None;

        if node_type == 1 {
            name = Some(String::decode(buf)?);
        }

        if node_type == 2 {
            name = Some(String::decode(buf)?);
            parser = Some(Identifier::decode(buf)?);
            properties = Some(Vec::<u8>::decode(buf)?);
        }

        let suggestions_type = if flags & 0x10 != 0 {
            Some(Identifier::decode(buf)?)
        } else {
            None
        };

        Ok(Self {
            flags,
            children,
            redirect_node,
            name,
            parser,
            properties,
            suggestions_type,
        })
    }
}

#[derive(ClientboundPacket)]
#[packet(id = 0x10)]
pub struct Commands {
    pub nodes: Vec<CommandNode>,
    pub root_index: i32,
}
