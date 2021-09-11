use serde::Deserialize;
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