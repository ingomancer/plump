from collections import deque
from contextlib import ExitStack
from secrets import choice
from socket import socket, AF_INET, SHUT_RDWR, SOCK_STREAM
from sys import argv
from plump.network import send, readline_with_prompt
from plump.game import Player, game


def create_players(player_names):
    players = deque()
    for name, human in player_names:
        players.append(Player(name, human))
    return players


def get_player_name(socket=None):
    return readline_with_prompt(socket, "Please input player name: ")


def get_random_name():
    name = "".join(choice("0123456789abcdef") for n in range(7))
    return f"{name[0:3]}-{name[3:]}".upper()


def read_int(prompt):
    while True:
        try:
            return int(input(prompt))
        except ValueError:
            pass


def main(args):
    port = 9999
    try:
        num_players = int(args[0])
    except IndexError:
        num_players = read_int("Number of players: ")
    num_rounds = 10 if num_players < 6 else 52 // num_players
    player_names_and_types = [(get_random_name(), False) for _ in range(num_players)]
    client_sockets = {}

    with ExitStack() as stack:
        server_socket = stack.enter_context(socket(AF_INET, SOCK_STREAM))
        server_socket.bind(("", port))
        server_socket.listen()

        name = get_player_name()
        player_names_and_types[-1] = (name, True)
        client_sockets[name] = None

        for i in range(num_players - 1):
            client_socket = stack.enter_context(server_socket.accept()[0])
            name = get_player_name(client_socket)
            client_sockets[name] = client_socket
            player_names_and_types[i] = (name, True)

        server_socket.close()  # stop accepting

        def write(text, name=None):
            line = text + "\n"
            if name:
                send(client_sockets[name], line)
            else:
                for client_socket in client_sockets.values():
                    send(client_socket, line)

        def read(prompt, name):
            return readline_with_prompt(client_sockets[name], prompt)

        players = create_players(player_names_and_types)
        winners = game(read, write, players, num_rounds)
        write(
            f"The winner(s) is/are {','.join(players[winner].name for winner in winners)}!"
        )
        client_socket.shutdown(SHUT_RDWR)


if __name__ == "__main__":
    main(argv[1:])
