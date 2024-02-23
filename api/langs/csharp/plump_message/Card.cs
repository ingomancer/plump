using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Numerics;

namespace plump_message {

    public sealed class Card: IEquatable<Card>, ICloneable {
        public ulong suit;
        public ulong value;

        public Card(ulong _suit, ulong _value) {
            suit = _suit;
            value = _value;
        }

        public void Serialize(Serde.ISerializer serializer) {
            serializer.increase_container_depth();
            serializer.serialize_u64(suit);
            serializer.serialize_u64(value);
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

        public static Card Deserialize(Serde.IDeserializer deserializer) {
            deserializer.increase_container_depth();
            Card obj = new Card(
            	deserializer.deserialize_u64(),
            	deserializer.deserialize_u64());
            deserializer.decrease_container_depth();
            return obj;
        }

        public static Card BincodeDeserialize(byte[] input) => BincodeDeserialize(new ArraySegment<byte>(input));

        public static Card BincodeDeserialize(ArraySegment<byte> input) {
            if (input == null) {
                 throw new Serde.DeserializationException("Cannot deserialize null array");
            }
            Serde.IDeserializer deserializer = new Bincode.BincodeDeserializer(input);
            Card value = Deserialize(deserializer);
            if (deserializer.get_buffer_offset() < input.Count) {
                 throw new Serde.DeserializationException("Some input bytes were not read");
            }
            return value;
        }
        public override bool Equals(object obj) => obj is Card other && Equals(other);

        public static bool operator ==(Card left, Card right) => Equals(left, right);

        public static bool operator !=(Card left, Card right) => !Equals(left, right);

        public bool Equals(Card other) {
            if (other == null) return false;
            if (ReferenceEquals(this, other)) return true;
            if (!suit.Equals(other.suit)) return false;
            if (!value.Equals(other.value)) return false;
            return true;
        }

        public override int GetHashCode() {
            unchecked {
                int value = 7;
                value = 31 * value + suit.GetHashCode();
                value = 31 * value + value.GetHashCode();
                return value;
            }
        }

        /// <summary>Creates a shallow clone of the object.</summary>
        public Card Clone() => (Card)MemberwiseClone();

        object ICloneable.Clone() => Clone();

    }

} // end of namespace plump_message
