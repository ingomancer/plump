mod format;
mod game;
mod message;
mod network;

use std::{
    collections::HashMap,
    io::Result as IoResult,
    net::{Shutdown, SocketAddr, TcpListener},
    str::FromStr,
    time::Duration,
};

use clap::Parser;
use game::PlayerName;
use itertools::Itertools;

use crate::{
    game::{create_players, game, Communicator, Player},
    message::Message,
};

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

struct CommunicatorImpl {
    sockets: HashMap<String, network::Client>,
    listener: TcpListener,
}

impl Communicator for CommunicatorImpl {
    fn read(&mut self, name: PlayerName, prompt: Message) -> String {
        loop {
            {
                let client = self.sockets.get_mut(name.as_str()).unwrap();
                if let Ok(res) = client.readline_with_prompt(prompt) {
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
                    if client.send(message).is_ok() {
                        break;
                    }
                }
                self.wait_for_reconnect(name.as_str());
            }
        }
    }

    fn write_to_one(&mut self, name: PlayerName, message: Message) {
        loop {
            {
                let client = self.sockets.get_mut(name.as_str()).unwrap();
                if client.send(message).is_ok() {
                    break;
                }
            }
            self.wait_for_reconnect(name.as_str());
        }
    }

    fn wait_for_reconnect(&mut self, player: &str) {
        let remote_client;
        println!("Player {player} has disconnected, waiting for rejoin");
        loop {
            remote_client = match self.listener.incoming().next().unwrap() {
                Ok(stream) => network::Client::RemoteText(stream),
                Err(_) => continue,
            };
            break;
        }
        if let Some(client) = self.sockets.get_mut(player) {
            *client = remote_client;
        }
    }
}

impl Drop for CommunicatorImpl {
    fn drop(&mut self) {
        for socket in self.sockets.values() {
            match socket {
                network::Client::Local => {}
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
    #[arg(long, conflicts_with = "local_robot")]
    local_player: bool,
    #[arg(long)]
    local_robot: bool,
    #[arg(long, default_value = "9999")]
    port: u16,
}

fn main() -> IoResult<()> {
    let args = Args::parse();

    #[cfg(windows)]
    enable_colors();

    let num_players = args.players;

    let num_rounds = if num_players < 6 {
        10
    } else {
        52 / num_players
    };

    let mut player_names_and_types: Vec<(String, bool)> = vec![];

    let mut client_sockets = HashMap::<String, network::Client>::new();
    let address =
        SocketAddr::from_str(&format!("0.0.0.0:{}", args.port)).expect("Unknown socket address");
    let listener = TcpListener::bind(address).expect("Failed to create listener socket");

    let remote_players;
    if args.local_player || args.local_robot {
        remote_players = num_players - 1;

        let mut local_client = network::Client::Local;
        let name = local_client.get_player_name()?;
        client_sockets.insert(name.clone(), local_client);
        player_names_and_types.push((name, args.local_player));
    } else {
        remote_players = num_players;
    }

    for _ in 0..(remote_players) {
        let mut remote_client = match listener.incoming().next().unwrap() {
            Ok(stream) => {
                stream.set_read_timeout(Some(Duration::from_secs(5)))?;
                stream.set_write_timeout(Some(Duration::from_secs(10)))?;
                network::Client::RemoteText(stream)
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

    let mut communicator = CommunicatorImpl {
        sockets: client_sockets,
        listener,
    };

    let mut players = create_players(&player_names_and_types);
    let winners = game(&mut communicator, &mut players, num_rounds);

    let players_vec: Vec<Player> = players.into_iter().collect_vec();
    communicator.write_to_all(Message::Winners {
        players: &players_vec,
        winner_indices: &winners,
    });

    Ok(())
}
