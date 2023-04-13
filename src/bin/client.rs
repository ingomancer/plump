use std::{
    env::args,
    io::Error as IoError,
    net::{AddrParseError, IpAddr},
    result::Result as StdResult,
    str::FromStr,
};

use tokio::{
    io::{copy, stdin, stdout},
    net::TcpStream,
    task::JoinError,
};

#[derive(Debug)]
enum Error {
    Connect(IoError),
    Copy(IoError),
    Join(JoinError),
    NoIpAdressGiven,
    ParseAddress(AddrParseError),
}

type Result<T> = StdResult<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    const PORT: u16 = 9999;

    let address = match args().nth(1) {
        Some(text) => text,
        None => return Err(Error::NoIpAdressGiven),
    };

    let ip_addr = IpAddr::from_str(address.trim()).map_err(Error::ParseAddress)?;
    let socket = TcpStream::connect((ip_addr, PORT))
        .await
        .map_err(Error::Connect)?;

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

    Ok(())
}
