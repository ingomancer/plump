using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Numerics;

namespace plump_message {

    public abstract class Message: IEquatable<Message>, ICloneable {

        public abstract void Serialize(Serde.ISerializer serializer);

        public static Message Deserialize(Serde.IDeserializer deserializer) {
            int index = deserializer.deserialize_variant_index();
            switch (index) {
                case 0: return RequestGuessContext.Load(deserializer);
                case 1: return Guesses.Load(deserializer);
                case 2: return Turn.Load(deserializer);
                case 3: return PlayRequestContext.Load(deserializer);
                case 4: return Trick.Load(deserializer);
                case 5: return Scoreboard.Load(deserializer);
                case 6: return Winner.Load(deserializer);
                case 7: return Winners.Load(deserializer);
                case 8: return RequestPlayerName.Load(deserializer);
                case 9: return PlayRequest.Load(deserializer);
                case 10: return RequestGuess.Load(deserializer);
                case 11: return GameOver.Load(deserializer);
                default: throw new Serde.DeserializationException("Unknown variant index for Message: " + index);
            }
        }

        public int BincodeSerialize(byte[] outputBuffer) => BincodeSerialize(new ArraySegment<byte>(outputBuffer));

        public int BincodeSerialize(ArraySegment<byte> outputBuffer) {
            Serde.ISerializer serializer = new Bincode.BincodeSerializer(outputBuffer);
            Serialize(serializer);
            return serializer.get_buffer_offset();
        }

        public byte[] BincodeSerialize()  {
            Serde.ISerializer serializer = new Bincode.BincodeSerializer();
            Serialize(serializer);
            return serializer.get_bytes();
        }

        public static Message BincodeDeserialize(byte[] input) => BincodeDeserialize(new ArraySegment<byte>(input));

        public static Message BincodeDeserialize(ArraySegment<byte> input) {
            if (input == null) {
                 throw new Serde.DeserializationException("Cannot deserialize null array");
            }
            Serde.IDeserializer deserializer = new Bincode.BincodeDeserializer(input);
            Message value = Deserialize(deserializer);
            if (deserializer.get_buffer_offset() < input.Count) {
                 throw new Serde.DeserializationException("Some input bytes were not read");
            }
            return value;
        }
        public override int GetHashCode() {
            switch (this) {
            case RequestGuessContext x: return x.GetHashCode();
            case Guesses x: return x.GetHashCode();
            case Turn x: return x.GetHashCode();
            case PlayRequestContext x: return x.GetHashCode();
            case Trick x: return x.GetHashCode();
            case Scoreboard x: return x.GetHashCode();
            case Winner x: return x.GetHashCode();
            case Winners x: return x.GetHashCode();
            case RequestPlayerName x: return x.GetHashCode();
            case PlayRequest x: return x.GetHashCode();
            case RequestGuess x: return x.GetHashCode();
            case GameOver x: return x.GetHashCode();
            default: throw new InvalidOperationException("Unknown variant type");
            }
        }
        public override bool Equals(object obj) => obj is Message other && Equals(other);

        public bool Equals(Message other) {
            if (other == null) return false;
            if (ReferenceEquals(this, other)) return true;
            if (GetType() != other.GetType()) return false;
            switch (this) {
            case RequestGuessContext x: return x.Equals((RequestGuessContext)other);
            case Guesses x: return x.Equals((Guesses)other);
            case Turn x: return x.Equals((Turn)other);
            case PlayRequestContext x: return x.Equals((PlayRequestContext)other);
            case Trick x: return x.Equals((Trick)other);
            case Scoreboard x: return x.Equals((Scoreboard)other);
            case Winner x: return x.Equals((Winner)other);
            case Winners x: return x.Equals((Winners)other);
            case RequestPlayerName x: return x.Equals((RequestPlayerName)other);
            case PlayRequest x: return x.Equals((PlayRequest)other);
            case RequestGuess x: return x.Equals((RequestGuess)other);
            case GameOver x: return x.Equals((GameOver)other);
            default: throw new InvalidOperationException("Unknown variant type");
            }
        }

        /// <summary>Creates a shallow clone of the object.</summary>
        public Message Clone() => (Message)MemberwiseClone();

        object ICloneable.Clone() => Clone();


        public sealed class RequestGuessContext: Message, IEquatable<RequestGuessContext>, ICloneable {
            public Player player;
            public Serde.ValueArray<Card> hand;
            public Serde.ValueArray<ulong> guesses;
            public ulong players;

