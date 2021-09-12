use serde::Deserialize;
use crate::protocol::common::*;
#[derive(Deserialize, Debug)]
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


use serde_repr::Deserialize_repr;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Resp {
    code: Status,
    msg: String,
    // payload: RespPayLoad
} 

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RespPayLoad {
    #[serde(rename_all = "camelCase",)]
    MsgResp {
        message_id: i64
    },
    NoPayLoad{}
}

#[derive(Deserialize_repr, Debug)]
#[repr(u32)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Status {
    Ok = 0,

    WrongVerifyKey = 1,
    BotNotFound = 2,
    SessionInvalid = 3,
    SessionNoAuth = 4,
    TargetNotFound = 5,
    ImageFileNotFound = 6,

    BotNoAuth = 10,
    BotMuted = 20,
    MessageTooLong = 30,

    WrongVisit = 400
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase", tag="type")]
pub enum Evt {
    #[serde(rename_all = "camelCase")]
    GroupRecallEvent {
        author_id: i64,
        message_id: i32,
        time: i32,
        group: GroupInfo,
        operator: GroupSender
    }
}