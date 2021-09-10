#[test]
fn connect_test() {
    use crate::conn::{MiraiStream, MiraiStreamConfig};
    use crate::protocol::{message::MsgUnit, MiraiReply};
    use futures::{StreamExt, SinkExt};
    use crate::{text, img};
    let send = async {
        let stream = MiraiStream::connect(
            "127.0.0.1:8001", 
            MiraiStreamConfig {
                verify_key:"no-key",
                qq: "2879606187"
            }
        ).await.unwrap();
        let (mut tx, mut rx) = stream.split();

        let msg_chain = vec![
            text!("hello {}", "world"),
            img!(url:"https://i1.hdslb.com/bfs/face/9a0b05e498c2a2eabb336267179c0e3b865ece6d.jpg@150w_150h.jpg")
        ];

        let _res = tx.send(
            MiraiReply::new_friend_reply(
                msg_chain, 
                2129310193, 
                0
            )
        ).await; 
        let s = rx.next().await;
        dbg!(s);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        tx.close().await.unwrap();
    };

    let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .thread_name("send_test")
    .thread_stack_size(3*1024*1024)
    .build()
    .unwrap();

    rt.block_on(send);
}