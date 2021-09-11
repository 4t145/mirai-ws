
use std::task::{Poll, Context};
use std::pin::Pin;
use std::io::Error as IoError;
use futures::{Sink, Stream};
use futures::stream::StreamExt;
use futures::sink::SinkExt;


use serde_json::{Error as SerdeError};

use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, tungstenite::{Message as WsMsg, Error as WsError}, client_async};

type WsStream = WebSocketStream<TcpStream>;



#[derive(Debug)]
pub enum MiraiError {
    WebsocketError(WsError),
    StdIoError(IoError)
}

impl From<WsError> for MiraiError {
    fn from(e: WsError) -> Self {
        MiraiError::WebsocketError(e)
    }
}

impl From<IoError> for MiraiError{
    fn from(e: IoError) -> Self {
        MiraiError::StdIoError(e)
    }
}

use crate::protocol::{MiraiRxPack, MiraiTxPack};

pub struct MiraiStream {
    pub ws: WsStream,
}

pub struct MiraiStreamConfig<'s> {
    pub verify_key: &'s str,
    pub qq: &'s str,
}

impl MiraiStream {
    pub async fn connect(addr: &str, cfg: MiraiStreamConfig<'_>) -> Result<Self, MiraiError>{
        let request = format!(
            "ws://{}/message?verifyKey={vk}&qq={qq}", 
            addr, 
            vk = cfg.verify_key, 
            qq = cfg.qq
        );
        let tcp_stream = TcpStream::connect(addr).await.map_err(MiraiError::from)?;
        let (ws, _response) = client_async(&request, tcp_stream).await?;
        dbg!(_response);
        Ok(Self{ws})
    }
}

impl Stream for MiraiStream {
    
    type Item = MiraiRxPack;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let poll = self.ws.poll_next_unpin(cx);
        use Poll::*;
        match poll {
            Pending => Pending,
            Ready(item) => Ready(
                match item {
                    None => None,
                    Some(res) => Some({
                        match res {
                            Ok(ws_msg) => {
                                match ws_msg {
                                    WsMsg::Text(text) => {
                                        let _msg: Result<MiraiRxPack, SerdeError> = serde_json::from_str(text.as_str());
                                        // can't deserde msg
                                        _msg.unwrap_or(MiraiRxPack::unparseable())
                                    },
                                    WsMsg::Binary(_) => todo!(),
                                    WsMsg::Ping(_) => todo!(),
                                    WsMsg::Pong(_) => todo!(),
                                    WsMsg::Close(_) => todo!(),
                                }
                            }
                            // bad inbound ws, just drop it
                            Err(_) => MiraiRxPack::invalid_ws()
                        }
                    })
                }
            )
        }
    }
}


impl Sink<MiraiTxPack> for MiraiStream {
    type Error = MiraiError;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.ws.poll_ready_unpin(cx).map_err(Self::Error::from)
    }

    fn start_send(mut self: Pin<&mut Self>, item: MiraiTxPack) -> Result<(), Self::Error> {
        let ws_msg = WsMsg::Text(serde_json::to_string(&item).unwrap());
        println!("{}",&ws_msg);
        self.ws.start_send_unpin(ws_msg).map_err(Self::Error::from)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.ws.poll_flush_unpin(cx).map_err(Self::Error::from)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.ws.poll_close_unpin(cx).map_err(Self::Error::from)
    }
}




