mod game;
mod network;

use futures::future::join_all;

use std::{
    collections::HashMap,
    io::Result as IoResult,
    net::{Shutdown, SocketAddr, TcpListener, TcpStream},
    str::FromStr,
    sync::mpsc::{channel, Receiver, Sender},
};

use clap::Parser;
use game::{create_players, game, Communicator};
use protocol::{message::Message, structs::PlayerName};

#[cfg(windows)]
fn enable_colors() {
    use windows::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, SetConsoleMode, SetConsoleOutputCP, CONSOLE_MODE,
        ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_ERROR_HANDLE, STD_OUTPUT_HANDLE,
    };

    const UTF8_CODEPAGE: u32 = 65001;
    unsafe { SetConsoleOutputCP(UTF8_CODEPAGE) }.expect("Failed to set output codepage");

    for name in [STD_OUTPUT_HANDLE, STD_ERROR_HANDLE] {
        let handle = unsafe { GetStdHandle(name) }.expect("Failed to get console handle");
        let mut mode: CONSOLE_MODE = Default::default();

        let result = unsafe { GetConsoleMode(handle, &mut mode) };

        result.as_bool().then(|| {
            mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
            unsafe { SetConsoleMode(handle, mode) }.expect("Failed to set console mode")
        });
    }
}

struct ReconnectRequest {
    player: String,
    return_channel: Sender<TcpStream>,
}

struct CommunicatorImpl {
    sockets: HashMap<String, network::Client>,
    reconnect: Sender<ReconnectRequest>,
    return_sender: Sender<TcpStream>,
    return_receiver: Receiver<TcpStream>,
}

impl Communicator for CommunicatorImpl {
    fn read(&mut self, name: &PlayerName, prompt: Message) -> String {
        loop {
            {
                let client = self.sockets.get_mut(name.as_str()).unwrap();
                if let Ok(res) = client.readline_with_prompt(prompt.clone()) {
                    break res;
                }
            }
            self.wait_for_reconnect(name.as_str());
        }
    }

    fn write_to_all(&mut self, message: Message) {
        for name in self.sockets.keys().cloned().collect::<Vec<String>>() {
            loop {
                {
                    let client = self.sockets.get_mut(name.as_str()).unwrap();
                    if client.send(message.clone()).is_ok() {
                        break;
                    }
                }
                self.wait_for_reconnect(name.as_str());
            }
        }
    }

    fn write_to_one(&mut self, name: &PlayerName, message: Message) {
        loop {
            {
                let client = self.sockets.get_mut(name.as_str()).unwrap();
                if client.send(message.clone()).is_ok() {
                    break;
                }
            }
            self.wait_for_reconnect(name.as_str());
        }
    }

    fn wait_for_reconnect(&mut self, player: &str) {
        println!("Player {player} has disconnected, waiting for rejoin");
        self.reconnect
            .send(ReconnectRequest {
                player: player.to_owned(),
                return_channel: self.return_sender.clone(),
            })
            .unwrap();
        let remote_client = self.return_receiver.recv().unwrap();
        if let Some(client) = self.sockets.get_mut(player) {
            match client {
                network::Client::RemoteText(socket) => *socket = remote_client,
                network::Client::RemoteJson(socket) => *socket = remote_client,
            }
        }
    }
}

impl Drop for CommunicatorImpl {
    fn drop(&mut self) {
        for socket in self.sockets.values() {
            match socket {
                network::Client::RemoteText(socket) | network::Client::RemoteJson(socket) => {
                    _ = socket.shutdown(Shutdown::Both);
                }
            };
        }
    }
}

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "4")]
    players: usize,
    #[arg(long, default_value = "9999")]
    port: u16,
    #[arg(long, default_value = "false")]
    ai: bool,
}

#[tokio::main]
async fn main() -> IoResult<()> {
    let args = Args::parse();

    #[cfg(windows)]
    enable_colors();

    let num_players = args.players;

    let num_rounds = if num_players < 6 {
        10
    } else {
        52 / num_players
    };
    let address =
        SocketAddr::from_str(&format!("0.0.0.0:{}", args.port)).expect("Unknown socket address");
    let listener = TcpListener::bind(address).expect("Failed to create listener socket");
    let mut player_names_and_types: Vec<(String, bool)> = vec![];

    let mut client_sockets = HashMap::<String, network::Client>::new();

    for _ in 0..(num_players) {
        let mut remote_client = match listener.incoming().next().unwrap() {
            Ok(stream) => {
                if args.ai {
                    network::Client::RemoteJson(stream)
                } else {
                    network::Client::RemoteText(stream)
                }
            }
            Err(_) => continue,
        };

        let name = remote_client.get_player_name()?;
        if name.starts_with('|') {
            remote_client = remote_client.into_remote_json().unwrap();
        }
        client_sockets.insert(name.clone(), remote_client);
        player_names_and_types.push((name, true));
    }

    let (reconnect_sender, reconnect_receiver) = channel();
    let (return_sender, return_receiver) = channel();
    let mut communicator = CommunicatorImpl {
        sockets: client_sockets,
        reconnect: reconnect_sender,
        return_sender,
        return_receiver,
    };

    let players = create_players(player_names_and_types);
    let running_game =
        tokio::spawn(async move { game(&mut communicator, players, num_rounds, args.ai).await });
    let reconnect_handler = tokio::spawn(async move {
        while let Ok(request) = reconnect_receiver.recv() {
            println!("Reconnecting player {}.", request.player);
            let remote_client = match listener.incoming().next().unwrap() {
                Ok(stream) => stream,
                Err(_) => continue,
            };
            request.return_channel.send(remote_client).unwrap();
        }
    });
    join_all([running_game, reconnect_handler]).await;
    Ok(())
}
