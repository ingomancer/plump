from collections import deque, namedtuple
from contextlib import ExitStack
import itertools
import math
from random import sample
from secrets import choice
from socket import socket, AF_INET, SHUT_RDWR, SOCK_STREAM
from sys import argv

suit_symbols = ["♥", "♣", "♦", "♠"]
suits = range(4)
cards_symbols = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"]
cards = range(13)


def create_players(player_names):
    players = deque()
    for name, human in player_names:
        players.append(Player(name, human))
    return players


def create_deck():
    return set(itertools.product(suits, cards))


def draw_hand(deck, num):
    hand = set(sample(list(deck), k=num))
    return deck - hand, hand


def make_guess(hand, prev_guesses, player_count):
    guess = len([card for card in hand if card[1] >= 7])
    if not validate_guess(len(hand), prev_guesses, player_count, guess):
        new_guess = len([card for card in hand if card[1] >= 9])
        if new_guess == guess:
            guess += 1
        else:
            guess = new_guess
    return guess


def validate_guess(hand_size, prev_guesses, player_count, guess):
    if not (0 <= guess <= hand_size):
        return False
    if len(prev_guesses) == player_count - 1:
        if (guess + sum(prev_guesses)) == hand_size:
            return False
    return True


def request_guess(read, write, name, hand, prev_guesses, player_count):
    hand_string = format_hand(sorted(hand), valid_cards=None)
    write(
        f"{name}: Hand: {hand_string}, Previous Guesses: {prev_guesses}, Players: {player_count}",
        name,
    )
    guess = -1
    while not validate_guess(len(hand), prev_guesses, player_count, guess):
        try:
            guess = int(read(f"{name}: Please provide a guess: ", name))
        except ValueError:
            continue
    return guess


def determine_start_player(guesses):
    return guesses.index(max(guesses))


def determine_winner(trick):
    first_suit, _ = trick[0]
    valid_cards = list(filter(lambda card: card[0] == first_suit, trick))
    return trick.index(max(valid_cards))


def play_card(hand, trick):
    card = choice(list(hand))
    return hand - set([card]), trick + [card]


def darken(text):
    return f"\033[2m{text}\033[0m"


def format_card(card, darkened=False, index=None):
    suit, value = card
    index_string = "" if index is None else f"{index}|"
    text = f"{index_string}{suit_symbols[suit]}{cards_symbols[value]}"
    return darken(text) if darkened else text


def format_trick(trick):
    if len(trick) > 0:
        return " ".join([format_card(card) for card in trick])
    return None


def format_hand(hand, valid_cards, with_indices=False):
    return " ".join(
        [
            format_card(
                card,
                darkened=(valid_cards and index not in valid_cards),
                index=index if with_indices else None,
            )
            for index, card in enumerate(hand)
        ]
    )


def format_guesses(players):
    return "Guesses: " + ", ".join(
        [f"{player.name}: {player.state.guess}" for player in players]
    )


upside_down_face = "\U0001F643"
slightly_smiling_face = "\U0001F642"


def format_scoreboard(players):
    sorted_players = sorted(players, key=lambda player: player.name)

    def format_state(state):
        did_plump = state.wins != state.guess
        return f"{state.wins}/{state.guess} {upside_down_face if did_plump else slightly_smiling_face} (total: {state.score})"

    return ", ".join(
        [f"{player.name}: {format_state(player.state)}" for player in sorted_players]
    )


def playable_card_indices(hand, trick):
    if trick:
        playable_cards = set(
            index for index, card in enumerate(hand) if card[0] == trick[0][0]
        )
        if playable_cards:
            return playable_cards
    return set()


def play_human_card(read, write, name, hand, trick):
    hand = sorted(list(hand))
    trick_string = format_trick(trick)
    valid_cards = playable_card_indices(hand, trick)
    hand_string = format_hand(hand, valid_cards, with_indices=True)
    write(f"{name}'s turn")
    write(
        f"{name}: Hand: {hand_string}, {'Trick: ' + trick_string if trick_string else 'You go first!'}",
        name,
    )
    card_index = -1
    while card_index < 0:
        try:
            card_index = int(
                read(f"{name}: Select card to play (leftmost is 0): ", name)
            )
        except ValueError:
            pass
        try:
            card = hand[card_index]
        except IndexError:
            card_index = -1
        if valid_cards and card_index not in valid_cards:
            card_index = -1
    hand = set(hand)
    return hand - set([card]), trick + [card]


