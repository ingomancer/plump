def send_to_remote(socket, text):
    data = text.encode("utf-8")
    while len(data) > 0:
        sent = socket.send(data)
        data = data[sent:]


def send(socket, text):
    send_to_remote(socket, text) if socket else print(text, end="")


def readline_from_remote(socket):
    all = b""
    while True:
        received = socket.recv(1024)
        all += received
        if received[-1] == b"\n"[0]:
            return all.decode("utf-8")


def readline(socket):
    return (readline_from_remote(socket) if socket else input()).strip()


def readline_with_prompt(socket, prompt):
    send(socket, prompt)
    return readline(socket)