            public RequestGuessContext(Player _player, Serde.ValueArray<Card> _hand, Serde.ValueArray<ulong> _guesses, ulong _players) {
                if (_player == null) throw new ArgumentNullException(nameof(_player));
                player = _player;
                if (_hand == null) throw new ArgumentNullException(nameof(_hand));
                hand = _hand;
                if (_guesses == null) throw new ArgumentNullException(nameof(_guesses));
                guesses = _guesses;
                players = _players;
            }

            public override void Serialize(Serde.ISerializer serializer) {
                serializer.increase_container_depth();
                serializer.serialize_variant_index(0);
                player.Serialize(serializer);
                TraitHelpers.serialize_vector_Card(hand, serializer);
                TraitHelpers.serialize_vector_u64(guesses, serializer);
                serializer.serialize_u64(players);
                serializer.decrease_container_depth();
            }

            internal static RequestGuessContext Load(Serde.IDeserializer deserializer) {
                deserializer.increase_container_depth();
                RequestGuessContext obj = new RequestGuessContext(
                	Player.Deserialize(deserializer),
                	TraitHelpers.deserialize_vector_Card(deserializer),
                	TraitHelpers.deserialize_vector_u64(deserializer),
                	deserializer.deserialize_u64());
                deserializer.decrease_container_depth();
                return obj;
            }
            public override bool Equals(object obj) => obj is RequestGuessContext other && Equals(other);

            public static bool operator ==(RequestGuessContext left, RequestGuessContext right) => Equals(left, right);

            public static bool operator !=(RequestGuessContext left, RequestGuessContext right) => !Equals(left, right);

            public bool Equals(RequestGuessContext other) {
                if (other == null) return false;
                if (ReferenceEquals(this, other)) return true;
                if (!player.Equals(other.player)) return false;
                if (!hand.Equals(other.hand)) return false;
                if (!guesses.Equals(other.guesses)) return false;
                if (!players.Equals(other.players)) return false;
                return true;
            }

            public override int GetHashCode() {
                unchecked {
                    int value = 7;
                    value = 31 * value + player.GetHashCode();
                    value = 31 * value + hand.GetHashCode();
                    value = 31 * value + guesses.GetHashCode();
                    value = 31 * value + players.GetHashCode();
                    return value;
                }
            }

        }

        public sealed class Guesses: Message, IEquatable<Guesses>, ICloneable {
            public Serde.ValueDictionary<PlayerName, PublicState> state;

            public Guesses(Serde.ValueDictionary<PlayerName, PublicState> _state) {
                if (_state == null) throw new ArgumentNullException(nameof(_state));
                state = _state;
            }

            public override void Serialize(Serde.ISerializer serializer) {
                serializer.increase_container_depth();
                serializer.serialize_variant_index(1);
                TraitHelpers.serialize_map_PlayerName_to_PublicState(state, serializer);
                serializer.decrease_container_depth();
            }

            internal static Guesses Load(Serde.IDeserializer deserializer) {
                deserializer.increase_container_depth();
                Guesses obj = new Guesses(
                	TraitHelpers.deserialize_map_PlayerName_to_PublicState(deserializer));
                deserializer.decrease_container_depth();
                return obj;
            }
            public override bool Equals(object obj) => obj is Guesses other && Equals(other);

            public static bool operator ==(Guesses left, Guesses right) => Equals(left, right);

            public static bool operator !=(Guesses left, Guesses right) => !Equals(left, right);

            public bool Equals(Guesses other) {
                if (other == null) return false;
                if (ReferenceEquals(this, other)) return true;
                if (!state.Equals(other.state)) return false;
                return true;
            }

            public override int GetHashCode() {
                unchecked {
                    int value = 7;
                    value = 31 * value + state.GetHashCode();
                    return value;
                }
            }

        }

        public sealed class Turn: Message, IEquatable<Turn>, ICloneable {
            public Player whose;

            public Turn(Player _whose) {
                if (_whose == null) throw new ArgumentNullException(nameof(_whose));
                whose = _whose;
            }

            public override void Serialize(Serde.ISerializer serializer) {
                serializer.increase_container_depth();
                serializer.serialize_variant_index(2);
                whose.Serialize(serializer);
                serializer.decrease_container_depth();
            }

            internal static Turn Load(Serde.IDeserializer deserializer) {
                deserializer.increase_container_depth();
                Turn obj = new Turn(
                	Player.Deserialize(deserializer));
                deserializer.decrease_container_depth();
                return obj;
            }
            public override bool Equals(object obj) => obj is Turn other && Equals(other);

            public static bool operator ==(Turn left, Turn right) => Equals(left, right);

