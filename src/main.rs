use std::time::{SystemTime, UNIX_EPOCH};

use futures::{StreamExt, SinkExt};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;
use clap::Parser;

/// Host a simple timesync server.
#[derive(Parser)]
struct Args {
    /// Port to use for receiving ws connections.
    #[arg(short, long, default_value_t = 46125)]
    port: u16,
}

macro_rules! debug_println {
    ($($t:tt)*) => {{
        #[cfg(debug_assertions)]
        println!($($t)*);
    }}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let server_addr = format!("0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(&server_addr).await?;
    println!("listening at {server_addr}");

    loop {
        let (socket, _addr) = listener.accept().await?;
        debug_println!("new connection: {_addr}");

        tokio::spawn(async move {
            let mut stream = match tokio_tungstenite::accept_async(socket).await {
                Ok(x) => x,
                Err(_e) => return debug_println!("accept ws error {_addr} - {_e:?}"),
            };
            while let Some(Ok(msg)) = stream.next().await {
                match msg {
                    Message::Text(msg) => {
                        let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                        match stream.send(Message::Text(format!("{msg},{t}"))).await {
                            Ok(()) => debug_println!("sent timestamp {_addr} - {t}"),
                            Err(_e) => debug_println!("send error {_addr} - {_e:?}"),
                        }
                    }
                    Message::Close(_) => {
                        debug_println!("closing connection {_addr}");
                        break
                    }
                    _ => debug_println!("unknown message type {_addr} - {msg:?}"),
                }
            }
        });
    }
}
