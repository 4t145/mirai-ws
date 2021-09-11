# 使用

```rust
use crate::conn::{MiraiStream, MiraiStreamConfig};
use crate::protocol::{message::MsgUnit, MiraiReply};
use futures::{StreamExt, SinkExt};
use crate::{text, img};

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

tx.send( 
    MiraiReply::new_friend_reply(msg_chain, 987654321).pack() 
).await.unwrap();

let resp = rx.next().await;
println!("get response {:?}", resp);
```