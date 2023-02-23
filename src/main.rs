use std::time::{SystemTime, UNIX_EPOCH};

use futures::{StreamExt, SinkExt};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;

macro_rules! debug_println {
    ($($t:tt)*) => {{
        #[cfg(debug_assertions)]
        println!($($t)*);
    }}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = "127.0.0.1:46125";
    let listener = TcpListener::bind(server_addr).await?;
    debug_println!("listening at {server_addr}");

    loop {
        let (socket, addr) = listener.accept().await?;
        debug_println!("new connection: {addr}");

        tokio::spawn(async move {
            let mut stream = match tokio_tungstenite::accept_async(socket).await {
                Ok(x) => x,
                Err(e) => return debug_println!("accept ws error {addr} - {e:?}"),
            };
            while let Some(Ok(msg)) = stream.next().await {
                match msg {
                    Message::Text(msg) => {
                        let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                        match stream.send(Message::Text(format!("{msg},{t}"))).await {
                            Ok(()) => debug_println!("sent timestamp {addr} - {t}"),
                            Err(e) => debug_println!("send error {addr} - {e:?}"),
                        }
                    }
                    Message::Close(_) => {
                        debug_println!("closing connection {addr}");
                        break
                    }
                    _ => debug_println!("unknown message type {addr} - {msg:?}"),
                }
            }
        });
    }
}