            public static bool operator !=(Turn left, Turn right) => !Equals(left, right);

            public bool Equals(Turn other) {
                if (other == null) return false;
                if (ReferenceEquals(this, other)) return true;
                if (!whose.Equals(other.whose)) return false;
                return true;
            }

            public override int GetHashCode() {
                unchecked {
                    int value = 7;
                    value = 31 * value + whose.GetHashCode();
                    return value;
                }
            }

        }

        public sealed class PlayRequestContext: Message, IEquatable<PlayRequestContext>, ICloneable {
            public Player player;
            public Serde.ValueArray<Card> hand;
            public plump_message.Trick trick;
            public Serde.Option<Serde.ValueArray<ulong>> valid_cards;

            public PlayRequestContext(Player _player, Serde.ValueArray<Card> _hand, plump_message.Trick _trick, Serde.Option<Serde.ValueArray<ulong>> _valid_cards) {
                if (_player == null) throw new ArgumentNullException(nameof(_player));
                player = _player;
                if (_hand == null) throw new ArgumentNullException(nameof(_hand));
                hand = _hand;
                if (_trick == null) throw new ArgumentNullException(nameof(_trick));
                trick = _trick;
                valid_cards = _valid_cards;
            }

            public override void Serialize(Serde.ISerializer serializer) {
                serializer.increase_container_depth();
                serializer.serialize_variant_index(3);
                player.Serialize(serializer);
                TraitHelpers.serialize_vector_Card(hand, serializer);
                trick.Serialize(serializer);
                TraitHelpers.serialize_option_vector_u64(valid_cards, serializer);
                serializer.decrease_container_depth();
            }

            internal static PlayRequestContext Load(Serde.IDeserializer deserializer) {
                deserializer.increase_container_depth();
                PlayRequestContext obj = new PlayRequestContext(
                	Player.Deserialize(deserializer),
                	TraitHelpers.deserialize_vector_Card(deserializer),
                	plump_message.Trick.Deserialize(deserializer),
                	TraitHelpers.deserialize_option_vector_u64(deserializer));
                deserializer.decrease_container_depth();
                return obj;
            }
            public override bool Equals(object obj) => obj is PlayRequestContext other && Equals(other);

            public static bool operator ==(PlayRequestContext left, PlayRequestContext right) => Equals(left, right);

            public static bool operator !=(PlayRequestContext left, PlayRequestContext right) => !Equals(left, right);

            public bool Equals(PlayRequestContext other) {
                if (other == null) return false;
                if (ReferenceEquals(this, other)) return true;
                if (!player.Equals(other.player)) return false;
                if (!hand.Equals(other.hand)) return false;
                if (!trick.Equals(other.trick)) return false;
                if (!valid_cards.Equals(other.valid_cards)) return false;
                return true;
            }

            public override int GetHashCode() {
                unchecked {
                    int value = 7;
                    value = 31 * value + player.GetHashCode();
                    value = 31 * value + hand.GetHashCode();
                    value = 31 * value + trick.GetHashCode();
                    value = 31 * value + valid_cards.GetHashCode();
                    return value;
                }
            }

        }

        public sealed class Trick: Message, IEquatable<Trick>, ICloneable {
            public plump_message.Trick value;

            public Trick(plump_message.Trick _value) {
                if (_value == null) throw new ArgumentNullException(nameof(_value));
                value = _value;
            }

            public override void Serialize(Serde.ISerializer serializer) {
                serializer.increase_container_depth();
                serializer.serialize_variant_index(4);
                value.Serialize(serializer);
                serializer.decrease_container_depth();
            }

            internal static Trick Load(Serde.IDeserializer deserializer) {
                deserializer.increase_container_depth();
                Trick obj = new Trick(
                	plump_message.Trick.Deserialize(deserializer));
                deserializer.decrease_container_depth();
                return obj;
            }
            public override bool Equals(object obj) => obj is Trick other && Equals(other);

            public static bool operator ==(Trick left, Trick right) => Equals(left, right);

            public static bool operator !=(Trick left, Trick right) => !Equals(left, right);

            public bool Equals(Trick other) {
                if (other == null) return false;
                if (ReferenceEquals(this, other)) return true;
                if (!value.Equals(other.value)) return false;
                return true;
            }

            public override int GetHashCode() {
                unchecked {
                    int value = 7;
                    value = 31 * value + value.GetHashCode();
                    return value;
                }
            }

        }

        public sealed class Scoreboard: Message, IEquatable<Scoreboard>, ICloneable {
            public Serde.ValueDictionary<PlayerName, PublicState> state;

