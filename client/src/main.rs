use std::{
    collections::HashSet,
    io::{Error as IoError, ErrorKind, Result as IoResult},
    net::IpAddr,
    result::Result as StdResult,
};

#[cfg(unix)]
use std::os::unix::thread;

use clap::Parser;
use interprocess::local_socket::{
    prelude::*, tokio::Stream, traits::tokio::Stream as StreamTrait, GenericNamespaced,
};
use playing_cards::structs::Card;
use protocol::message::Message;
use rand::{distributions::Alphanumeric, seq::IteratorRandom, thread_rng, Rng};
use tokio::{
    io::{copy, stdin, stdout, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    task::JoinError,
};

#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    Connect(IoError),
    Copy(IoError),
    Join(JoinError),
}

type Result<T> = StdResult<T, Error>;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    address: IpAddr,
    #[arg(long, default_value = "9999")]
    port: u16,
    #[arg(long, default_value = "false")]
    ai: bool,
    #[arg(long, default_value = "false")]
    local_only: bool,
}

enum SocketKind {
    Local(Stream),
    Remote(TcpStream),
}

impl SocketKind {
    async fn read(&mut self, buffer: &mut [u8; 1024]) -> StdResult<usize, IoError> {
        match self {
            SocketKind::Local(stream) => stream.read(buffer).await,
            SocketKind::Remote(tcp_stream) => tcp_stream.read(buffer).await,
        }
    }

    async fn write(&mut self, data: &[u8]) -> StdResult<usize, IoError> {
        match self {
            SocketKind::Local(ref mut stream) => stream.write(data).await,
            SocketKind::Remote(ref mut tcp_stream) => tcp_stream.write(data).await,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut socket = if args.local_only {
        let name = "plump.socket"
            .to_ns_name::<GenericNamespaced>()
            .expect("Could not find socket. Is the server running?");
        SocketKind::Local(Stream::connect(name).await.expect("I am error"))
    } else {
        let socket = TcpStream::connect((args.address, args.port))
            .await
            .map_err(Error::Connect)?;
        socket.set_nodelay(true).expect("set_nodelay failed");
        SocketKind::Remote(socket)
    };
    if args.ai {
        let name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .map(|c| c.to_uppercase().to_string())
            .collect::<String>();

        loop {
            let server_message = readline_from_remote(&mut socket).await.unwrap();

            for line in server_message.lines() {
                if let Ok(message) = serde_json::from_str::<Message>(line) {
                    match message {
                        Message::RequestGuessContext {
                            player: _,
                            hand,
                            guesses,
                            players,
                        } => {
                            let guess = make_guess(players, hand, guesses);
                            send_to_remote(&mut socket, guess.to_string() + "\n")
                                .await
                                .unwrap();
                        }
                        Message::PlayRequestContext {
                            player: _,
                            hand,
                            trick: _,
                            valid_cards,
                        } => {
                            let play = make_play(hand, valid_cards);
                            send_to_remote(&mut socket, play.to_string() + "\n")
                                .await
                                .unwrap();
                        }
                        Message::RequestPlayerName => {
                            send_to_remote(&mut socket, name.to_string() + "\n")
                                .await
                                .unwrap();
                        }
                        Message::Winners {
                            players: _,
                            winner_indices: _,
                        } => {
                            if !args.ai {
                                println!("{}", message);
                            }
                        }
                        Message::GameOver => {
                            return Ok(());
                        }
                        _ => send_to_remote(
                            &mut socket,
                            format!("{}\n", (0..10).choose(&mut thread_rng()).unwrap()),
                        )
                        .await
                        .unwrap(),
                    }
                }
            }
        }
    } else {
        match socket {
            SocketKind::Local(stream) => {
                let (mut reader, mut writer) = stream.split();

                let read_handle = tokio::spawn(async move {
                    let mut stdout = stdout();
                    copy(&mut reader, &mut stdout).await.map_err(Error::Copy)
                });

                let write_handle = tokio::spawn(async move {
                    let mut stdin = stdin();
                    copy(&mut stdin, &mut writer).await.map_err(Error::Copy)
                });

                read_handle.await.map_err(Error::Join)??;
                write_handle.await.map_err(Error::Join)??;
            }
            SocketKind::Remote(tcp_stream) => {
                let (mut reader, mut writer) = tcp_stream.into_split();

                let read_handle = tokio::spawn(async move {
                    let mut stdout = stdout();
                    copy(&mut reader, &mut stdout).await.map_err(Error::Copy)
                });

                let write_handle = tokio::spawn(async move {
                    let mut stdin = stdin();
                    copy(&mut stdin, &mut writer).await.map_err(Error::Copy)
                });

                read_handle.await.map_err(Error::Join)??;
                write_handle.await.map_err(Error::Join)??;
            }
        }
    }

    Ok(())
}

fn make_play(hand: Vec<Card>, valid_cards: Option<HashSet<usize>>) -> usize {
    valid_cards.map_or_else(
        || rand::thread_rng().gen_range(0..hand.len()),
        |choices| *choices.iter().next().unwrap(),
    )
}

fn make_guess(players: usize, hand: Vec<Card>, guesses: Vec<usize>) -> usize {
    let guess = hand.iter().filter(|x| x.value >= 10).count();
    if guesses.len() == players - 1 && guess + guesses.iter().sum::<usize>() == hand.len() {
        return if guess != 0 { guess - 1 } else { guess + 1 };
    }

    guess
}

async fn send_to_remote(socket: &mut SocketKind, text: String) -> IoResult<()> {
    let mut data = text.into_bytes();
    while !data.is_empty() {
        let sent = socket.write(&data).await?;
        if sent == 0 {
            return Err(IoError::new(
                ErrorKind::WriteZero,
                "Remote socket was closed",
            ));
        }
        data.drain(0..sent);
    }

    Ok(())
}

async fn readline_from_remote(socket: &mut SocketKind) -> IoResult<String> {
    const NEWLINE: u8 = 0xA;
    let mut all = Vec::<u8>::new();
    loop {
        let mut buffer = [0_u8; 1024];
        let received = socket.read(&mut buffer).await?;
        if received == 0 {
            return Err(IoError::new(
                ErrorKind::UnexpectedEof,
                "Remote socket has been closed",
            ));
        }
        all.extend_from_slice(&buffer[0..received]);

        if all.last() != Some(&NEWLINE) {
            continue;
        }

        return String::from_utf8(all).map_err(|_| IoError::other("invalid UTF-8"));
    }
}
