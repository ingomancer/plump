mod format;
mod game;
mod message;
mod network;

use std::{
    collections::HashMap,
    env::args,
    io::Result as IoResult,
    net::{Shutdown, SocketAddr, TcpListener},
    str::FromStr,
};

use game::PlayerName;
use itertools::Itertools;
use rand::seq::SliceRandom;

use crate::{
    game::{create_players, game, Communicator, Player},
    message::Message,
    network::input,
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
            mode = mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;
            unsafe { SetConsoleMode(handle, mode) }.expect("Failed to set console mode")
        });
    }
}

struct CommunicatorImpl {
    sockets: HashMap<String, network::Client>,
}

impl Communicator for CommunicatorImpl {
    fn read(&mut self, name: PlayerName, prompt: &str) -> String {
        let client = self.sockets.get_mut(name.as_str()).unwrap();
        client.readline_with_prompt(prompt).expect("Failed to read")
    }

    fn write_to_all(&mut self, message: Message) {
        let line = message.to_string() + "\n";

        for client in self.sockets.values_mut() {
            client.send(&line).expect("Failed to write")
        }
    }

    fn write_to_one(&mut self, name: PlayerName, message: Message) {
        let line = message.to_string() + "\n";
        let client = &mut self.sockets.get_mut(name.as_str()).unwrap();
        client.send(&line).expect("Failed to write")
    }
}

impl Drop for CommunicatorImpl {
    fn drop(&mut self) {
        for socket in self.sockets.values() {
            match socket {
                network::Client::Local => {}
                network::Client::Remote(socket) => _ = socket.shutdown(Shutdown::Both),
            };
        }
    }
}

fn main() -> IoResult<()> {
    const PORT: u16 = 9999;

    #[cfg(windows)]
    enable_colors();

    let num_players = match args().nth(1).map(|s| s.parse()) {
        Some(Ok(value)) => value,
        _ => read_u32("Number of players: "),
    };

    let num_rounds = match num_players < 6 {
        true => 10,
        false => 52 / num_players,
    };

    let mut player_names_and_types: Vec<(String, bool)> = (0..(num_players.max(1) - 1))
        .map(|_| (get_random_name(), false))
        .collect();

    let mut client_sockets = HashMap::<String, network::Client>::new();
    let address = SocketAddr::from_str(&format!("0.0.0.0:{PORT}")).expect("Unknown socket address");

    {
        let listener = TcpListener::bind(address).expect("Failed to create listener socket");
        let mut local_client = network::Client::Local;

        let name = local_client.get_player_name()?;
        client_sockets.insert(name.clone(), local_client);
        player_names_and_types.push((name, true));

        for i in 0..(num_players.max(1) - 1) {
            let mut remote_client = match listener.incoming().next().unwrap() {
                Ok(stream) => network::Client::Remote(stream),
                Err(_) => continue,
            };

            let name = remote_client.get_player_name()?;
            client_sockets.insert(name.clone(), remote_client);
            player_names_and_types[i as usize] = (name, true);
        }
    }

    let mut communicator = CommunicatorImpl {
        sockets: client_sockets,
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

fn get_random_name() -> String {
    let choices: Vec<char> = "0123456789abcdef".chars().collect();
    let chars: String = (0..7)
        .map(|_| choices.choose(&mut rand::thread_rng()).unwrap())
        .collect();

    let prefix = &chars[0..3];
    let suffix = &chars[3..];
    format!("{prefix}-{suffix}").to_uppercase()
}

fn read_u32(prompt: &str) -> u32 {
    loop {
        let text = input(prompt).expect("Failed to read number");

        if let Ok(value) = text.trim().parse() {
            return value;
        }
    }
}
