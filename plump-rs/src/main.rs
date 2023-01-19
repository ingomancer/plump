use std::io::{self, stdin, stdout, Write};

use game::{create_players, game};
use rand::seq::SliceRandom;

mod format;
mod game;

fn main() -> io::Result<()> {
    let player_count: u32;
    loop {
        let mut player_count_input = String::new();
        print!("Player count: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut player_count_input)?;
        if let Ok(x) = player_count_input.trim().parse::<u32>() {
            player_count = x;
            break;
        }
    }
    let num_rounds = if player_count < 6 {
        10
    } else {
        52 / player_count
    };
    let player_names_and_types: Vec<(String, bool)> = (0..player_count)
        .map(|x| {
            if x == 0 {
                (get_player_name(), true)
            } else {
                (get_random_name(), false)
            }
        })
        .collect();

    fn write(text: &str, name: Option<&str>) {
        match name {
            Some(name) => println!("{}: {}", name, text),
            None => println!("{}", text),
        }
    }

    fn read(prompt: &str, name: &str) -> String {
        print!("{prompt}{name}: ");
        stdout().flush().unwrap();
        let mut player_input = String::new();
        stdin().read_line(&mut player_input).unwrap();
        player_input
    }

    let players = create_players(&player_names_and_types);
    let winners = game(read, write, players, num_rounds);

    Ok(())
}

fn get_random_name() -> String {
    let choices: Vec<char> = "123456789abcdef".chars().collect();
    (0..7)
        .map(|_| choices.choose(&mut rand::thread_rng()).unwrap())
        .collect()
}

fn get_player_name() -> String {
    print!("Player name: ");
    stdout().flush().unwrap();
    let mut player_name = String::new();
    stdin().read_line(&mut player_name).unwrap();
    player_name.trim().to_owned()
}
