# pyre-strict
from dataclasses import dataclass
import typing
import serde_types as st
import bincode

@dataclass(frozen=True)
class Card:
    suit: st.uint64
    value: st.uint64

    def bincode_serialize(self) -> bytes:
        return bincode.serialize(self, Card)

    @staticmethod
    def bincode_deserialize(input: bytes) -> 'Card':
        v, buffer = bincode.deserialize(input, Card)
        if buffer:
            raise st.DeserializationError("Some input bytes were not read");
        return v


class Message:
    VARIANTS = []  # type: typing.Sequence[typing.Type[Message]]

    def bincode_serialize(self) -> bytes:
        return bincode.serialize(self, Message)

    @staticmethod
    def bincode_deserialize(input: bytes) -> 'Message':
        v, buffer = bincode.deserialize(input, Message)
        if buffer:
            raise st.DeserializationError("Some input bytes were not read");
        return v


@dataclass(frozen=True)
class Message__RequestGuessContext(Message):
    INDEX = 0  # type: int
    player: "Player"
    hand: typing.Sequence["Card"]
    guesses: typing.Sequence[st.uint64]
    players: st.uint64


@dataclass(frozen=True)
class Message__Guesses(Message):
    INDEX = 1  # type: int
    state: typing.Dict["PlayerName", "PublicState"]


@dataclass(frozen=True)
class Message__Turn(Message):
    INDEX = 2  # type: int
    whose: "Player"


@dataclass(frozen=True)
class Message__PlayRequestContext(Message):
    INDEX = 3  # type: int
    player: "Player"
    hand: typing.Sequence["Card"]
    trick: "Trick"
    valid_cards: typing.Optional[typing.Sequence[st.uint64]]


@dataclass(frozen=True)
class Message__Trick(Message):
    INDEX = 4  # type: int
    value: "Trick"


@dataclass(frozen=True)
class Message__Scoreboard(Message):
    INDEX = 5  # type: int
    state: typing.Dict["PlayerName", "PublicState"]


@dataclass(frozen=True)
class Message__Winner(Message):
    INDEX = 6  # type: int
    value: "Player"


@dataclass(frozen=True)
class Message__Winners(Message):
    INDEX = 7  # type: int
    players: typing.Sequence["Player"]
    winner_indices: typing.Sequence[st.uint64]


@dataclass(frozen=True)
class Message__RequestPlayerName(Message):
    INDEX = 8  # type: int
    pass


@dataclass(frozen=True)
class Message__PlayRequest(Message):
    INDEX = 9  # type: int
    value: "Player"


@dataclass(frozen=True)
class Message__RequestGuess(Message):
    INDEX = 10  # type: int
    pass


@dataclass(frozen=True)
class Message__GameOver(Message):
    INDEX = 11  # type: int
    pass

Message.VARIANTS = [
    Message__RequestGuessContext,
    Message__Guesses,
    Message__Turn,
    Message__PlayRequestContext,
    Message__Trick,
    Message__Scoreboard,
    Message__Winner,
    Message__Winners,
    Message__RequestPlayerName,
    Message__PlayRequest,
    Message__RequestGuess,
    Message__GameOver,
]


@dataclass(frozen=True)
class Player:
    name: "PlayerName"
    human: bool
    hand: typing.Sequence["Card"]

    def bincode_serialize(self) -> bytes:
        return bincode.serialize(self, Player)

    @staticmethod
    def bincode_deserialize(input: bytes) -> 'Player':
        v, buffer = bincode.deserialize(input, Player)
        if buffer:
            raise st.DeserializationError("Some input bytes were not read");
        return v


@dataclass(frozen=True)
class PlayerName:
    value: str

    def bincode_serialize(self) -> bytes:
        return bincode.serialize(self, PlayerName)

    @staticmethod
    def bincode_deserialize(input: bytes) -> 'PlayerName':
        v, buffer = bincode.deserialize(input, PlayerName)
        if buffer:
            raise st.DeserializationError("Some input bytes were not read");
        return v


@dataclass(frozen=True)
class PublicState:
    guess: typing.Optional[st.uint64]
    wins: st.uint64
    score: st.uint64

    def bincode_serialize(self) -> bytes:
        return bincode.serialize(self, PublicState)

    @staticmethod
    def bincode_deserialize(input: bytes) -> 'PublicState':
        v, buffer = bincode.deserialize(input, PublicState)
        if buffer:
            raise st.DeserializationError("Some input bytes were not read");
        return v


@dataclass(frozen=True)
class Trick:
    value: typing.Sequence["Card"]

    def bincode_serialize(self) -> bytes:
        return bincode.serialize(self, Trick)

    @staticmethod
    def bincode_deserialize(input: bytes) -> 'Trick':
        v, buffer = bincode.deserialize(input, Trick)
        if buffer:
            raise st.DeserializationError("Some input bytes were not read");
        return v