def determine_total_winners(players):
    winners = []
    highest_score = -math.inf
    for index, player in enumerate(players):
        if player.state.score > highest_score:
            highest_score = player.state.score
            winners = [index]
        elif player.state.score == highest_score:
            winners.append(index)
    return winners


State = namedtuple("state", ["hand", "guess", "wins", "score"])


class Player:
    def __init__(
        self,
        name,
        human,
    ):
        self.name = name
        self.human = human
        self.state = State(hand=[], guess=-1, wins=0, score=0)


def game(read, write, players: "list[str]", num_rounds):
    players = create_players(players)
    sets = (
        list(range(num_rounds, 1, -1))
        + [1] * len(players)
        + list(range(2, num_rounds + 1))
    )

    for set in sets:
        players_in_set = players.copy()
        deck = create_deck()
        prev_guesses = []
        for player in players_in_set:
            deck, hand = draw_hand(deck, set)
            write(f"{player.name}'s turn")
            if player.human:
                player.state = player.state._replace(
                    guess=request_guess(
                        read,
                        write,
                        player.name,
                        hand,
                        prev_guesses,
                        len(players_in_set),
                    ),
                    hand=hand,
                )
            else:
                player.state = player.state._replace(
                    guess=make_guess(hand, prev_guesses, len(players_in_set)), hand=hand
                )
            prev_guesses.append(player.state.guess)
        write(format_guesses(players))
        index = determine_start_player(prev_guesses)
        players_in_set.rotate(-index)

        while len(players_in_set[0].state.hand) > 0:
            trick = []
            for player in players_in_set:
                if player.human:
                    hand, trick = play_human_card(
                        read, write, player.name, player.state.hand, trick
                    )
                else:
                    hand, trick = play_card(player.state.hand, trick)
                player.state = player.state._replace(hand=hand)
            index = determine_winner(trick)
            winner = players_in_set[index]
            winner.state = winner.state._replace(wins=winner.state.wins + 1)
            write(format_scoreboard(players_in_set))
            write(f"{winner.name} won!")
            players_in_set.rotate(-index)
        for player in players_in_set:
            player.state = score_round(player.state)
        players.rotate(-1)
    return determine_total_winners(players)


def score_round(state):
    if state.guess == state.wins:
        score = state.score + max(10, 10 * state.guess)
    else:
        score = state.score
    state = state._replace(score=score, wins=0)
    return state


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


def get_player_name(socket=None):
    return readline_with_prompt(socket, "Please input player name: ")


def get_random_name():
    name = "".join(choice("0123456789abcdef") for n in range(7))
    return f"{name[0:3]}-{name[3:]}".upper()


def main(args):
    port = 9999
    try:
        num_players = int(args[0])
    except IndexError:
        num_players = 4
    num_rounds = 10 if num_players < 6 else 52 // num_players
    players = [(get_random_name(), False) for _ in range(num_players)]
    client_sockets = {}

    with ExitStack() as stack:
        server_socket = stack.enter_context(socket(AF_INET, SOCK_STREAM))
        server_socket.bind(("", port))
        server_socket.listen()

        name = get_player_name()
        players[-1] = (name, True)
        client_sockets[name] = None

        for i in range(num_players - 1):
            client_socket = stack.enter_context(server_socket.accept()[0])
            name = get_player_name(client_socket)
            client_sockets[name] = client_socket
            players[i] = (name, True)

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

        winners = game(read, write, players, num_rounds)
        write(
            f"The winner(s) is/are {','.join(players[winner][0] for winner in winners)}!"
        )
        client_socket.shutdown(SHUT_RDWR)


if __name__ == "__main__":
    main(argv[1:])
