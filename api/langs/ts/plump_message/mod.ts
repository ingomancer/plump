
import { Serializer, Deserializer } from '../serde/mod.ts';
import { BcsSerializer, BcsDeserializer } from '../bcs/mod.ts';
import { Optional, Seq, Tuple, ListTuple, unit, bool, int8, int16, int32, int64, int128, uint8, uint16, uint32, uint64, uint128, float32, float64, char, str, bytes } from '../serde/mod.ts';

export class Card {

constructor (public suit: uint64, public value: uint64) {
}

public serialize(serializer: Serializer): void {
  serializer.serializeU64(this.suit);
  serializer.serializeU64(this.value);
}

static deserialize(deserializer: Deserializer): Card {
  const suit = deserializer.deserializeU64();
  const value = deserializer.deserializeU64();
  return new Card(suit,value);
}

}
export abstract class Message {
abstract serialize(serializer: Serializer): void;

static deserialize(deserializer: Deserializer): Message {
  const index = deserializer.deserializeVariantIndex();
  switch (index) {
    case 0: return MessageVariantRequestGuessContext.load(deserializer);
    case 1: return MessageVariantGuesses.load(deserializer);
    case 2: return MessageVariantTurn.load(deserializer);
    case 3: return MessageVariantPlayRequestContext.load(deserializer);
    case 4: return MessageVariantTrick.load(deserializer);
    case 5: return MessageVariantScoreboard.load(deserializer);
    case 6: return MessageVariantWinner.load(deserializer);
    case 7: return MessageVariantWinners.load(deserializer);
    case 8: return MessageVariantRequestPlayerName.load(deserializer);
    case 9: return MessageVariantPlayRequest.load(deserializer);
    case 10: return MessageVariantRequestGuess.load(deserializer);
    case 11: return MessageVariantGameOver.load(deserializer);
    default: throw new Error("Unknown variant index for Message: " + index);
  }
}
}


export class MessageVariantRequestGuessContext extends Message {

constructor (public player: Player, public hand: Seq<Card>, public guesses: Seq<uint64>, public players: uint64) {
  super();
}

public serialize(serializer: Serializer): void {
  serializer.serializeVariantIndex(0);
  this.player.serialize(serializer);
  Helpers.serializeVectorCard(this.hand, serializer);
  Helpers.serializeVectorU64(this.guesses, serializer);
  serializer.serializeU64(this.players);
}

static load(deserializer: Deserializer): MessageVariantRequestGuessContext {
  const player = Player.deserialize(deserializer);
  const hand = Helpers.deserializeVectorCard(deserializer);
  const guesses = Helpers.deserializeVectorU64(deserializer);
  const players = deserializer.deserializeU64();
  return new MessageVariantRequestGuessContext(player,hand,guesses,players);
}

}

export class MessageVariantGuesses extends Message {

constructor (public state: Map<PlayerName,PublicState>) {
  super();
}

public serialize(serializer: Serializer): void {
  serializer.serializeVariantIndex(1);
  Helpers.serializeMapPlayerNameToPublicState(this.state, serializer);
}

static load(deserializer: Deserializer): MessageVariantGuesses {
  const state = Helpers.deserializeMapPlayerNameToPublicState(deserializer);
  return new MessageVariantGuesses(state);
}

}

export class MessageVariantTurn extends Message {

constructor (public whose: Player) {
  super();
}

public serialize(serializer: Serializer): void {
  serializer.serializeVariantIndex(2);
  this.whose.serialize(serializer);
}

static load(deserializer: Deserializer): MessageVariantTurn {
  const whose = Player.deserialize(deserializer);
  return new MessageVariantTurn(whose);
}

}

export class MessageVariantPlayRequestContext extends Message {

constructor (public player: Player, public hand: Seq<Card>, public trick: Trick, public valid_cards: Optional<Seq<uint64>>) {
  super();
}

public serialize(serializer: Serializer): void {
  serializer.serializeVariantIndex(3);
  this.player.serialize(serializer);
  Helpers.serializeVectorCard(this.hand, serializer);
  this.trick.serialize(serializer);
  Helpers.serializeOptionVectorU64(this.valid_cards, serializer);
}

static load(deserializer: Deserializer): MessageVariantPlayRequestContext {
  const player = Player.deserialize(deserializer);
  const hand = Helpers.deserializeVectorCard(deserializer);
  const trick = Trick.deserialize(deserializer);
  const valid_cards = Helpers.deserializeOptionVectorU64(deserializer);
  return new MessageVariantPlayRequestContext(player,hand,trick,valid_cards);
}

}

