using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Numerics;

namespace plump_message {

    public sealed class PublicState: IEquatable<PublicState>, ICloneable {
        public Serde.Option<ulong> guess;
        public ulong wins;
        public ulong score;

        public PublicState(Serde.Option<ulong> _guess, ulong _wins, ulong _score) {
            guess = _guess;
            wins = _wins;
            score = _score;
        }

        public void Serialize(Serde.ISerializer serializer) {
            serializer.increase_container_depth();
            TraitHelpers.serialize_option_u64(guess, serializer);
            serializer.serialize_u64(wins);
            serializer.serialize_u64(score);
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

        public static PublicState Deserialize(Serde.IDeserializer deserializer) {
            deserializer.increase_container_depth();
            PublicState obj = new PublicState(
            	TraitHelpers.deserialize_option_u64(deserializer),
            	deserializer.deserialize_u64(),
            	deserializer.deserialize_u64());
            deserializer.decrease_container_depth();
            return obj;
        }

        public static PublicState BincodeDeserialize(byte[] input) => BincodeDeserialize(new ArraySegment<byte>(input));

        public static PublicState BincodeDeserialize(ArraySegment<byte> input) {
            if (input == null) {
                 throw new Serde.DeserializationException("Cannot deserialize null array");
            }
            Serde.IDeserializer deserializer = new Bincode.BincodeDeserializer(input);
            PublicState value = Deserialize(deserializer);
            if (deserializer.get_buffer_offset() < input.Count) {
                 throw new Serde.DeserializationException("Some input bytes were not read");
            }
            return value;
        }
        public override bool Equals(object obj) => obj is PublicState other && Equals(other);

        public static bool operator ==(PublicState left, PublicState right) => Equals(left, right);

        public static bool operator !=(PublicState left, PublicState right) => !Equals(left, right);

        public bool Equals(PublicState other) {
            if (other == null) return false;
            if (ReferenceEquals(this, other)) return true;
            if (!guess.Equals(other.guess)) return false;
            if (!wins.Equals(other.wins)) return false;
            if (!score.Equals(other.score)) return false;
            return true;
        }

        public override int GetHashCode() {
            unchecked {
                int value = 7;
                value = 31 * value + guess.GetHashCode();
                value = 31 * value + wins.GetHashCode();
                value = 31 * value + score.GetHashCode();
                return value;
            }
        }

        /// <summary>Creates a shallow clone of the object.</summary>
        public PublicState Clone() => (PublicState)MemberwiseClone();

        object ICloneable.Clone() => Clone();

    }

} // end of namespace plump_message
