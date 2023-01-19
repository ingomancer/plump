from collections import namedtuple
import itertools
import math
from random import sample
from secrets import choice
from plump.format import format_hand, format_trick, format_scoreboard, format_guesses

suits = range(4)
cards = range(13)


class Player:
    def __init__(
        self,
        name,
        human,
    ):
        self.name = name
        self.human = human
        self.hand = []


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


def determine_total_winners(players, public):
    winners = []
    highest_score = -math.inf
    for index, player in enumerate(players):
        if public[player.name].score > highest_score:
            highest_score = public[player.name].score
            winners = [index]
        elif public[player.name].score == highest_score:
            winners.append(index)
    return winners


PublicState = namedtuple("PublicState", ["guess", "wins", "score"])


def game(read, write, players, num_rounds):
    sets = (
        list(range(num_rounds, 1, -1))
        + [1] * len(players)
        + list(range(2, num_rounds + 1))
    )

    public = {player.name: PublicState(guess=-1, wins=0, score=0) for player in players}

    for set in sets:
        players_in_set = players.copy()
        deck = create_deck()
        prev_guesses = []
        for player in players_in_set:
            deck, hand = draw_hand(deck, set)
            write(f"{player.name}'s turn")
            if player.human:
                public[player.name] = public[player.name]._replace(
                    guess=request_guess(
                        read,
                        write,
                        player.name,
                        hand,
                        prev_guesses,
                        len(players_in_set),
                    ),
                )
                player.hand = hand
            else:
                public[player.name] = public[player.name]._replace(
                    guess=make_guess(hand, prev_guesses, len(players_in_set))
                )
                player.hand = hand
            prev_guesses.append(public[player.name].guess)
        write(format_guesses(public))
        index = determine_start_player(prev_guesses)
        players_in_set.rotate(-index)

        while len(players_in_set[0].hand) > 0:
            trick = []
            for player in players_in_set:
                if player.human:
                    hand, trick = play_human_card(
                        read, write, player.name, player.hand, trick
                    )
                else:
                    hand, trick = play_card(player.hand, trick)
                player.hand = hand
                write(format_trick(trick))
            index = determine_winner(trick)
            winner = players_in_set[index].name
            public[winner] = public[winner]._replace(wins=public[winner].wins + 1)
            write(format_scoreboard(public))
            write(f"{winner} won!")
            players_in_set.rotate(-index)
        for player in players_in_set:
            public[player.name] = score_round(public[player.name])
        players.rotate(-1)
    return determine_total_winners(players)


def score_round(player_state):
    if player_state.guess == player_state.wins:
        score = player_state.score + max(10, 10 * player_state.guess)
    else:
        score = player_state.score
    player_state = player_state._replace(score=score, wins=0)
    return player_state
