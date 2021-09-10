# 使用

```rust
let stream = MiraiStream::connect(
    "127.0.0.1:8001", 
    MiraiStreamConfig {
        verify_key:"no-key",
        qq: "123456789"
    }
).await.unwrap();

let (mut tx, mut rx) = stream.split();

let msg_chain = vec![
    text!("hello {}", "world"),
    img!(url:"https://some.web/img.jpg")
];

let _res = tx.send(
    MiraiReply::new_friend_reply(msg_chain, 987654321, 0)
).await;

let response = rx.next().await;
```