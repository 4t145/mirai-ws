pub mod reply;
pub mod recieve;
pub mod common;

use serde::{Serialize, Deserialize};
use common::MsgUnit;
use recieve::{Msg, Status, RespPayLoad};

use self::recieve::Evt;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MiraiTxPack {
    pub sync_id: i32,
    #[serde(flatten)]
    pub payload: MiraiReply   
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MiraiRxPack {
    pub sync_id: i32,
    pub data: MiraiRecieve
}
impl MiraiRxPack {
    pub(crate) fn unparseable() -> Self {
        Self {
            sync_id: i32::MIN,
            data: MiraiRecieve::Unparseable
        }
    }
    pub(crate) fn invalid_ws() -> Self {
        Self {
            sync_id: i32::MIN,
            data: MiraiRecieve::InvalidWs
        }
    }
}

#[derive(Serialize, Debug)]
pub enum MiraiReply {
    Msg(reply::MsgReply)
}

impl MiraiReply {
    pub fn new_group_reply(msg_chain: Vec<MsgUnit>, group_id: i64) -> Self {
        Self::Msg(reply::MsgReply {
            command: reply::MsgCmd::SendGroupMessage {
                target: group_id,
                quote: None,
                message_chain: msg_chain
            }
        })
    }

    pub fn new_friend_reply(msg_chain: Vec<MsgUnit>, friend_id: i64) -> Self {
        Self::Msg(reply::MsgReply {
            command: reply::MsgCmd::SendFriendMessage {
                target: friend_id,
                quote: None,
                message_chain: msg_chain
            }
        })
    }

    pub fn pack(self) -> MiraiTxPack {
        MiraiTxPack {
            sync_id: -1,
            payload: self
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum MiraiRecieve {
    #[serde(rename_all = "camelCase")]
    Resp{
        code: Status,
        msg: String,
        #[serde(flatten)]
        payload: RespPayLoad
    },
    Msg(Msg),
    Evt(Evt),
    Unparseable,
    InvalidWs
}