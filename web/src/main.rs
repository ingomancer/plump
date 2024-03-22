use std::{
    io::{ErrorKind, Result},
    time::Duration,
};

use rocket::{
    fs::{relative, FileServer},
    get, launch, post,
    response::stream::{Event, EventStream},
    routes,
    serde::json::Json,
};
use serde::{Deserialize, Serialize};
use tokio::{io::AsyncWriteExt, net::TcpStream};

// import { randomBytes } from 'crypto'
// import { Buffer } from 'buffer'
// import { EventEmitter } from 'events'
// import { createConnection } from 'net'
// import { dirname, join as joinPath } from 'path'
// import { fileURLToPath } from 'url'

// import cookieParser from 'cookie-parser'
// import bodyParser from 'body-parser'
// import express from 'express'

// const app = express()
// app.use(cookieParser())

// const clientsById = new Map() // TODO: Still leaks if any request has been made, but event stream never conneccted.

// app.use((req, res, next) => {
//     const cookieName = 'player_id'
//     let playerId = req.cookies[cookieName]

//     if (playerId === undefined) {
//         playerId = randomBytes(64).toString('base64url')
//         res.cookie(cookieName, playerId, { httpOnly: true, })
//     }

//     let client = clientsById.get(playerId)

//     if (client === undefined) {
//         const events = new EventEmitter()
//         const commands = new EventEmitter()

//         client = { events, commands, }
//         clientsById.set(playerId, client)
//     }

//     req.playerId = playerId
//     req.playerClient = client

//     return next()
// })

#[get("/api/events")]
async fn events() -> EventStream![] {
    EventStream! {
        yield Event::retry(Duration::from_secs(10));
        // yield Event::json(ping);
    }

    //     const listener = event => {
    //         const message = JSON.stringify(event.message)
    //         res.write(`data: ${message}\n\n`)
    //     }

    //     const { events, } = req.playerClient
    //     events.on('server', listener)

    //     res.on('close', () => {
    //         events.off('server', listener)
    //         clientsById.delete(req.playerId)
    //         res.end()
    //     })
}

#[derive(Serialize, Deserialize)]
struct Join {
    name: String,
}

#[post("/api/join", data = "<action>")]
async fn join(action: Json<Join>) -> Result<()> {
    const NEWLINE: u8 = b'\n';

    let Join { mut name } = action.into_inner();
    name.insert(0, '|');
    name.push('\n');

    let mut stream = TcpStream::connect("127.0.0.1:9999").await?;
    let mut buffer = [0u8; 1024];

    enum State {
        Name,
        Length,
    }

    let mut state = State::Name;

    loop {
        stream.readable().await?;

        let data = match stream.try_read(&mut buffer) {
            Ok(0) => return Ok(()),
            Ok(value) => &buffer[0..value],
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e),
        };

        match state {
            State::Name => {
                match data.iter().position(|c| *c == NEWLINE) {
                    Some(_) => {}
                    None => continue,
                };

                // TODO: Read remaining as length.

                stream.writable().await?;
                stream.write_all(name.as_bytes()).await?;

                state = State::Length;
            }

            State::Length => {
                println!("test");
                println!("{:?}", data.len())
            }
        }
    }
    //     const COMMA_BYTE = 0x2c

    //     let buffer = Buffer.alloc(0)
    //     let length = 0

    //     const onLength = () => {
    //         let i = 0

    //         for (; i < buffer.length; ++i) {
    //             const byte = buffer[i]

    //             if (0x30 <= byte && byte <= 0x39) {
    //                 length = (10 * length) + (byte - 0x30)
    //                 continue
    //             }

    //             if (byte == COMMA_BYTE) {
    //                 break
    //             }

    //             throw new Error(`unexpected input: "${byte}"`)
    //         }

    //         const done = i < buffer.length
    //         buffer = done ? buffer.subarray(i + 1) : Buffer.alloc(0)
    //         return done ? 'data' : 'length'
    //     }

    //     const onData = () => {
    //         if (buffer.length < length) {
    //             return 'data'
    //         }

    //         const bytes = buffer.subarray(0, length)
    //         buffer = buffer.subarray(length)
    //         length = 0

    //         const text = bytes.toString('utf-8')
    //         const message = JSON.parse(text)
    //         events.emit('server', { name, message, })

    //         return 'length'
    //     }

    //     socket.on('data', data => {
    //         buffer = Buffer.concat([buffer, data], buffer.length + data.length)
    //         let oldState = state

    //         do {
    //             oldState = state
    //             let newState = null

    //             switch (state) {
    //                 case 'enter-name': newState = onEnterName(); break;
    //                 case 'length': newState = onLength(); break
    //                 case 'data': newState = onData(); break
    //                 default: socket.end()
    //             }

    //             state = newState
    //         } while (state !== oldState)
    //     })

    //     const onCommand = text => {
    //         socket.write(Buffer.from(text, 'utf-8'))
    //     }

    //     socket.on('end', () => {
    //         commands.off('command', onCommand)
    //     })

    //     commands.on('command', onCommand)
}

#[derive(Serialize, Deserialize)]
struct Guess {
    value: usize,
}

#[post("/api/guess", data = "<action>")]
fn guess(action: Json<Guess>) {
    //     const client = req.playerClient
    //     client.commands.emit('command', `${req.body.value}\n`)
}

#[derive(Serialize, Deserialize)]
struct Play {
    index: usize,
}

#[post("/api/play", data = "<action>")]
fn play(action: Json<Play>) {
    //     const client = req.playerClient
    //     client.commands.emit('command', `${req.body.index}\n`)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![events, join, guess, play])
        .mount("/", FileServer::from(relative!("public")))
}
