pub mod reply;
pub mod message;
pub mod recieve;

use message::MsgUnit;

pub enum MiraiReply {
    MsgReply(reply::MsgReply)
}

impl MiraiReply {
    pub fn json(&self) -> String {
        match self {
            MiraiReply::MsgReply(msg_reply) => serde_json::json!(msg_reply).to_string(),
        }
    }

    pub fn new_group_reply(msg_chain: Vec<MsgUnit>, group_id: i64, sync_id: i32) -> Self {
        Self::MsgReply(reply::MsgReply {
            sync_id: sync_id,
            command: reply::MsgCmd::SendGroupMessage {
                target: group_id,
                quote: None,
                message_chain: msg_chain
            }
        })
    }

    pub fn new_friend_reply(msg_chain: Vec<MsgUnit>, friend_id: i64, sync_id: i32) -> Self {
        Self::MsgReply(reply::MsgReply {
            sync_id,
            command: reply::MsgCmd::SendFriendMessage {
                target: friend_id,
                quote: None,
                message_chain: msg_chain
            }
        })
    }
}