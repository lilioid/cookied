use std::net::SocketAddr;
use std::sync::Arc;

use clap::Parser;
use time::format_description;
use time::OffsetDateTime;
use time::UtcOffset;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::UdpSocket;

mod cli;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Arc::new(cli::Cli::parse());
    let join_tcp = tokio::spawn({
        let cli = cli.clone();
        async move { listen_tcp(cli).await }
    });
    let join_udp = tokio::spawn(async move { listen_udp(cli).await });

    let _ = tokio::join!(join_tcp, join_udp);
}

async fn listen_tcp(cli: Arc<cli::Cli>) {
    eprintln!("Listening on tcp {}:{}", cli.bind, cli.port);
    let listener = TcpListener::bind((cli.bind, cli.port))
        .await
        .expect("Could not bind to TCP socket");

    loop {
        let incoming = listener.accept().await;
        match incoming {
            Err(e) => {
                eprintln!("Could not accept incoming connection: {}", e);
            }
            Ok((mut stream, remote_addr)) => {
                eprintln!("New connection from {remote_addr}");
                let quote = generate_quote(&cli, &remote_addr);
                eprintln!("Sending quote to {remote_addr}");
                stream
                    .write_all(quote.as_bytes())
                    .await
                    .expect("Could not write quote to remote client");
                eprintln!("Closing connection to {remote_addr}")
            }
        }
    }
}

async fn listen_udp(cli: Arc<cli::Cli>) {
    eprintln!("Listening on udp {}:{}", cli.bind, cli.port);
    let socket = UdpSocket::bind((cli.bind, cli.port))
        .await
        .expect("Could not bind UDP socket");
    let mut recv_buf = [0u8; 1024];
    loop {
        let (_, remote_addr) = socket
            .recv_from(&mut recv_buf)
            .await
            .expect("Could not receive UDP datagram");
        eprintln!("Received datagram from {remote_addr}");
        let quote = generate_quote(&cli, &remote_addr);
        eprintln!("Sending quote to {remote_addr}");
        socket
            .send_to(quote.as_bytes(), remote_addr)
            .await
            .expect("Could not send response UDP datagram");
    }
}

fn generate_quote(cli: &cli::Cli, remote_addr: &SocketAddr) -> String {
    match cli.alg {
        cli::ResponseAlgorithm::Pattern => {
            const PATTERN: [u8; 64] = [0x55; 64];
            std::str::from_utf8(&PATTERN).unwrap().to_string()
        }
        cli::ResponseAlgorithm::TimeAndPlace => {
            let now = OffsetDateTime::now_utc().to_offset(UtcOffset::from_hms(-10, 0, 0).unwrap());
            let format =
                format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
            format!(
                "Hello, you are {remote_addr} and it is now {} in Hawaii\n",
                now.format(&format).unwrap()
            )
        }
        cli::ResponseAlgorithm::Text => cli.text.to_owned(),
    }
}
