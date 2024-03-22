use std::{
    collections::HashSet,
    io::{Error as IoError, ErrorKind, Result as IoResult},
    net::IpAddr,
    result::Result as StdResult,
};

use clap::Parser;
use playing_cards::structs::Card;
use protocol::message::Message;
use rand::distributions::Alphanumeric;
use rand::Rng;
use tokio::{
    io::{copy, stdin, stdout, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    task::JoinError,
};

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
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut socket = TcpStream::connect((args.address, args.port))
        .await
        .map_err(Error::Connect)?;
    socket.set_nodelay(true).expect("set_nodelay failed");

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
                let message: Message = serde_json::from_str(line).unwrap();

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
                        println!("{}", message.to_string());
                    }
                    Message::GameOver => {
                        return Ok(());
                    }
                    _ => (),
                }
            }
        }
    } else {
        let (mut reader, mut writer) = socket.into_split();

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

async fn send_to_remote(socket: &mut TcpStream, text: String) -> IoResult<()> {
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

async fn readline_from_remote(socket: &mut TcpStream) -> IoResult<String> {
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

        return String::from_utf8(all).map_err(|_| IoError::new(ErrorKind::Other, "invalid UTF-8"));
    }
}
