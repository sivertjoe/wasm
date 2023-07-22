use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use gloo_net::websocket::{futures::WebSocket as Socket, Message};
use gloo_utils::errors::JsError;
use wasm_bindgen_futures::spawn_local;

use crate::js::console_log;


fn try_reconnect(url: &str, w: &mut SplitSink<Socket, Message>, r: &mut SplitStream<Socket>)
{
    if let Ok(s) = Socket::open(&url)
    {
        let (_w, _r) = s.split();
        *w = _w;
        *r = _r;
    }
}

pub fn spawn<S: AsRef<str>>(url: S) -> Result<(), JsError>
{
    let url = url.as_ref().to_string();
    let s = Socket::open(&url)?;
    let (mut write, mut read) = s.split();


    spawn_local(async move {
        console_log("waiting..");
        loop
        {
            if let Some(msg) = read.next().await
            {
                if let Ok(Message::Text(msg)) = msg
                {
                    console_log(&msg);
                    if msg == "ping"
                    {
                        write.send(Message::Text("pong".into())).await.unwrap();
                    }
                }
            }
            else
            {
                use gloo_timers::future::TimeoutFuture as Timeout;
                Timeout::new(5_000).await;
                try_reconnect(&url, &mut write, &mut read);
            }
        }
    });

    spawn_local(async move {
        /*write.send(Message::Text(String::from("test"))).await.unwrap();
        write.send(Message::Text(String::from("test 2"))).await.unwrap();*/
        console_log("sivert");
    });

    Ok(())
}
/*loop
{
    // sleep
}*/
