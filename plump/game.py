from collections import deque, namedtuple
import itertools
import math
from random import sample
from secrets import choice

suits = ("♥", "♣", "♦", "♠")
cards = ("2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A")


def create_players(player_names):
    players = deque()
    for name in player_names:
        players.append(Player(name))
    return players


def create_deck():
    return set([x for x in range(52)])
    # return set(["".join(elem) for elem in itertools.product(suits, cards)])


def draw_hand(deck, num):
    hand = set(sample(list(deck), k=num))
    return deck - hand, hand


def make_guess(player, prev_guesses, player_count):
    guess = len([card for card in player.state.hand if card >= 40])
    if len(prev_guesses) == player_count - 1:
        if guess + sum(prev_guesses) == len(player.state.hand):
            new_guess = len([card for card in player.state.hand if card >= 45])
            if new_guess == guess:
                guess += 1
            else:
                guess = new_guess
    return guess


def determine_start_player(guesses):
    return guesses.index(max(guesses))


def determine_winner(trick):
    return trick.index(max(trick))


def play_card(hand, trick):
    card = choice(list(hand))
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
    ):
        self.name = name
        self.state = State(hand=[], guess=-1, wins=0, score=0)


def game(players: "list[str]"):
    players = create_players(players)
    sets = list(range(10, 1, -1)) + [1] * len(players) + list(range(2, 11))

    for set in sets:
        deck = create_deck()
        prev_guesses = []
        for player in players:
            deck, hand = draw_hand(deck, set)
            player.state = player.state._replace(hand=hand)
            guess = make_guess(player, prev_guesses, len(players))
            prev_guesses.append(guess)
            player.state = player.state._replace(guess=guess)
        index = determine_start_player(prev_guesses)
        players.rotate(-index)

        while len(players[0].state.hand) > 0:
            trick = []
            for player in players:
                hand, trick = play_card(player.state.hand, trick)
                player.state = player.state._replace(hand=hand)
            index = determine_winner(trick)
            players[index].state = player.state._replace(wins=player.state.wins + 1)
            players.rotate(-index)
        for player in players:
            if player.state.guess == player.state.wins:
                player.state = player.state._replace(
                    score=player.state.score + max(10, 10 * player.state.guess)
                )
            player.state = player.state._replace(wins=0)
    return determine_total_winners(players)


if __name__ == "__main__":
    players = ["Ingo", "Klara"]
    winners = game(players)
    print(f"The winner(s) is/are {','.join(players[winner] for winner in winners)}!")