export class MessageVariantTrick extends Message {

constructor (public value: Trick) {
  super();
}

public serialize(serializer: Serializer): void {
  serializer.serializeVariantIndex(4);
  this.value.serialize(serializer);
}

static load(deserializer: Deserializer): MessageVariantTrick {
  const value = Trick.deserialize(deserializer);
  return new MessageVariantTrick(value);
}

}

export class MessageVariantScoreboard extends Message {

constructor (public state: Map<PlayerName,PublicState>) {
  super();
}

public serialize(serializer: Serializer): void {
  serializer.serializeVariantIndex(5);
  Helpers.serializeMapPlayerNameToPublicState(this.state, serializer);
}

static load(deserializer: Deserializer): MessageVariantScoreboard {
  const state = Helpers.deserializeMapPlayerNameToPublicState(deserializer);
  return new MessageVariantScoreboard(state);
}

}

export class MessageVariantWinner extends Message {

constructor (public value: Player) {
  super();
}

public serialize(serializer: Serializer): void {
  serializer.serializeVariantIndex(6);
  this.value.serialize(serializer);
}

static load(deserializer: Deserializer): MessageVariantWinner {
  const value = Player.deserialize(deserializer);
  return new MessageVariantWinner(value);
}

}

export class MessageVariantWinners extends Message {

constructor (public players: Seq<Player>, public winner_indices: Seq<uint64>) {
  super();
}

public serialize(serializer: Serializer): void {
  serializer.serializeVariantIndex(7);
  Helpers.serializeVectorPlayer(this.players, serializer);
  Helpers.serializeVectorU64(this.winner_indices, serializer);
}

static load(deserializer: Deserializer): MessageVariantWinners {
  const players = Helpers.deserializeVectorPlayer(deserializer);
  const winner_indices = Helpers.deserializeVectorU64(deserializer);
  return new MessageVariantWinners(players,winner_indices);
}

}

export class MessageVariantRequestPlayerName extends Message {
constructor () {
  super();
}

public serialize(serializer: Serializer): void {
  serializer.serializeVariantIndex(8);
}

static load(deserializer: Deserializer): MessageVariantRequestPlayerName {
  return new MessageVariantRequestPlayerName();
}

}

export class MessageVariantPlayRequest extends Message {

constructor (public value: Player) {
  super();
}

public serialize(serializer: Serializer): void {
  serializer.serializeVariantIndex(9);
  this.value.serialize(serializer);
}

static load(deserializer: Deserializer): MessageVariantPlayRequest {
  const value = Player.deserialize(deserializer);
  return new MessageVariantPlayRequest(value);
}

}

export class MessageVariantRequestGuess extends Message {
constructor () {
  super();
}

public serialize(serializer: Serializer): void {
  serializer.serializeVariantIndex(10);
}

static load(deserializer: Deserializer): MessageVariantRequestGuess {
  return new MessageVariantRequestGuess();
}

}

export class MessageVariantGameOver extends Message {
constructor () {
  super();
}

public serialize(serializer: Serializer): void {
  serializer.serializeVariantIndex(11);
}

static load(deserializer: Deserializer): MessageVariantGameOver {
  return new MessageVariantGameOver();
}

}
export class Player {

constructor (public name: PlayerName, public human: bool, public hand: Seq<Card>) {
}

public serialize(serializer: Serializer): void {
  this.name.serialize(serializer);
  serializer.serializeBool(this.human);
  Helpers.serializeVectorCard(this.hand, serializer);
}

static deserialize(deserializer: Deserializer): Player {
  const name = PlayerName.deserialize(deserializer);
  const human = deserializer.deserializeBool();
  const hand = Helpers.deserializeVectorCard(deserializer);
  return new Player(name,human,hand);
}

}
export class PlayerName {

constructor (public value: str) {
}

public serialize(serializer: Serializer): void {
  serializer.serializeStr(this.value);
}

static deserialize(deserializer: Deserializer): PlayerName {
  const value = deserializer.deserializeStr();
  return new PlayerName(value);
}

}
export class PublicState {

constructor (public guess: Optional<uint64>, public wins: uint64, public score: uint64) {
}

public serialize(serializer: Serializer): void {
  Helpers.serializeOptionU64(this.guess, serializer);
  serializer.serializeU64(this.wins);
  serializer.serializeU64(this.score);
}

static deserialize(deserializer: Deserializer): PublicState {
  const guess = Helpers.deserializeOptionU64(deserializer);
  const wins = deserializer.deserializeU64();
  const score = deserializer.deserializeU64();
  return new PublicState(guess,wins,score);
}

}
export class Trick {

constructor (public value: Seq<Card>) {
}

public serialize(serializer: Serializer): void {
  Helpers.serializeVectorCard(this.value, serializer);
}

static deserialize(deserializer: Deserializer): Trick {
  const value = Helpers.deserializeVectorCard(deserializer);
  return new Trick(value);
}

}
export class Helpers {
  static serializeMapPlayerNameToPublicState(value: Map<PlayerName,PublicState>, serializer: Serializer): void {
    serializer.serializeLen(value.size);
    const offsets: number[] = [];
    for (const [k, v] of value.entries()) {
      offsets.push(serializer.getBufferOffset());
      k.serialize(serializer);
      v.serialize(serializer);
    }
    serializer.sortMapEntries(offsets);
  }

