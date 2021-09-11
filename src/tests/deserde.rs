#[test]
fn deserde_rxpack_msg() {
    use crate::protocol::MiraiRxPack;
    use serde_json;
    let rxpack_json = include_str!("mockdata/rx/msg_pack.json");
    let _rxpack:MiraiRxPack = serde_json::from_str(rxpack_json).unwrap();
}

#[test]
fn deserde_rxpack_resp() {
    use crate::protocol::MiraiRxPack;
    use serde_json;
    let rxpack_json = include_str!("mockdata/rx/resp_pack.json");
    let _rxpack:MiraiRxPack = serde_json::from_str(rxpack_json).unwrap();
}