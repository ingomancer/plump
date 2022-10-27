from collections import deque
from hypothesis import assume, example, given
from hypothesis.strategies import lists, text, integers, sets, tuples, booleans
import plump.game as game

cards = tuples(integers(min_value=0, max_value=3), integers(min_value=0, max_value=12))
player_count = {"min_size": 2, "max_size": 4}


@given(lists(tuples(text(), booleans())))
def test_create_players(names):
    players = game.create_players(names)
    assert len(players) == len(names)
    for name_and_human, player in zip(names, players):
        name, human = name_and_human
        assert name == player.name
        assert human == player.human
    assert type(players) == deque


def test_create_deck():
    deck = game.create_deck()
    assert len(deck) == 52
    assert len(set(deck)) == 52


@given(sets(cards, max_size=52), integers(max_value=10, min_value=1))
def test_draw_hand(deck, hand_size):
    assume(hand_size <= len(deck))
    new_deck, hand = game.draw_hand(set(deck), hand_size)
    assert new_deck.issubset(deck)
    assert len(new_deck) + hand_size == len(deck)
    assert hand.issubset(deck)
    assert len(hand) == hand_size


@given(
    sets(cards, max_size=10),
    sets(
        integers(max_value=10, min_value=0),
        max_size=3,
        min_size=0,
    ),
)
@example(prev_guesses={0, 1, 2}, hand={(0, 0), (0, 1), (1, 2), (3, 4)})
def test_make_guess(hand, prev_guesses):
    assume(max(prev_guesses, default=0) <= len(hand))
    guess = game.make_guess(hand, prev_guesses, 4)
    assert guess >= 0
    assert guess <= len(hand)
    if len(prev_guesses) == 3:
        assert (sum(prev_guesses) + guess) != len(hand)


@given(lists(integers(max_value=10, min_value=0), **player_count))
def test_determine_start_player(guesses):
    index = game.determine_start_player(guesses)
    assert index < len(guesses)
    assert index >= 0
    assert index == guesses.index(max(guesses))


@given(sets(cards, min_size=1, max_size=10), lists(cards, max_size=4))
def test_play_card(hand, trick):
    new_hand, new_trick = game.play_card(hand, trick)
    assert len(new_trick) == len(trick) + 1
    assert len(new_hand) == len(hand) - 1
    assert new_trick[-1] in hand
    assert trick == new_trick[:-1]
    assert set(x for x in hand if x != new_trick[-1]) == new_hand


@given(lists(cards, **player_count))
def test_determine_winner(trick):
    index = game.determine_winner(trick)
    assert index < len(trick)
    assert index >= 0
    assert index == trick.index(max(trick))


@given(sets(tuples(text(), integers()), **player_count))
def test_determine_total_winner(names_and_scores):
    names, scores = list(map(list, zip(*names_and_scores)))
    players = game.create_players((name, False) for name in names)
    for player, score in zip(players, scores):
        player.state = player.state._replace(score=score)
    winners = game.determine_total_winners(players)
    assert len(winners) <= len(players)
    assert max(winners) < len(players)
    assert len(winners) > 0
    assert min(winners) >= 0
    assert all(
        (
            players[winner].state.score > player.state.score
            for index, player in enumerate(players)
            if index not in winners
            for winner in winners
        )
    )
