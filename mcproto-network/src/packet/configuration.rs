// 配置阶段
use mcproto_utils::{ClientboundPacketTrait, CodecError, Identifier, Int, Long, PacketCodec, ServerboundPacketTrait};
use uuid::Uuid;
use std::io::{Read, Write};
use mcproto_derive::{ClientboundPacket, ServerboundPacket};

#[derive(ClientboundPacket)]
#[packet(id = 0x00)]
pub struct CookieRequest { // 1.20+
    pub key: Identifier, // Identifier
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ChatMode {
    Enabled = 0,
    CommandsOnly = 1,
    Hidden = 2,
}
impl PacketCodec for ChatMode {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (*self as i32).encode(buf)
    }
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let value = i32::decode(buf)?;
        match value {
            0 => Ok(ChatMode::Enabled),
            1 => Ok(ChatMode::CommandsOnly),
            2 => Ok(ChatMode::Hidden),
            _ => Err(CodecError::InvalidEnumValue {enum_name: "Chat Mode", value, expected: "0, 1 or 2"}),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainHand {
    Left = 0,
    Right = 1
}
impl PacketCodec for MainHand {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (*self as i32).encode(buf)
    }
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let value = i32::decode(buf)?;
        match value {
            0 => Ok(MainHand::Left),
            1 => Ok(MainHand::Right),
            _ => Err(CodecError::InvalidEnumValue {enum_name: "Main Hand", value, expected: "0 or 1"}),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleStatus {
    All = 0,
    Decreased = 1,
    Minimal = 2
}
impl PacketCodec for ParticleStatus {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (*self as i32).encode(buf)
    }
    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let value = i32::decode(buf)?;
        match value {
            0 => Ok(ParticleStatus::All),
            1 => Ok(ParticleStatus::Decreased),
            2 => Ok(ParticleStatus::Minimal),
            _ => Err(CodecError::InvalidEnumValue {enum_name: "Particle Status", value, expected: "0, 1 or 2"}),
        }
    }
}
// 皮肤部分显示
// 使用: skin_parts:: CAPE | skin_parts:: JACKET 这样
// MC牛逼克拉斯啊，用BitFlag
pub mod skin_parts {
    pub const CAPE: u8 = 0x01;
    pub const JACKET: u8 = 0x02;
    pub const LEFT_SLEEVE: u8 = 0x04;
    pub const RIGHT_SLEEVE: u8 = 0x08;
    pub const LEFT_PANTS: u8 = 0x10;
    pub const RIGHT_PANTS: u8 = 0x20;
    pub const HAT: u8 = 0x40;

    pub const ALL: u8 =
        CAPE |
        JACKET |
        LEFT_SLEEVE |
        RIGHT_SLEEVE |
        LEFT_PANTS |
        RIGHT_PANTS |
        HAT;
}


#[derive(ClientboundPacket)]
#[packet(id = 0x01)]
pub struct PluginMessage {
    pub channel: Identifier,
    pub data: Vec<u8> // 原始数据
}

#[derive(ClientboundPacket)]
#[packet(id = 0x02)]
pub struct Disconnect {
    pub reason: String
}

#[derive(ClientboundPacket)]
#[packet(id = 0x03)]
pub struct FinishConfiguration; // empty

#[derive(ClientboundPacket)]
#[packet(id = 0x04)]
pub struct KeepAlive {
    // wiki The server will frequently send out a keep-alive, each containing a random ID. The client must respond with the same payload (see Serverbound Keep Alive). If the client does not respond to a Keep Alive packet within 15 seconds after it was sent, the server kicks the client. Vice versa, if the server does not send any keep-alives for 20 seconds, the client will disconnect and yield a "Timed out" exception. 
    pub keep_alive_id: Long, // 大端序Long
}
#[derive(ClientboundPacket)]
#[packet(id = 0x05)]
pub struct Ping {
    pub id: Int // 大端序 Int
}

#[derive(ClientboundPacket)]
#[packet(id = 0x06)]
pub struct ResetChat;
#[derive(ClientboundPacket)]
#[packet(id = 0x07)]
pub struct RegistryData {
    // wiki: Sent by the server to inform the client of the contents of synchronized registries, which are sourced from the server's data packs. Each packet contains the contents of a single registry. The client will accumulate the data contained in these packets during the configuration phase, and validate it once Finish Configuration is received from the server. 
    pub registry_id: Identifier,
    pub entries: Entries
}
#[derive(Debug, Clone)]
pub struct Entry {
    pub entry_id: Identifier,
    pub data: Option<Vec<u8>> // Optional nbt
}
impl PacketCodec for Entry {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.entry_id.encode(buf)?;
        self.data.encode(buf)?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(Self {
            entry_id: Identifier::decode(buf)?,
            data: Option::<Vec<u8>>::decode(buf)?,
        })
    }
}
#[derive(Debug, Clone)]
pub struct Entries(pub Vec<Entry>);

