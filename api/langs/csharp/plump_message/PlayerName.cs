using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Numerics;

namespace plump_message {

    public sealed class PlayerName: IEquatable<PlayerName>, ICloneable {
        public string value;

        public PlayerName(string _value) {
            if (_value == null) throw new ArgumentNullException(nameof(_value));
            value = _value;
        }

        public void Serialize(Serde.ISerializer serializer) {
            serializer.increase_container_depth();
            serializer.serialize_str(value);
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

        public static PlayerName Deserialize(Serde.IDeserializer deserializer) {
            deserializer.increase_container_depth();
            PlayerName obj = new PlayerName(
            	deserializer.deserialize_str());
            deserializer.decrease_container_depth();
            return obj;
        }

        public static PlayerName BincodeDeserialize(byte[] input) => BincodeDeserialize(new ArraySegment<byte>(input));

        public static PlayerName BincodeDeserialize(ArraySegment<byte> input) {
            if (input == null) {
                 throw new Serde.DeserializationException("Cannot deserialize null array");
            }
            Serde.IDeserializer deserializer = new Bincode.BincodeDeserializer(input);
            PlayerName value = Deserialize(deserializer);
            if (deserializer.get_buffer_offset() < input.Count) {
                 throw new Serde.DeserializationException("Some input bytes were not read");
            }
            return value;
        }
        public override bool Equals(object obj) => obj is PlayerName other && Equals(other);

        public static bool operator ==(PlayerName left, PlayerName right) => Equals(left, right);

        public static bool operator !=(PlayerName left, PlayerName right) => !Equals(left, right);

        public bool Equals(PlayerName other) {
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
        public PlayerName Clone() => (PlayerName)MemberwiseClone();

        object ICloneable.Clone() => Clone();

    }

} // end of namespace plump_message
