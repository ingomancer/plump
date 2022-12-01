from collections import deque, namedtuple
import itertools
import math
from random import sample
from secrets import choice
import time

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


def request_guess(hand, prev_guesses, player_count):
    print(f"Hand: {hand}, Previous Guesses: {prev_guesses}, Players: {player_count}")
    guess = -1
    while not validate_guess(len(hand), prev_guesses, player_count, guess):
        try:
            guess = int(input("Please provide a guess: "))
        except ValueError:
            continue
    return guess


def determine_start_player(guesses):
    return guesses.index(max(guesses))


def determine_winner(trick):
    return trick.index(max(trick))


def play_card(hand, trick):
    card = choice(list(hand))
    return hand - set([card]), trick + [card]


def format_trick(trick):
    pass


def format_hand(hand, valid_cards):
    pass


def playable_card_indices(hand, trick):
    if trick:
        playable_cards = [
            index for index, card in enumerate(hand) if card[0] == trick[0][0]
        ]
        if playable_cards:
            return playable_cards
    return [0] * len(hand)


def play_human_card(hand, trick):
    hand = list(hand)
    trick_string = format_trick(trick)
    valid_cards = playable_card_indices(hand, trick)
    hand_string = format_hand(hand, trick)
    print(f"Hand: {hand_string}, Trick: {trick_string}")
    card_index = -1
    while card_index < 0:
        try:
            card_index = int(input("Select card to play (leftmost is 0): "))
        except ValueError:
            pass
        try:
            card = hand[card_index]
        except IndexError:
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


def game(players: "list[str]"):
    players = create_players(players)
    sets = list(range(10, 1, -1)) + [1] * len(players) + list(range(2, 11))

    for set in sets:
        players_in_set = players.copy()
        deck = create_deck()
        prev_guesses = []
        for player in players_in_set:
            deck, hand = draw_hand(deck, set)
            print(f"{player.name} is thinking...")
            if player.human:
                player.state = player.state._replace(
                    guess=request_guess(hand, prev_guesses, len(players_in_set)),
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
                    hand, trick = play_human_card(player.state.hand, trick)
                else:
                    hand, trick = play_card(player.state.hand, trick)  # TODO: Humans?
                player.state = player.state._replace(hand=hand)
            index = determine_winner(trick)
            players_in_set[index].state = player.state._replace(
                wins=player.state.wins + 1
            )
            players_in_set.rotate(-index)
        for player in players_in_set:
            player.state = score_round(player.state)
        players.rotate(-1)
    return determine_total_winners(players)


def score_round(state):
    if state.guess == state.wins:
        score = state.score + state.score + max(10, 10 * state.guess)
    else:
        score = state.score
    state = state._replace(score=score, wins=0)
    return state


if __name__ == "__main__":
    players = [("Ingo", True), ("Klara", True)]
    winners = game(players)
    print(f"The winner(s) is/are {','.join(players[winner][0] for winner in winners)}!")