impl PacketCodec for Entries {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (self.0.len() as i32).encode(buf)?; // VarInt length

        for entry in &self.0 {
            entry.encode(buf)?;
        }
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let len = i32::decode(buf)? as usize;
        let mut entries = Vec::with_capacity(len);
        for _ in 0..len {
            entries.push(Entry::decode(buf)?);
        }
        Ok(Entries(entries))
    }
}

#[derive(ClientboundPacket)]
#[packet(id = 0x08)]
pub struct RemoveResourcePack {
    pub uuid: Option<Uuid>
}

#[derive(ClientboundPacket)]
#[packet(id = 0x09)]
pub struct AddResourcePack {
    pub uuid: Uuid,
    pub url: String,
    pub hash: String,
    pub forced: bool,
    pub prompt_message: Option<String>
}

#[derive(ClientboundPacket)]
#[packet(id = 0x0A)]
pub struct StoreCookie {
    pub key: Identifier,
    pub payload: Option<Vec<u8>>,
}

#[derive(ClientboundPacket)]
#[packet(id = 0x0B)]
pub struct Transfer {
    pub host: String,
    pub port: u16
}

#[derive(ClientboundPacket)]
#[packet(id = 0x0C)]
pub struct FeatureFlags {
    pub feature_flags: FeatureFlagList
}

pub struct FeatureFlagList(pub Vec<Identifier>);

impl PacketCodec for FeatureFlagList {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (self.0.len() as i32).encode(buf)?; // VarInt length

        for flag in &self.0 {
            flag.encode(buf)?;
        }

        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let len = i32::decode(buf)? as usize;

        let mut flags = Vec::with_capacity(len);

        for _ in 0..len {
            flags.push(Identifier::decode(buf)?);
        }

        Ok(FeatureFlagList(flags))
    }
}


#[derive(ClientboundPacket)]
#[packet(id = 0x0D)]
pub struct UpdateTags {
    pub tagged_registries: TaggedRegistries
}

pub struct TaggedRegistries(pub Vec<TagRegistry>);

impl PacketCodec for TaggedRegistries {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (self.0.len() as i32).encode(buf)?; // VarInt length

        for registry in &self.0 {
            registry.registry.encode(buf)?;
            registry.tags.encode(buf)?;
        }

        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let len = i32::decode(buf)? as usize;

        let mut registries = Vec::with_capacity(len);

        for _ in 0..len {
            let registry_id = Identifier::decode(buf)?;
            let tags = Tags::decode(buf)?;
            registries.push(TagRegistry {
                registry: registry_id,
                tags,
            });
        }

        Ok(TaggedRegistries(registries))
    }
}

