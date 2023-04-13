use std::{
    io::{stdin, stdout, Error as IoError, ErrorKind, Read, Result as IoResult, Write},
    net::TcpStream,
};

pub fn input(prompt: &str) -> IoResult<String> {
    print!("{}", prompt);
    stdout().flush()?;

    let mut result = String::new();
    stdin().read_line(&mut result)?;

    Ok(result)
}

fn send_to_remote(socket: &mut TcpStream, text: String) -> IoResult<()> {
    let mut data = text.into_bytes();
    while !data.is_empty() {
        let sent = socket.write(&data)?;
        data.drain(0..sent);
    }

    Ok(())
}

fn readline_from_remote(socket: &mut TcpStream) -> IoResult<String> {
    const NEWLINE: u8 = 0xA;
    let mut all = Vec::<u8>::new();
    loop {
        let mut buffer = [0u8; 1024];
        let received = socket.read(&mut buffer)?;
        all.extend_from_slice(&buffer[0..received]);

        if all.last() != Some(&NEWLINE) {
            continue;
        }

        return String::from_utf8(all).map_err(|_| IoError::new(ErrorKind::Other, "invalid UTF-8"));
    }
}

pub(crate) enum Client {
    Local,
    Remote(TcpStream),
}

impl Client {
    pub(crate) fn send(&mut self, prompt: &str) -> IoResult<()> {
        match self {
            Client::Remote(socket) => send_to_remote(socket, prompt.into()),
            Client::Local => {
                print!("{}", prompt);
                Ok(())
            }
        }
    }

    pub(crate) fn readline(&mut self) -> IoResult<String> {
        let text = match self {
            Client::Local => input(""),
            Client::Remote(socket) => readline_from_remote(socket),
        }?;

        Ok(text.trim().to_owned())
    }

    pub(crate) fn readline_with_prompt(&mut self, prompt: &str) -> IoResult<String> {
        self.send(prompt)?;
        self.readline()
    }

    pub(crate) fn get_player_name(&mut self) -> IoResult<String> {
        self.readline_with_prompt("Please input player name: ")
    }
}
