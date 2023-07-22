use std::time::Duration;

use futures::{stream::StreamExt, SinkExt};
use tokio::{
    net::{TcpListener, TcpStream},
    select, time,
};
use tokio_tungstenite::{accept_hdr_async, WebSocketStream};
use tungstenite::{
    handshake::server::Request,
    Message::{Ping, Pong, Text},
    Result,
};


#[derive(Debug)]
pub struct Message {}


#[derive(Default)]
struct State {}


async fn get_websocket_and_uri(
    stream: TcpStream,
) -> Result<(WebSocketStream<TcpStream>, hyper::Uri), tungstenite::Error>
{
    let mut uri = None;
    let ws_stream = accept_hdr_async(stream, |req: &Request, res| {
        uri = Some(req.uri().clone());
        Ok(res)
    })
    .await?;
    let uri = uri.unwrap();
    Ok((ws_stream, uri))
}

pub async fn spawn_web_socket_server()
{
    let addr = "0.0.0.0:5000";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    println!("spawning websocket addr at {}", addr);


    loop
    {
        if let Ok((stream, _)) = listener.accept().await
        {
            if let Ok((ws, _uri)) = get_websocket_and_uri(stream).await
            {
                tokio::spawn(handle_connection(ws));
            }
        }
    }
}

async fn handle_connection(mut ws: WebSocketStream<TcpStream>)
{
    println!("ENTER");


    let mut interval = time::interval(Duration::from_secs(5));
    interval.reset();

    loop
    {
        select! {

            _ = interval.tick() =>
            {
                let msg = Text("ping".to_string());
                if ws.send(msg).await.is_err()
                {
                    break;
                }

            }

            res = ws.next() =>
            {
                match res
                {
                    Some(Ok(Text(s))) if s == "pong" => {
                        }

                    Some(Ok(Text(s))) => {
                            println!("{}", s);
                        }
                    _ => break,
                }
            }
        }
    }
    println!("DROPED");
}