pub struct Tags(pub Vec<Tag>);
pub struct TagRegistry {
    pub registry: Identifier,
    pub tags: Tags,
}
pub struct Tag {
    pub tag_name: Identifier,
    pub entries: Vec<i32>, // Prefixed Array of VarInt 	
}
impl PacketCodec for Tag {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.tag_name.encode(buf)?;
        self.entries.encode(buf)?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(Self {
            tag_name: Identifier::decode(buf)?,
            entries: Vec::<i32>::decode(buf)?,
        })
    }
}
impl PacketCodec for Tags {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (self.0.len() as i32).encode(buf)?; // VarInt length

        for tag in &self.0 {
            tag.encode(buf)?;
        }

        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let len = i32::decode(buf)? as usize;

        let mut tags = Vec::with_capacity(len);

        for _ in 0..len {
            tags.push(Tag::decode(buf)?);
        }

        Ok(Tags(tags))
    }
}


#[derive(ClientboundPacket)]
#[packet(id = 0x0E)]
pub struct ClientboundKnownPacks {
    pub packs: KnownPackList
}

pub struct KnownPack {
    pub namespace: Identifier,
    pub id: Identifier,
    pub version: String,
}

impl PacketCodec for KnownPack {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.namespace.encode(buf)?;
        self.id.encode(buf)?;
        self.version.encode(buf)?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(Self {
            namespace: Identifier::decode(buf)?,
            id: Identifier::decode(buf)?,
            version: String::decode(buf)?,
        })
    }
}

pub struct KnownPackList(pub Vec<KnownPack>);

impl PacketCodec for KnownPackList {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (self.0.len() as i32).encode(buf)?;

        for pack in &self.0 {
            pack.encode(buf)?;
        }

        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let len = i32::decode(buf)? as usize;

        let mut packs = Vec::with_capacity(len);

        for _ in 0..len {
            packs.push(KnownPack::decode(buf)?);
        }

        Ok(KnownPackList(packs))
    }
}

#[derive(ClientboundPacket)]
#[packet(id = 0x0F)]
pub struct CustomReportDetails {
    pub details: ReportDetails
}

pub struct ReportDetail {
    pub key: String,
    pub value: String,
}

impl PacketCodec for ReportDetail {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.key.encode(buf)?;
        self.value.encode(buf)?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(Self {
            key: String::decode(buf)?,
            value: String::decode(buf)?,
        })
    }
}
pub struct ReportDetails(pub Vec<ReportDetail>);

impl PacketCodec for ReportDetails {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (self.0.len() as i32).encode(buf)?;

        for detail in &self.0 {
            detail.encode(buf)?;
        }

        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let len = i32::decode(buf)? as usize;

        let mut details = Vec::with_capacity(len);

        for _ in 0..len {
            details.push(ReportDetail::decode(buf)?);
        }

        Ok(ReportDetails(details))
    }
}

#[derive(ClientboundPacket)]
#[packet(id = 0x10)]
pub struct ServerLinks {
    pub links: ServerLinkList
}

pub struct ServerLink {
    pub link_type: ServerLinkType,
    pub url: String,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServerLinkType {
    BugReport = 0,
    CommunityGuidelines = 1,
    Support = 2,
    Status = 3,
    Feedback = 4,
    Community = 5,
    Website = 6,
    Forums = 7,
    News = 8,
    Announcements = 9,
}
impl PacketCodec for ServerLinkType {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (*self as i32).encode(buf)
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let value = i32::decode(buf)?;

        match value {
            0 => Ok(ServerLinkType::BugReport),
            1 => Ok(ServerLinkType::CommunityGuidelines),
            2 => Ok(ServerLinkType::Support),
            3 => Ok(ServerLinkType::Status),
            4 => Ok(ServerLinkType::Feedback),
            5 => Ok(ServerLinkType::Community),
            6 => Ok(ServerLinkType::Website),
            7 => Ok(ServerLinkType::Forums),
            8 => Ok(ServerLinkType::News),
            9 => Ok(ServerLinkType::Announcements),
            _ => Err(CodecError::InvalidEnumValue {
                enum_name: "ServerLinkType",
                value,
                expected: "0..9",
            }),
        }
    }
}

