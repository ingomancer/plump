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
    while data.len() > 0 {
        let sent = socket.write(&data)?;
        data.drain(0..sent);
    }

    Ok(())
}

pub fn send(socket: &mut Option<TcpStream>, prompt: &str) -> IoResult<()> {
    if let Some(socket) = socket {
        send_to_remote(socket, prompt.into())
    } else {
        print!("{}", prompt);
        Ok(())
    }
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

fn readline(socket: &mut Option<TcpStream>) -> IoResult<String> {
    let text = match socket {
        Some(socket) => readline_from_remote(socket),
        None => input(""),
    }?;

    Ok(text.trim().to_owned())
}

pub fn readline_with_prompt(socket: &mut Option<TcpStream>, prompt: &str) -> IoResult<String> {
    send(socket, prompt)?;
    readline(socket)
}
