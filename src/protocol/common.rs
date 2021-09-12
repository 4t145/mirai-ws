//! # common data structure
//! 
//! 
//! 
//! 
//! 
//! 
//! 
//! 
//! 
//! 
//! 
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!


use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct PersonalSender {
    pub id: i64,
    pub nickname: String,
    pub remark: String,
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
    pub id: i64,
    pub name: String,
    pub permission: GroupPermisson
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GroupSender {
    pub id: i64,
    pub member_name: String,
    pub special_title: String,
    pub permission: GroupPermisson,
    pub join_timestamp: i64,
    pub last_speak_timestamp: i64,
    pub mute_time_remaining: i64,
    pub group: GroupInfo
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
    pub id: i64,
    #[serde(default)]
    pub platform: ClientDevice
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

/// # Usage
/// 
/// use this macro to create a plain text `MsgUnit`
/// ```rust
/// let msg_chain = vec![
///     text!("hello"),
///     text!("just simply using the macro {}!()", "text")
/// ];
/// ```
#[macro_export]
macro_rules! text {
    ($s:expr) => {
        MsgUnit::Plain{text:$s.into()}
    };
    ($($arg:tt)*) => {{
        MsgUnit::Plain{text: format!($($arg)*)}
    }}
}

/// # Usage
/// 
/// use this macro to create a image `MsgUnit`
/// ```rust
/// let msg_chain = vec![
///     // in default case, this macro create image by url
///     img!("https://some.web.com/img.jpg"),
///     // you can specify whether url 
///     img!(url = "https://some.web.com/img.jpg"),
///     // or image-id you are going to use
///     img!(id = "abcdefg-hijk")
/// ];
/// ```
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

/// # Usage
/// 
/// use this macro to create a image `At`
/// ```rust
/// let msg_chain = vec![
///     // at some body
///     at!(123456789),
///     // at all
///     at!(/all),
/// ];
/// ```
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