impl PacketCodec for ServerLink {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        self.link_type.encode(buf)?;
        self.url.encode(buf)?;
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        Ok(Self {
            link_type: ServerLinkType::decode(buf)?,
            url: String::decode(buf)?,
        })
    }
}
pub struct ServerLinkList(pub Vec<ServerLink>);

impl PacketCodec for ServerLinkList {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (self.0.len() as i32).encode(buf)?;
        for link in &self.0 {
            link.encode(buf)?;
        }
        Ok(())
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let len = i32::decode(buf)? as usize;
        let mut links = Vec::with_capacity(len);
        for _ in 0..len {
            links.push(ServerLink::decode(buf)?);
        }

        Ok(ServerLinkList(links))
    }
}
#[derive(ClientboundPacket)]
#[packet(id = 0x11)]
pub struct ClearDialog;

#[derive(ClientboundPacket)]
#[packet(id = 0x12)]
pub struct ShowDialog {
    pub dialog: Vec<u8> // nbt
}

// serverbound
#[derive(ServerboundPacket)]
#[packet(id = 0x00)]
pub struct ClientInformation {
    // wiki: Sent when the player connects, or when settings are changed. 
    pub locale: String, // s16
    pub view_distance: i8, //byte
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8, // Unsigned byte
    pub main_hand: MainHand,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
    pub particle_status: ParticleStatus,
}
#[derive(ServerboundPacket)]
#[packet(id = 0x01)]
pub struct CookieResponse {
    pub key: Identifier,
    pub payload: Option<Vec<u8>> // Prefixed Optional Prefixed Array (5120) of Byte 	

}
#[derive(ServerboundPacket)]
#[packet(id = 0x02)]
pub struct ServerboundPluginMessage {
    pub channel: Identifier,
    pub data: Vec<u8> // varies
}
#[derive(ServerboundPacket)]
#[packet(id = 0x03)]
pub struct AcknowledgeFinishConfiguration;

#[derive(ServerboundPacket)]
#[packet(id = 0x04)]
pub struct ServerboundKeepAlive {
    pub keep_alive_id: Long
}

#[derive(ServerboundPacket)]
#[packet(id = 0x05)]
pub struct Pong {
    pub id: Int
}

#[derive(ServerboundPacket)]
#[packet(id = 0x06)]
pub struct ResourcePackResponse {
    pub uuid: Uuid,
    pub result: ResourcePackResult,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResourcePackResult {
    SuccessfullyLoaded = 0,
    Declined = 1,
    FailedDownload = 2,
    Accepted = 3,
}
impl PacketCodec for ResourcePackResult {
    fn encode(&self, buf: &mut impl Write) -> Result<(), CodecError> {
        (*self as i32).encode(buf)
    }

    fn decode(buf: &mut impl Read) -> Result<Self, CodecError> {
        let value = i32::decode(buf)?;

        match value {
            0 => Ok(ResourcePackResult::SuccessfullyLoaded),
            1 => Ok(ResourcePackResult::Declined),
            2 => Ok(ResourcePackResult::FailedDownload),
            3 => Ok(ResourcePackResult::Accepted),
            _ => Err(CodecError::InvalidEnumValue {
                enum_name: "ResourcePackResult",
                value,
                expected: "0, 1, 2 or 3",
            }),
        }
    }
}
#[derive(ServerboundPacket)]
#[packet(id = 0x07)]
pub struct ServerboundKnownPacks {
    pub known_packs: KnownPackList
}
#[derive(ServerboundPacket)]
#[packet(id = 0x08)]
pub struct CustomClickAction {
    pub id: Identifier,
    pub payload: Vec<u8>
}
#[derive(ServerboundPacket)]
#[packet(id = 0x09)]
pub struct AcceptCodeOfConduct;


