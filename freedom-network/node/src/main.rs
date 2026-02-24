use tokio::io::{AsyncReadExt, AsyncWriteExt};
use quinn::{Endpoint, ServerConfig};
use rcgen::generate_simple_self_signed;
use std::sync::Arc;
use std::net::SocketAddr;
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cert = generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_der = cert.serialize_der()?;
    let key_der = cert.serialize_private_key_der();

    let mut server_config = ServerConfig::with_single_cert(
        vec![rustls::Certificate(cert_der)],
        rustls::PrivateKey(key_der)
    )?;
    server_config.transport = Arc::new(quinn::TransportConfig::default());

    let addr: SocketAddr = "127.0.0.1:5000".parse()?;
    let (_endpoint, mut incoming) = Endpoint::server(server_config, addr)?;

    println!("Freedom QUIC Node running on {}", addr);

    while let Some(conn) = incoming.next().await {
        tokio::spawn(async move {
            if let Ok(new_conn) = conn.await {
                println!("New connection established");
                while let Some(stream) = new_conn.bi_streams.next().await {
                    if let Ok((mut send, mut recv)) = stream {
                        let mut buf = vec![0;4096];
                        let n = recv.read(&mut buf).await.unwrap_or(0);
                        println!("Received: {:?}", &buf[..n]);
                        send.write_all(b"ACK").await.unwrap();
                    }
                }
            }
        });
    }
    Ok(())
}