import itertools
from os import linesep
import random


suits = ("♥", "♣", "♦", "♠")
cards = ("2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A")
deck = set(["".join(elem) for elem in itertools.product(suits, cards)])

def draw(num, deck):
    drawn_cards = set(random.sample(deck, num))
    return drawn_cards, deck - drawn_cards

def valid_play(card, trick):
    if card[0] == trick[0]:
        return True
    return False

def format_hand(hand, trick):
    num_str = ""
    card_str = ""
    i = 0
    for elem in hand:
        start_str = ""
        end_str = ""
        if valid_play(elem, trick):
            start_str += '\033[1m'
            end_str = '\033[0m'
        num_str += start_str + f" {i}" + " "*(len(str(elem))-1) + end_str
        card_str += start_str + f"{elem}," + end_str
        i += 1
    return num_str, card_str

hand, deck = draw(7, deck)
trick, deck = draw(1, deck)
trick = trick.pop()
print(trick)
num_str, hand_str = format_hand(hand, trick)
print(num_str)
print(hand_str)