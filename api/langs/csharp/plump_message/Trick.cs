using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Numerics;

namespace plump_message {

    public sealed class Trick: IEquatable<Trick>, ICloneable {
        public Serde.ValueArray<Card> value;

        public Trick(Serde.ValueArray<Card> _value) {
            if (_value == null) throw new ArgumentNullException(nameof(_value));
            value = _value;
        }

        public void Serialize(Serde.ISerializer serializer) {
            serializer.increase_container_depth();
            TraitHelpers.serialize_vector_Card(value, serializer);
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

        public static Trick Deserialize(Serde.IDeserializer deserializer) {
            deserializer.increase_container_depth();
            Trick obj = new Trick(
            	TraitHelpers.deserialize_vector_Card(deserializer));
            deserializer.decrease_container_depth();
            return obj;
        }

        public static Trick BincodeDeserialize(byte[] input) => BincodeDeserialize(new ArraySegment<byte>(input));

        public static Trick BincodeDeserialize(ArraySegment<byte> input) {
            if (input == null) {
                 throw new Serde.DeserializationException("Cannot deserialize null array");
            }
            Serde.IDeserializer deserializer = new Bincode.BincodeDeserializer(input);
            Trick value = Deserialize(deserializer);
            if (deserializer.get_buffer_offset() < input.Count) {
                 throw new Serde.DeserializationException("Some input bytes were not read");
            }
            return value;
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

        /// <summary>Creates a shallow clone of the object.</summary>
        public Trick Clone() => (Trick)MemberwiseClone();

        object ICloneable.Clone() => Clone();

    }

} // end of namespace plump_message
