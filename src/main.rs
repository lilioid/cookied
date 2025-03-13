use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use listenfd::ListenFd;
use time::format_description;
use time::OffsetDateTime;
use time::UtcOffset;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::net::UdpSocket;
use tokio::task::JoinHandle;

mod cli;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Arc::new(cli::Cli::parse());
    let join_handles = take_listeners(cli)?;
    for i in join_handles {
        let _ = tokio::join!(i);
    }
    Ok(())
}

/// Take listeners from the environment (passed via systemd socket-activation or systemfd)
fn take_listeners(cli: Arc<cli::Cli>) -> anyhow::Result<Vec<JoinHandle<()>>> {
    let mut join_handles = Vec::new();

    // take sockets from environment first
    let mut env_fds = ListenFd::from_env();
    eprintln!(
        "Taking {} file descriptors from the environment",
        env_fds.len()
    );
    for i in 0..env_fds.len() {
        let cli = cli.clone();

        // tcp
        if let Ok(Some(listener)) = env_fds.take_tcp_listener(i) {
            eprintln!("Handling TCP listener on {}", listener.local_addr()?);
            listener
                .set_nonblocking(true)
                .context("Could not configure passed socket for nonblocking IO")?;
            let listener = TcpListener::from_std(listener)
                .context("Could not convert stdlib socket into tokio socket")?;
            join_handles.push(tokio::spawn(async move {
                handle_tcp_listener(cli, listener).await
            }));
        }
        // udp
        else if let Ok(Some(socket)) = env_fds.take_udp_socket(i) {
            eprintln!("Handling UDP socket on {}", socket.local_addr()?);
            socket
                .set_nonblocking(true)
                .context("Could not configure passed socket for nonblocking IO")?;
            let socket = UdpSocket::from_std(socket)
                .context("Could not convert stdlib socket into tokio socket")?;
            join_handles.push(tokio::spawn(
                async move { handle_udp_socket(cli, socket).await },
            ))
        }
    }

    Ok(join_handles)
}

async fn handle_tcp_listener(cli: Arc<cli::Cli>, listener: TcpListener) {
    loop {
        let incoming = listener.accept().await;
        match incoming {
            Err(e) => {
                eprintln!("Could not accept incoming connection: {}", e);
            }
            Ok((mut stream, remote_addr)) => {
                eprintln!("Sending quote of the day to tcp://{remote_addr}");
                let quote = generate_quote(&cli, &remote_addr);
                if let Err(e) = stream.write_all(quote.as_bytes()).await {
                    eprintln!("Could not send quote to tcp://{remote_addr}: {e}");
                }
            }
        }
    }
}

async fn handle_udp_socket(cli: Arc<cli::Cli>, socket: UdpSocket) {
    let mut recv_buf = [0u8; 1024];
    loop {
        let (_, remote_addr) = socket
            .recv_from(&mut recv_buf)
            .await
            .expect("Could not receive UDP datagram");
        eprintln!("Sending quote of the day to udp://{remote_addr}");
        let quote = generate_quote(&cli, &remote_addr);
        if let Err(e) = socket.send_to(quote.as_bytes(), remote_addr).await {
            eprintln!("Could not send quote to udp://{remote_addr}: {e}");
        }
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