            public Scoreboard(Serde.ValueDictionary<PlayerName, PublicState> _state) {
                if (_state == null) throw new ArgumentNullException(nameof(_state));
                state = _state;
            }

            public override void Serialize(Serde.ISerializer serializer) {
                serializer.increase_container_depth();
                serializer.serialize_variant_index(5);
                TraitHelpers.serialize_map_PlayerName_to_PublicState(state, serializer);
                serializer.decrease_container_depth();
            }

            internal static Scoreboard Load(Serde.IDeserializer deserializer) {
                deserializer.increase_container_depth();
                Scoreboard obj = new Scoreboard(
                	TraitHelpers.deserialize_map_PlayerName_to_PublicState(deserializer));
                deserializer.decrease_container_depth();
                return obj;
            }
            public override bool Equals(object obj) => obj is Scoreboard other && Equals(other);

            public static bool operator ==(Scoreboard left, Scoreboard right) => Equals(left, right);

            public static bool operator !=(Scoreboard left, Scoreboard right) => !Equals(left, right);

            public bool Equals(Scoreboard other) {
                if (other == null) return false;
                if (ReferenceEquals(this, other)) return true;
                if (!state.Equals(other.state)) return false;
                return true;
            }

            public override int GetHashCode() {
                unchecked {
                    int value = 7;
                    value = 31 * value + state.GetHashCode();
                    return value;
                }
            }

        }

        public sealed class Winner: Message, IEquatable<Winner>, ICloneable {
            public Player value;

            public Winner(Player _value) {
                if (_value == null) throw new ArgumentNullException(nameof(_value));
                value = _value;
            }

            public override void Serialize(Serde.ISerializer serializer) {
                serializer.increase_container_depth();
                serializer.serialize_variant_index(6);
                value.Serialize(serializer);
                serializer.decrease_container_depth();
            }

            internal static Winner Load(Serde.IDeserializer deserializer) {
                deserializer.increase_container_depth();
                Winner obj = new Winner(
                	Player.Deserialize(deserializer));
                deserializer.decrease_container_depth();
                return obj;
            }
            public override bool Equals(object obj) => obj is Winner other && Equals(other);

            public static bool operator ==(Winner left, Winner right) => Equals(left, right);

            public static bool operator !=(Winner left, Winner right) => !Equals(left, right);

            public bool Equals(Winner other) {
                if (other == null) return false;
                if (ReferenceEquals(this, other)) return true;
                if (!value.Equals(other.value)) return false;
                return true;
            }

            public override int GetHashCode() {
                unchecked {
                    int value = 7;
                    value = 31 * value + value.GetHashCode();
                    return value;
                }
            }

        }

        public sealed class Winners: Message, IEquatable<Winners>, ICloneable {
            public Serde.ValueArray<Player> players;
            public Serde.ValueArray<ulong> winner_indices;

            public Winners(Serde.ValueArray<Player> _players, Serde.ValueArray<ulong> _winner_indices) {
                if (_players == null) throw new ArgumentNullException(nameof(_players));
                players = _players;
                if (_winner_indices == null) throw new ArgumentNullException(nameof(_winner_indices));
                winner_indices = _winner_indices;
            }

            public override void Serialize(Serde.ISerializer serializer) {
                serializer.increase_container_depth();
                serializer.serialize_variant_index(7);
                TraitHelpers.serialize_vector_Player(players, serializer);
                TraitHelpers.serialize_vector_u64(winner_indices, serializer);
                serializer.decrease_container_depth();
            }

            internal static Winners Load(Serde.IDeserializer deserializer) {
                deserializer.increase_container_depth();
                Winners obj = new Winners(
                	TraitHelpers.deserialize_vector_Player(deserializer),
                	TraitHelpers.deserialize_vector_u64(deserializer));
                deserializer.decrease_container_depth();
                return obj;
            }
            public override bool Equals(object obj) => obj is Winners other && Equals(other);

            public static bool operator ==(Winners left, Winners right) => Equals(left, right);

            public static bool operator !=(Winners left, Winners right) => !Equals(left, right);

            public bool Equals(Winners other) {
                if (other == null) return false;
                if (ReferenceEquals(this, other)) return true;
                if (!players.Equals(other.players)) return false;
                if (!winner_indices.Equals(other.winner_indices)) return false;
                return true;
            }

            public override int GetHashCode() {
                unchecked {
                    int value = 7;
                    value = 31 * value + players.GetHashCode();
                    value = 31 * value + winner_indices.GetHashCode();
                    return value;
                }
            }

        }

