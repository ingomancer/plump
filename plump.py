from aifc import Aifc_read
import itertools
import random


suits = ("♥", "♣", "♦", "♠")
cards = ("2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A")

def draw(num, deck):
    drawn_cards = set(random.sample(list(deck), num))
    return drawn_cards, deck - drawn_cards

def valid_play(card, trick):
    if card[0] == trick[0]:
        return True
    return False

def format_hand(hand, trick = None):
    num_str = ""
    card_str = ""
    i = 0
    for elem in sorted(hand, key=sort_cards):
        start_str = ""
        end_str = ""
        if trick:
            if valid_play(elem, trick):
                start_str += '\033[1m'
                end_str = '\033[0m'
            num_str += start_str + f" {i}" + " "*(len(str(elem))-1) + end_str
        card_str += start_str + f"{elem}," + end_str
        i += 1
    return num_str, card_str

def sort_cards(card):
    suit = ord(card[0])
    value = card[1:]
    try:
        value = int(value)
    except ValueError:
        value = ord(value)
    return (suit, value)

def game(players):
    player_count = len(players)
    if player_count > 4:
        print("A max of four players is supported right now")
        exit(1)
    sets = list(range(10, 1, -1)) + [1] * player_count + list(range(2, 11))
    opening_player = 0
    for round in sets:
        deck = set(["".join(elem) for elem in itertools.product(suits, cards)])
        trump, deck = draw(1, deck)
        guesses = []
        for i in range(player_count):
            player_hand, deck = draw(round, deck)
            guesses.append(players[(i+opening_player)%player_count].guess(player_hand, trump))



class AIPlayer():
    def guess(self, hand, trump):
        return random.randint(0, len(hand))

    def play(self, hand, trump, trick):
        pass

class HumanPlayer():
    def guess(self, hand, trump):
        print(trump.pop())
        print(format_hand(hand)[1])
        return input("Number of tricks: ")

    def play(self, hand, trump, trick):
        pass

if __name__ == "__main__":
    game([AIPlayer(), AIPlayer(), AIPlayer(), HumanPlayer()])