  static deserializeMapPlayerNameToPublicState(deserializer: Deserializer): Map<PlayerName,PublicState> {
    const length = deserializer.deserializeLen();
    const obj = new Map<PlayerName, PublicState>();
    let previousKeyStart = 0;
    let previousKeyEnd = 0;
    for (let i = 0; i < length; i++) {
        const keyStart = deserializer.getBufferOffset();
        const key = PlayerName.deserialize(deserializer);
        const keyEnd = deserializer.getBufferOffset();
        if (i > 0) {
            deserializer.checkThatKeySlicesAreIncreasing(
                [previousKeyStart, previousKeyEnd],
                [keyStart, keyEnd]);
        }
        previousKeyStart = keyStart;
        previousKeyEnd = keyEnd;
        const value = PublicState.deserialize(deserializer);
        obj.set(key, value);
    }
    return obj;
  }

  static serializeOptionU64(value: Optional<uint64>, serializer: Serializer): void {
    if (value) {
        serializer.serializeOptionTag(true);
        serializer.serializeU64(value);
    } else {
        serializer.serializeOptionTag(false);
    }
  }

  static deserializeOptionU64(deserializer: Deserializer): Optional<uint64> {
    const tag = deserializer.deserializeOptionTag();
    if (!tag) {
        return null;
    } else {
        return deserializer.deserializeU64();
    }
  }

  static serializeOptionVectorU64(value: Optional<Seq<uint64>>, serializer: Serializer): void {
    if (value) {
        serializer.serializeOptionTag(true);
        Helpers.serializeVectorU64(value, serializer);
    } else {
        serializer.serializeOptionTag(false);
    }
  }

  static deserializeOptionVectorU64(deserializer: Deserializer): Optional<Seq<uint64>> {
    const tag = deserializer.deserializeOptionTag();
    if (!tag) {
        return null;
    } else {
        return Helpers.deserializeVectorU64(deserializer);
    }
  }

  static serializeVectorCard(value: Seq<Card>, serializer: Serializer): void {
    serializer.serializeLen(value.length);
    value.forEach((item: Card) => {
        item.serialize(serializer);
    });
  }

  static deserializeVectorCard(deserializer: Deserializer): Seq<Card> {
    const length = deserializer.deserializeLen();
    const list: Seq<Card> = [];
    for (let i = 0; i < length; i++) {
        list.push(Card.deserialize(deserializer));
    }
    return list;
  }

  static serializeVectorPlayer(value: Seq<Player>, serializer: Serializer): void {
    serializer.serializeLen(value.length);
    value.forEach((item: Player) => {
        item.serialize(serializer);
    });
  }

  static deserializeVectorPlayer(deserializer: Deserializer): Seq<Player> {
    const length = deserializer.deserializeLen();
    const list: Seq<Player> = [];
    for (let i = 0; i < length; i++) {
        list.push(Player.deserialize(deserializer));
    }
    return list;
  }

  static serializeVectorU64(value: Seq<uint64>, serializer: Serializer): void {
    serializer.serializeLen(value.length);
    value.forEach((item: uint64) => {
        serializer.serializeU64(item);
    });
  }

  static deserializeVectorU64(deserializer: Deserializer): Seq<uint64> {
    const length = deserializer.deserializeLen();
    const list: Seq<uint64> = [];
    for (let i = 0; i < length; i++) {
        list.push(deserializer.deserializeU64());
    }
    return list;
  }

}

