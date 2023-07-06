use std::{io::Error as IoError, net::IpAddr, result::Result as StdResult};

use clap::Parser;
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
}

type Result<T> = StdResult<T, Error>;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    address: IpAddr,
    #[arg(long, default_value = "9999")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let socket = TcpStream::connect((args.address, args.port))
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
