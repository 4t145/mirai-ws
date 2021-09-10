use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase", tag="type")]
pub enum Msg {
    #[serde(rename_all = "camelCase")]
    FriendMessage {
        sender: PersonalSender,
        message_chain: Vec<MsgUnit>
    },
    #[serde(rename_all = "camelCase")]
    GroupMessage {
        sender: GroupSender,
        message_chain: Vec<MsgUnit>
    },
    #[serde(rename_all = "camelCase")]
    TempMessage {
        sender: GroupSender,
        message_chain: Vec<MsgUnit>
    },
    #[serde(rename_all = "camelCase")]
    StrangerMessage {
        sender: PersonalSender,
        message_chain: Vec<MsgUnit>
    },
    #[serde(rename_all = "camelCase")]
    OtherClientMessage {
        sender: OtherClientSender,
        message_chain: Vec<MsgUnit>
    },
    #[serde(rename_all = "camelCase")]
    BadMessage,
}

impl Msg {
    pub fn get_sender_qq(&self) -> i64 {
        match self {
            Msg::FriendMessage { sender, message_chain:_ } => sender.id,
            Msg::GroupMessage { sender, message_chain:_ } => sender.id,
            Msg::TempMessage { sender, message_chain:_ } => sender.id,
            Msg::StrangerMessage { sender, message_chain:_ } => sender.id,
            Msg::OtherClientMessage { sender, message_chain:_ } => sender.id,
            _ => 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonalSender {
    id: i64,
    nickname: String,
    remark: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum GroupPermisson {
    Member,
    Administrator,
    Owner,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupInfo {
    id: i64,
    name: String,
    permission: GroupPermisson
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GroupSender {
    id: i64,
    member_name: String,
    special_title: String,
    permission: GroupPermisson,
    join_timestamp: i64,
    last_speak_timestamp: i64,
    mute_time_remaining: i64,
    group: GroupInfo
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ClientDevice {
    Windows,
    Mobile,
    // for the devices those 我也知不道的
    Unknown
}

impl Default for ClientDevice {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OtherClientSender {
    id: i64,
    #[serde(default)]
    platform: ClientDevice
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase", tag="type")]
pub enum MsgUnit {
    Source {
        id: i64,
        time: i64,
    },
    #[serde(rename_all = "camelCase")]
    Quote {
        group_id: i64,
        sender_id: i64,
        target_id: i64,
        origin: Vec<MsgUnit>
    },
    At {
        target: i64,
        display: String
    },
    AtAll,
    #[serde(rename_all = "camelCase")]
    Face {
        face_id: i64,
        name: String
    },
    Plain {
        text: String
    },
    #[serde(rename_all = "camelCase")]
    Image {
        image_id: String,
        url: String,
    },
    #[serde(rename_all = "camelCase")]
    FlashImage{
        image_id: String,
        url: String,
    },
    Xml {
        xml:String
    },
    Json {
        json: String
    },
    App {
        content: String
    },
    Poke {
        name: String
    },
    Dice {
        value: i8
    },
    #[serde(rename_all = "camelCase")]
    MusicShare {
        kind: String,
        title: String,
        summary: String,
        jump_url: String,
        picture_url: String,
        music_url: String,
        brief: String
    },
    #[serde(rename_all = "camelCase")]
    Forward {
        node_list: Vec<MsgNode>
    },
    File {
        id: String,
        name: String,
        size: i64
    },
    MiraiCode {
        code: String
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MsgNode {
    sender_id: i64,
    time: i64,
    sender_name: String,
    message_chain: Vec<MsgUnit>,
    message_id: Option<usize>
}

#[macro_export]
macro_rules! text {
    ($s:expr) => {
        MsgUnit::Plain{text:$s.into()}
    };
    ($($arg:tt)*) => {{
        MsgUnit::Plain{text: format!($($arg)*)}
    }}
}

#[macro_export]
macro_rules! img {
    (url:$url:expr) => {
        MsgUnit::Image{image_id:"".into(), url:($url).into()}
    };
    (id:$id:expr) => {
        MsgUnit::Image{image_id:($id).into(), url:"".into()}
    };
    ($url:expr) => {
        MsgUnit::Image{image_id:"".into(), url:($url).into()}
    };
}

#[macro_export]
macro_rules! at {
    ($target:expr) => {
        MsgUnit::At{target:$target.into(), display:"".into()}
    };
    (/all) => {
        MsgUnit::AtAll
    }
}

#[macro_export]
macro_rules! chain {
    ($($x:expr,)*) => {
        vec![$($x),*]
    };
}