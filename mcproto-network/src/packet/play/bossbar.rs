use std::io::{Read, Write};
use uuid::Uuid;
use mcproto_utils::{PacketCodec, CodecError};

use crate::packet::TextComponent;

#[derive(Debug, Clone, Copy)]
pub enum BossBarColor {
    Pink = 0,
    Blue = 1,
    Red = 2,
    Green = 3,
    Yellow = 4,
    Purple = 5,
    White = 6,
}

impl PacketCodec for BossBarColor {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (*self as i32).encode(buf)
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        match i32::decode(buf)? {
            0 => Ok(Self::Pink),
            1 => Ok(Self::Blue),
            2 => Ok(Self::Red),
            3 => Ok(Self::Green),
            4 => Ok(Self::Yellow),
            5 => Ok(Self::Purple),
            6 => Ok(Self::White),
            v => Err(CodecError::InvalidEnumValue {
                value: v,
                enum_name: "BossBarColor",
                expected: "0..6",
            }),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BossBarDivision {
    NoDivision = 0,
    SixNotches = 1,
    TenNotches = 2,
    TwelveNotches = 3,
    TwentyNotches = 4,
}

impl PacketCodec for BossBarDivision {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (*self as i32).encode(buf)
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        match i32::decode(buf)? {
            0 => Ok(Self::NoDivision),
            1 => Ok(Self::SixNotches),
            2 => Ok(Self::TenNotches),
            3 => Ok(Self::TwelveNotches),
            4 => Ok(Self::TwentyNotches),
            v => Err(CodecError::InvalidEnumValue {
                value: v,
                enum_name: "BossBarDivision",
                expected: "0..4",
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BossBarAction {
    Add {
        title: TextComponent,
        health: f32,
        color: BossBarColor,
        division: BossBarDivision,
        flags: u8,
    },
    Remove,
    UpdateHealth {
        health: f32,
    },
    UpdateTitle {
        title: TextComponent,
    },
    UpdateStyle {
        color: BossBarColor,
        division: BossBarDivision,
    },
    UpdateFlags {
        flags: u8,
    },
}

#[derive(Debug, Clone)]
pub struct BossBar {
    pub uuid: Uuid,
    pub action: BossBarAction,
}

impl PacketCodec for BossBar {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.uuid.encode(buf)?;

        match &self.action {
            BossBarAction::Add { title, health, color, division, flags } => {
                0i32.encode(buf)?;
                title.encode(buf)?;
                health.encode(buf)?;
                color.encode(buf)?;
                division.encode(buf)?;
                flags.encode(buf)?;
            }
            BossBarAction::Remove => {
                1i32.encode(buf)?;
            }
            BossBarAction::UpdateHealth { health } => {
                2i32.encode(buf)?;
                health.encode(buf)?;
            }
            BossBarAction::UpdateTitle { title } => {
                3i32.encode(buf)?;
                title.encode(buf)?;
            }
            BossBarAction::UpdateStyle { color, division } => {
                4i32.encode(buf)?;
                color.encode(buf)?;
                division.encode(buf)?;
            }
            BossBarAction::UpdateFlags { flags } => {
                5i32.encode(buf)?;
                flags.encode(buf)?;
            }
        }

        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let uuid = Uuid::decode(buf)?;
        let action_id = i32::decode(buf)?;

        let action = match action_id {
            0 => BossBarAction::Add {
                title: TextComponent::decode(buf)?,
                health: f32::decode(buf)?,
                color: BossBarColor::decode(buf)?,
                division: BossBarDivision::decode(buf)?,
                flags: u8::decode(buf)?,
            },
            1 => BossBarAction::Remove,
            2 => BossBarAction::UpdateHealth {
                health: f32::decode(buf)?,
            },
            3 => BossBarAction::UpdateTitle {
                title: TextComponent::decode(buf)?,
            },
            4 => BossBarAction::UpdateStyle {
                color: BossBarColor::decode(buf)?,
                division: BossBarDivision::decode(buf)?,
            },
            5 => BossBarAction::UpdateFlags {
                flags: u8::decode(buf)?,
            },
            v => {
                return Err(CodecError::InvalidEnumValue {
                    value: v,
                    enum_name: "BossBarAction",
                    expected: "0..5",
                })
            }
        };

        Ok(Self { uuid, action })
    }
}