        public sealed class RequestPlayerName: Message, IEquatable<RequestPlayerName>, ICloneable {
            public RequestPlayerName() {
            }

            public override void Serialize(Serde.ISerializer serializer) {
                serializer.increase_container_depth();
                serializer.serialize_variant_index(8);
                serializer.decrease_container_depth();
            }

            internal static RequestPlayerName Load(Serde.IDeserializer deserializer) {
                deserializer.increase_container_depth();
                RequestPlayerName obj = new RequestPlayerName(
                	);
                deserializer.decrease_container_depth();
                return obj;
            }
            public override bool Equals(object obj) => obj is RequestPlayerName other && Equals(other);

            public static bool operator ==(RequestPlayerName left, RequestPlayerName right) => Equals(left, right);

            public static bool operator !=(RequestPlayerName left, RequestPlayerName right) => !Equals(left, right);

            public bool Equals(RequestPlayerName other) {
                if (other == null) return false;
                if (ReferenceEquals(this, other)) return true;
                return true;
            }

            public override int GetHashCode() {
                unchecked {
                    int value = 7;
                    return value;
                }
            }

        }

        public sealed class PlayRequest: Message, IEquatable<PlayRequest>, ICloneable {
            public Player value;

            public PlayRequest(Player _value) {
                if (_value == null) throw new ArgumentNullException(nameof(_value));
                value = _value;
            }

            public override void Serialize(Serde.ISerializer serializer) {
                serializer.increase_container_depth();
                serializer.serialize_variant_index(9);
                value.Serialize(serializer);
                serializer.decrease_container_depth();
            }

            internal static PlayRequest Load(Serde.IDeserializer deserializer) {
                deserializer.increase_container_depth();
                PlayRequest obj = new PlayRequest(
                	Player.Deserialize(deserializer));
                deserializer.decrease_container_depth();
                return obj;
            }
            public override bool Equals(object obj) => obj is PlayRequest other && Equals(other);

            public static bool operator ==(PlayRequest left, PlayRequest right) => Equals(left, right);

            public static bool operator !=(PlayRequest left, PlayRequest right) => !Equals(left, right);

            public bool Equals(PlayRequest other) {
                if (other == null) return false;
                if (ReferenceEquals(this, other)) return true;
                if (!value.Equals(other.value)) return false;
                return true;
            }

            public override int GetHashCode() {
                unchecked {
                    int value = 7;
                    value = 31 * value + value.GetHashCode();
                    return value;
                }
            }

        }

        public sealed class RequestGuess: Message, IEquatable<RequestGuess>, ICloneable {
            public RequestGuess() {
            }

            public override void Serialize(Serde.ISerializer serializer) {
                serializer.increase_container_depth();
                serializer.serialize_variant_index(10);
                serializer.decrease_container_depth();
            }

            internal static RequestGuess Load(Serde.IDeserializer deserializer) {
                deserializer.increase_container_depth();
                RequestGuess obj = new RequestGuess(
                	);
                deserializer.decrease_container_depth();
                return obj;
            }
            public override bool Equals(object obj) => obj is RequestGuess other && Equals(other);

            public static bool operator ==(RequestGuess left, RequestGuess right) => Equals(left, right);

            public static bool operator !=(RequestGuess left, RequestGuess right) => !Equals(left, right);

            public bool Equals(RequestGuess other) {
                if (other == null) return false;
                if (ReferenceEquals(this, other)) return true;
                return true;
            }

            public override int GetHashCode() {
                unchecked {
                    int value = 7;
                    return value;
                }
            }

        }

        public sealed class GameOver: Message, IEquatable<GameOver>, ICloneable {
            public GameOver() {
            }

            public override void Serialize(Serde.ISerializer serializer) {
                serializer.increase_container_depth();
                serializer.serialize_variant_index(11);
                serializer.decrease_container_depth();
            }

            internal static GameOver Load(Serde.IDeserializer deserializer) {
                deserializer.increase_container_depth();
                GameOver obj = new GameOver(
                	);
                deserializer.decrease_container_depth();
                return obj;
            }
            public override bool Equals(object obj) => obj is GameOver other && Equals(other);

            public static bool operator ==(GameOver left, GameOver right) => Equals(left, right);

            public static bool operator !=(GameOver left, GameOver right) => !Equals(left, right);

            public bool Equals(GameOver other) {
                if (other == null) return false;
                if (ReferenceEquals(this, other)) return true;
                return true;
            }

            public override int GetHashCode() {
                unchecked {
                    int value = 7;
                    return value;
                }
            }

        }
    }


} // end of namespace plump_message
