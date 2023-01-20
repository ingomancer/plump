use std::{
    collections::HashMap,
    env::args,
    io::Result as IoResult,
    net::{Shutdown, SocketAddr, TcpListener, TcpStream},
    str::FromStr,
};

use game::{create_players, game, Communicator};
use itertools::Itertools;
use network::{input, readline_with_prompt};
use rand::seq::SliceRandom;

use crate::network::send;

mod format;
mod game;
mod network;

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
    sockets: HashMap<String, Option<TcpStream>>,
}

impl Communicator for CommunicatorImpl {
    fn write(&mut self, text: &str, name: Option<&str>) {
        let line = text.to_owned() + "\n";

        if let Some(name) = name {
            send(self.sockets.get_mut(name).unwrap(), &line).expect("Failed to write")
        } else {
            for client_socket in self.sockets.values_mut() {
                send(client_socket, &line).expect("Failed to write")
            }
        }
    }

    fn read(&mut self, prompt: &str, name: &str) -> String {
        readline_with_prompt(self.sockets.get_mut(name).unwrap(), prompt).expect("Failed to read")
    }
}

impl Drop for CommunicatorImpl {
    fn drop(&mut self) {
        for (_, socket) in &self.sockets {
            if let Some(socket) = socket {
                _ = socket.shutdown(Shutdown::Both);
            }
        }
    }
}

fn main() -> IoResult<()> {
    const PORT: u16 = 9999;

    if cfg!(windows) {
        enable_colors()
    }

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

    let mut client_sockets = HashMap::<String, Option<TcpStream>>::new();
    let address = SocketAddr::from_str(&format!("0.0.0.0:{PORT}")).expect("Unknown socket address");

    {
        let listener = TcpListener::bind(address).expect("Failed to create listener socket");

        let name = get_player_name(&mut None)?;
        client_sockets.insert(name.clone(), None);
        player_names_and_types.push((name, true));

        for i in 0..(num_players.max(1) - 1) {
            let mut client_socket = match listener.incoming().next().unwrap() {
                Ok(stream) => Some(stream),
                Err(_) => continue,
            };

            let name = get_player_name(&mut client_socket)?;
            client_sockets.insert(name.clone(), client_socket);
            player_names_and_types[i as usize] = (name, true);
        }
    }

    let mut communicator = CommunicatorImpl {
        sockets: client_sockets,
    };

    let mut players = create_players(&player_names_and_types);
    let winners = game(&mut communicator, &mut players, num_rounds);

    let winners_text = winners.into_iter().map(|i| players[i].name).join(", ");
    communicator.write(&format!("The winner(s) is/are {winners_text}!"), None);

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

fn get_player_name(socket: &mut Option<TcpStream>) -> IoResult<String> {
    readline_with_prompt(socket, "Please input player name: ")
}

fn read_u32(prompt: &str) -> u32 {
    loop {
        let text = input(prompt).expect("Failed to read number");

        match text.trim().parse() {
            Ok(value) => return value,
            Err(_) => {}
        }
    }
}
