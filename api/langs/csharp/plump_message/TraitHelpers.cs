using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Numerics;

namespace plump_message {
    static class TraitHelpers {
        public static void serialize_map_PlayerName_to_PublicState(Serde.ValueDictionary<PlayerName, PublicState> value, Serde.ISerializer serializer) {
            serializer.serialize_len(value.Count);
            int[] offsets = new int[value.Count];
            int count = 0;
            foreach (KeyValuePair<PlayerName, PublicState> entry in value) {
                offsets[count++] = serializer.get_buffer_offset();
                entry.Key.Serialize(serializer);
                entry.Value.Serialize(serializer);
            }
            serializer.sort_map_entries(offsets);
        }

        public static Serde.ValueDictionary<PlayerName, PublicState> deserialize_map_PlayerName_to_PublicState(Serde.IDeserializer deserializer) {
            long length = deserializer.deserialize_len();
            var obj = new Dictionary<PlayerName, PublicState>();
            int previous_key_start = 0;
            int previous_key_end = 0;
            for (long i = 0; i < length; i++) {
                int key_start = deserializer.get_buffer_offset();
                var key = PlayerName.Deserialize(deserializer);
                int key_end = deserializer.get_buffer_offset();
                if (i > 0) {
                    deserializer.check_that_key_slices_are_increasing(
                        new Serde.Range(previous_key_start, previous_key_end),
                        new Serde.Range(key_start, key_end));
                }
                previous_key_start = key_start;
                previous_key_end = key_end;
                var value = PublicState.Deserialize(deserializer);
                obj[key] = value;
            }
            return new Serde.ValueDictionary<PlayerName, PublicState>(obj);
        }

        public static void serialize_option_u64(Serde.Option<ulong> value, Serde.ISerializer serializer) {
            if (value.IsSome(out var val)) {
                serializer.serialize_option_tag(true);
                serializer.serialize_u64(val);
            } else {
                serializer.serialize_option_tag(false);
            }
        }

        public static Serde.Option<ulong> deserialize_option_u64(Serde.IDeserializer deserializer) {
            bool tag = deserializer.deserialize_option_tag();
            if (!tag) {
                return Serde.Option<ulong>.None;
            } else {
                return Serde.Option<ulong>.Some(deserializer.deserialize_u64());
            }
        }

        public static void serialize_option_vector_u64(Serde.Option<Serde.ValueArray<ulong>> value, Serde.ISerializer serializer) {
            if (value.IsSome(out var val)) {
                serializer.serialize_option_tag(true);
                TraitHelpers.serialize_vector_u64(val, serializer);
            } else {
                serializer.serialize_option_tag(false);
            }
        }

        public static Serde.Option<Serde.ValueArray<ulong>> deserialize_option_vector_u64(Serde.IDeserializer deserializer) {
            bool tag = deserializer.deserialize_option_tag();
            if (!tag) {
                return Serde.Option<Serde.ValueArray<ulong>>.None;
            } else {
                return Serde.Option<Serde.ValueArray<ulong>>.Some(TraitHelpers.deserialize_vector_u64(deserializer));
            }
        }

        public static void serialize_vector_Card(Serde.ValueArray<Card> value, Serde.ISerializer serializer) {
            serializer.serialize_len(value.Count);
            foreach (var item in value) {
                item.Serialize(serializer);
            }
        }

        public static Serde.ValueArray<Card> deserialize_vector_Card(Serde.IDeserializer deserializer) {
            long length = deserializer.deserialize_len();
            Card[] obj = new Card[length];
            for (int i = 0; i < length; i++) {
                obj[i] = Card.Deserialize(deserializer);
            }
            return new Serde.ValueArray<Card>(obj);
        }

        public static void serialize_vector_Player(Serde.ValueArray<Player> value, Serde.ISerializer serializer) {
            serializer.serialize_len(value.Count);
            foreach (var item in value) {
                item.Serialize(serializer);
            }
        }

        public static Serde.ValueArray<Player> deserialize_vector_Player(Serde.IDeserializer deserializer) {
            long length = deserializer.deserialize_len();
            Player[] obj = new Player[length];
            for (int i = 0; i < length; i++) {
                obj[i] = Player.Deserialize(deserializer);
            }
            return new Serde.ValueArray<Player>(obj);
        }

        public static void serialize_vector_u64(Serde.ValueArray<ulong> value, Serde.ISerializer serializer) {
            serializer.serialize_len(value.Count);
            foreach (var item in value) {
                serializer.serialize_u64(item);
            }
        }

        public static Serde.ValueArray<ulong> deserialize_vector_u64(Serde.IDeserializer deserializer) {
            long length = deserializer.deserialize_len();
            ulong[] obj = new ulong[length];
            for (int i = 0; i < length; i++) {
                obj[i] = deserializer.deserialize_u64();
            }
            return new Serde.ValueArray<ulong>(obj);
        }

    }


} // end of namespace plump_message
