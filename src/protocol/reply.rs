use serde::{Serialize};
use crate::protocol::common::MsgUnit;


#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MsgReply {
    #[serde(flatten)]
    pub command: MsgCmd
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]

pub enum NudgeKind { Group, Friend, Stranger }

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase", tag="command", content="content")]
pub enum MsgCmd {
    #[serde(rename_all = "camelCase")]
    SendFriendMessage {
        target: i64,
        quote: Option<i32>,
        message_chain: Vec<MsgUnit>
    },
    #[serde(rename_all = "camelCase")]
    SendGroupMessage {
        target: i64,
        quote: Option<i32>,
        message_chain: Vec<MsgUnit>
    },
    #[serde(rename_all = "camelCase")]
    SendTempMessage {
        qq:i64,
        group:i64,
        quote: Option<i32>,
        message_chain: Vec<MsgUnit>
    },
    #[serde(rename_all = "camelCase")]
    SendNudge {
        target: i64,
        subject: i64,
        kind: NudgeKind
    },
    #[serde(rename_all = "camelCase")]
    Recall {
        target: i64
    }
}

