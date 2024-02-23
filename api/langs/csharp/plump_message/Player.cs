using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Numerics;

namespace plump_message {

    public sealed class Player: IEquatable<Player>, ICloneable {
        public PlayerName name;
        public bool human;
        public Serde.ValueArray<Card> hand;

        public Player(PlayerName _name, bool _human, Serde.ValueArray<Card> _hand) {
            if (_name == null) throw new ArgumentNullException(nameof(_name));
            name = _name;
            human = _human;
            if (_hand == null) throw new ArgumentNullException(nameof(_hand));
            hand = _hand;
        }

        public void Serialize(Serde.ISerializer serializer) {
            serializer.increase_container_depth();
            name.Serialize(serializer);
            serializer.serialize_bool(human);
            TraitHelpers.serialize_vector_Card(hand, serializer);
            serializer.decrease_container_depth();
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

        public static Player Deserialize(Serde.IDeserializer deserializer) {
            deserializer.increase_container_depth();
            Player obj = new Player(
            	PlayerName.Deserialize(deserializer),
            	deserializer.deserialize_bool(),
            	TraitHelpers.deserialize_vector_Card(deserializer));
            deserializer.decrease_container_depth();
            return obj;
        }

        public static Player BincodeDeserialize(byte[] input) => BincodeDeserialize(new ArraySegment<byte>(input));

        public static Player BincodeDeserialize(ArraySegment<byte> input) {
            if (input == null) {
                 throw new Serde.DeserializationException("Cannot deserialize null array");
            }
            Serde.IDeserializer deserializer = new Bincode.BincodeDeserializer(input);
            Player value = Deserialize(deserializer);
            if (deserializer.get_buffer_offset() < input.Count) {
                 throw new Serde.DeserializationException("Some input bytes were not read");
            }
            return value;
        }
        public override bool Equals(object obj) => obj is Player other && Equals(other);

        public static bool operator ==(Player left, Player right) => Equals(left, right);

        public static bool operator !=(Player left, Player right) => !Equals(left, right);

        public bool Equals(Player other) {
            if (other == null) return false;
            if (ReferenceEquals(this, other)) return true;
            if (!name.Equals(other.name)) return false;
            if (!human.Equals(other.human)) return false;
            if (!hand.Equals(other.hand)) return false;
            return true;
        }

        public override int GetHashCode() {
            unchecked {
                int value = 7;
                value = 31 * value + name.GetHashCode();
                value = 31 * value + human.GetHashCode();
                value = 31 * value + hand.GetHashCode();
                return value;
            }
        }

        /// <summary>Creates a shallow clone of the object.</summary>
        public Player Clone() => (Player)MemberwiseClone();

        object ICloneable.Clone() => Clone();

    }

} // end of namespace plump_message
