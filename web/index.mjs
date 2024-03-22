import { randomBytes } from 'crypto'
import { Buffer } from 'buffer'
import { EventEmitter } from 'events'
import { createConnection } from 'net'
import { dirname, join as joinPath } from 'path'
import { fileURLToPath } from 'url'

import cookieParser from 'cookie-parser'
import bodyParser from 'body-parser'
import express from 'express'

const join = (client, { host, port, name, }) => {
    const { events, commands, } = client

    const socket = createConnection({ host, port, }, () => { })
    socket.on('error', err => console.error(err))

    const COMMA_BYTE = 0x2c
    const NEWLINE = 0x0a

    let state = 'enter-name'
    let buffer = Buffer.alloc(0)
    let length = 0

    const onEnterName = () => {
        const index = buffer.findIndex(b => b === NEWLINE)
        const found = index > -1

        if (found) {
            socket.write(`|${name}\n`)
        }

        buffer = found ? Buffer.alloc(0) : buffer.subarray(index + 1)
        return found ? 'length' : 'enter-name'
    }

    const onLength = () => {
        let i = 0

        for (; i < buffer.length; ++i) {
            const byte = buffer[i]

            if (0x30 <= byte && byte <= 0x39) {
                length = (10 * length) + (byte - 0x30)
                continue
            }

            if (byte == COMMA_BYTE) {
                break
            }

            throw new Error(`unexpected input: "${byte}"`)
        }

        const done = i < buffer.length
        buffer = done ? buffer.subarray(i + 1) : Buffer.alloc(0)
        return done ? 'data' : 'length'
    }

    const onData = () => {
        if (buffer.length < length) {
            return 'data'
        }

        const bytes = buffer.subarray(0, length)
        buffer = buffer.subarray(length)
        length = 0

        const text = bytes.toString('utf-8')
        const message = JSON.parse(text)
        events.emit('server', { name, message, })

        return 'length'
    }

    socket.on('data', data => {
        buffer = Buffer.concat([buffer, data], buffer.length + data.length)
        let oldState = state

        do {
            oldState = state
            let newState = null

            switch (state) {
                case 'enter-name': newState = onEnterName(); break;
                case 'length': newState = onLength(); break
                case 'data': newState = onData(); break
                default: socket.end()
            }

            state = newState
        } while (state !== oldState)
    })

    const onCommand = text => {
        socket.write(Buffer.from(text, 'utf-8'))
    }

    socket.on('end', () => {
        commands.off('command', onCommand)
    })

    commands.on('command', onCommand)
}

const publicDir = joinPath(dirname(fileURLToPath(import.meta.url)), 'public')
const index = joinPath(publicDir, 'index.html')

const app = express()

app.use(cookieParser())
app.use(bodyParser.json())

const clientsById = new Map() // TODO: Still leaks if any request has been made, but event stream never conneccted.

app.use((req, res, next) => {
    const cookieName = 'player_id'
    let playerId = req.cookies[cookieName]

    if (playerId === undefined) {
        playerId = randomBytes(64).toString('base64url')
        res.cookie(cookieName, playerId, { httpOnly: true, })
    }

    let client = clientsById.get(playerId)

    if (client === undefined) {
        const events = new EventEmitter()
        const commands = new EventEmitter()

        client = { events, commands, }
        clientsById.set(playerId, client)
    }

    req.playerId = playerId
    req.playerClient = client

    return next()
})

app.get('/', (_req, res) => res.sendFile(index))
app.use(express.static(publicDir))

const OK = 200

app.get('/api/events', async (req, res) => {
    res.set({
        'Cache-Control': 'no-cache',
        'Content-Type': 'text/event-stream',
        'Connection': 'keep-alive'
    })

    res.flushHeaders()
    res.write('retry: 10000\n\n')

    const listener = event => {
        const message = JSON.stringify(event.message)
        res.write(`data: ${message}\n\n`)
    }

    const { events, } = req.playerClient
    events.on('server', listener)

    res.on('close', () => {
        events.off('server', listener)
        clientsById.delete(req.playerId)
        res.end()
    })
})

app.post('/api/join', (req, res) => {
    const client = req.playerClient
    join(client, { host: '127.0.0.1', port: 9999, name: req.body.name, })
    res.sendStatus(OK)
})

app.post('/api/guess', (req, res) => {
    const client = req.playerClient
    client.commands.emit('command', `${req.body.value}\n`)
    res.sendStatus(OK)
})

app.post('/api/play', (req, res) => {
    const client = req.playerClient
    client.commands.emit('command', `${req.body.index}\n`)
    res.sendStatus(OK)
})

app.listen(3000)
