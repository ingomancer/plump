package plump_message;

final class TraitHelpers {
    static void serialize_map_PlayerName_to_PublicState(java.util.Map<PlayerName, PublicState> value, com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.serialize_len(value.size());
        int[] offsets = new int[value.size()];
        int count = 0;
        for (java.util.Map.Entry<PlayerName, PublicState> entry : value.entrySet()) {
            offsets[count++] = serializer.get_buffer_offset();
            entry.getKey().serialize(serializer);
            entry.getValue().serialize(serializer);
        }
        serializer.sort_map_entries(offsets);
    }

    static java.util.Map<PlayerName, PublicState> deserialize_map_PlayerName_to_PublicState(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        long length = deserializer.deserialize_len();
        java.util.Map<PlayerName, PublicState> obj = new java.util.HashMap<PlayerName, PublicState>();
        int previous_key_start = 0;
        int previous_key_end = 0;
        for (long i = 0; i < length; i++) {
            int key_start = deserializer.get_buffer_offset();
            PlayerName key = PlayerName.deserialize(deserializer);
            int key_end = deserializer.get_buffer_offset();
            if (i > 0) {
                deserializer.check_that_key_slices_are_increasing(
                    new com.novi.serde.Slice(previous_key_start, previous_key_end),
                    new com.novi.serde.Slice(key_start, key_end));
            }
            previous_key_start = key_start;
            previous_key_end = key_end;
            PublicState value = PublicState.deserialize(deserializer);
            obj.put(key, value);
        }
        return obj;
    }

    static void serialize_option_u64(java.util.Optional<@com.novi.serde.Unsigned Long> value, com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        if (value.isPresent()) {
            serializer.serialize_option_tag(true);
            serializer.serialize_u64(value.get());
        } else {
            serializer.serialize_option_tag(false);
        }
    }

    static java.util.Optional<@com.novi.serde.Unsigned Long> deserialize_option_u64(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        boolean tag = deserializer.deserialize_option_tag();
        if (!tag) {
            return java.util.Optional.empty();
        } else {
            return java.util.Optional.of(deserializer.deserialize_u64());
        }
    }

    static void serialize_option_vector_u64(java.util.Optional<java.util.List<@com.novi.serde.Unsigned Long>> value, com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        if (value.isPresent()) {
            serializer.serialize_option_tag(true);
            TraitHelpers.serialize_vector_u64(value.get(), serializer);
        } else {
            serializer.serialize_option_tag(false);
        }
    }

    static java.util.Optional<java.util.List<@com.novi.serde.Unsigned Long>> deserialize_option_vector_u64(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        boolean tag = deserializer.deserialize_option_tag();
        if (!tag) {
            return java.util.Optional.empty();
        } else {
            return java.util.Optional.of(TraitHelpers.deserialize_vector_u64(deserializer));
        }
    }

    static void serialize_vector_Card(java.util.List<Card> value, com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.serialize_len(value.size());
        for (Card item : value) {
            item.serialize(serializer);
        }
    }

    static java.util.List<Card> deserialize_vector_Card(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        long length = deserializer.deserialize_len();
        java.util.List<Card> obj = new java.util.ArrayList<Card>((int) length);
        for (long i = 0; i < length; i++) {
            obj.add(Card.deserialize(deserializer));
        }
        return obj;
    }

    static void serialize_vector_Player(java.util.List<Player> value, com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.serialize_len(value.size());
        for (Player item : value) {
            item.serialize(serializer);
        }
    }

    static java.util.List<Player> deserialize_vector_Player(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        long length = deserializer.deserialize_len();
        java.util.List<Player> obj = new java.util.ArrayList<Player>((int) length);
        for (long i = 0; i < length; i++) {
            obj.add(Player.deserialize(deserializer));
        }
        return obj;
    }

    static void serialize_vector_u64(java.util.List<@com.novi.serde.Unsigned Long> value, com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.serialize_len(value.size());
        for (@com.novi.serde.Unsigned Long item : value) {
            serializer.serialize_u64(item);
        }
    }

    static java.util.List<@com.novi.serde.Unsigned Long> deserialize_vector_u64(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        long length = deserializer.deserialize_len();
        java.util.List<@com.novi.serde.Unsigned Long> obj = new java.util.ArrayList<@com.novi.serde.Unsigned Long>((int) length);
        for (long i = 0; i < length; i++) {
            obj.add(deserializer.deserialize_u64());
        }
        return obj;
    }

}

