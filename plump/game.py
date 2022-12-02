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


def request_guess(read, write, prompt, hand, prev_guesses, player_count):
    hand_string = format_hand(sorted(hand), valid_cards=None)
    write(
        f"{prompt}Hand: {hand_string}, Previous Guesses: {prev_guesses}, Players: {player_count}"
    )
    guess = -1
    while not validate_guess(len(hand), prev_guesses, player_count, guess):
        try:
            guess = int(read(f"{prompt}Please provide a guess: "))
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


def play_human_card(read, write, prompt, hand, trick):
    hand = sorted(list(hand))
    trick_string = format_trick(trick)
    valid_cards = playable_card_indices(hand, trick)
    hand_string = format_hand(hand, valid_cards, with_indices=True)
    write(
        f"{prompt}Hand: {hand_string}, {'Trick: ' + trick_string if trick_string else 'You go first!'}"
    )
    card_index = -1
    while card_index < 0:
        try:
            card_index = int(read(f"{prompt}Select card to play (leftmost is 0): "))
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
            write(f"{player.name} is thinking...")
            if player.human:
                player.state = player.state._replace(
                    guess=request_guess(
                        read,
                        write,
                        f"{player.name}: ",
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
        index = determine_start_player(prev_guesses)
        players_in_set.rotate(-index)

        while len(players_in_set[0].state.hand) > 0:
            trick = []
            for player in players_in_set:
                if player.human:
                    hand, trick = play_human_card(
                        read, write, f"{player.name}: ", player.state.hand, trick
                    )
                else:
                    hand, trick = play_card(player.state.hand, trick)  # TODO: Humans?
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


def random_name():
    name = "".join(choice("0123456789abcdef") for n in range(7))
    return f"{name[0:3]}-{name[3:]}".upper()


def main(args):
    port = 9999
    num_players = int(args[0])
    num_rounds = 10
    # players = [("Ingo", True), ("Klara", True)]
    players = [(random_name(), False) for _ in range(num_players)]

    with ExitStack() as stack:
        server_socket = stack.enter_context(socket(AF_INET, SOCK_STREAM))
        server_socket.bind(("", port))
        server_socket.listen()
        client_sockets = []

        for i in range(num_players):
            client_socket = stack.enter_context(server_socket.accept()[0])
            client_sockets.append(client_socket)
            players[i] = (random_name(), True)  # TODO: Ask name.
            # TODO: Allow the host to start the game here without waiting for more players.

        server_socket.close()  # stop accepting

        # TODO: Handle socket exceptions.

        def send_all(text):
            data = text.encode("utf-8")
            for socket in client_sockets:
                socket.send(data)

        def read_from(socket, prompt):
            socket.send(prompt.encode("utf-8"))
            return socket.recv(1024)  # TODO: Handle eof. Read until end of line.

        read = lambda prompt: read_from(client_sockets[0], prompt)
        write = lambda text: send_all(f"{text}\n")

        winners = game(read, write, players, num_rounds)
        write(
            f"The winner(s) is/are {','.join(players[winner][0] for winner in winners)}!"
        )
        client_socket.shutdown(SHUT_RDWR)


if __name__ == "__main__":
    main(argv[1:])
