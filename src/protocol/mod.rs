pub mod reply;
pub mod message;
pub mod recieve;
pub mod response;
use serde::{Serialize, Deserialize};
use message::MsgUnit;
use message::Msg;
use response::{Status, RespPayLoad};

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

#[derive(Serialize, Debug)]
pub enum MiraiReply {
    Msg(reply::MsgReply)
}

impl MiraiReply {
    pub fn json(&self) -> String {
        match self {
            MiraiReply::Msg(msg_reply) => serde_json::json!(msg_reply).to_string(),
        }
    }

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
}




