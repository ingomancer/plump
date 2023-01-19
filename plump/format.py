suit_symbols = ["♥", "♣", "♦", "♠"]
cards_symbols = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"]


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


def format_guesses(public):
    return "Guesses: " + ", ".join(
        [f"{name}: {state.guess}" for name, state in public.items()]
    )


upside_down_face = "\U0001F643"
slightly_smiling_face = "\U0001F642"


def format_scoreboard(public):
    sorted_player_names = sorted(public.keys())

    def format_state(public):
        did_plump = public.wins != public.guess
        return f"{public.wins}/{public.guess} {upside_down_face if did_plump else slightly_smiling_face} (total: {public.score})"

    return ", ".join(
        [f"{name}: {format_state(public[name])}" for name in sorted_player_names]
    )
