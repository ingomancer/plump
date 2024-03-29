use std::{
    io::{Error as IoError, ErrorKind, Read, Result as IoResult, Write},
    net::TcpStream,
};

use protocol::message::Message;

fn send_to_remote(socket: &mut TcpStream, text: String) -> IoResult<()> {
    let mut data = text.into_bytes();
    while !data.is_empty() {
        let sent = socket.write(&data)?;
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

fn readline_from_remote(socket: &mut TcpStream) -> IoResult<String> {
    const NEWLINE: u8 = 0xA;
    let mut all = Vec::<u8>::new();
    loop {
        let mut buffer = [0_u8; 1024];
        let received = socket.read(&mut buffer)?;
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

pub(crate) enum Client {
    RemoteText(TcpStream),
    RemoteJson(TcpStream),
}

impl Client {
    pub(crate) fn send(&mut self, msg: Message) -> IoResult<()> {
        match self {
            Client::RemoteText(socket) => send_to_remote(socket, msg.to_string() + "\n"),
            Client::RemoteJson(socket) => {
                let line = serde_json::to_string(&msg).unwrap();
                let line = format!("{}\n", line);
                send_to_remote(socket, line)
            }
        }
    }

    pub(crate) fn readline(&mut self) -> IoResult<String> {
        let text = match self {
            Client::RemoteText(socket) | Client::RemoteJson(socket) => readline_from_remote(socket),
        }?;

        Ok(text.trim().to_owned())
    }

    pub(crate) fn readline_with_prompt(&mut self, prompt: Message) -> IoResult<String> {
        self.send(prompt)?;
        self.readline()
    }

    pub(crate) fn get_player_name(&mut self) -> IoResult<String> {
        self.readline_with_prompt(Message::RequestPlayerName)
    }

    pub(crate) fn into_remote_json(self) -> Option<Client> {
        match self {
            Client::RemoteText(socket) => Some(Client::RemoteJson(socket)),
            Client::RemoteJson(_) => Some(self),
        }
    }
}
