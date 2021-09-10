use serde::{Deserialize};

use crate::protocol::message::Msg;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyncMsg {
    pub sinc_id: i32,
    pub data: Msg
}

impl SyncMsg {
    pub fn bad() -> Self {
        Self {
            sinc_id: -1,
            data: Msg::BadMessage
        }
    }
}