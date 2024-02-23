package plump_message;


public final class Player {
    public final PlayerName name;
    public final Boolean human;
    public final java.util.List<Card> hand;

    public Player(PlayerName name, Boolean human, java.util.List<Card> hand) {
        java.util.Objects.requireNonNull(name, "name must not be null");
        java.util.Objects.requireNonNull(human, "human must not be null");
        java.util.Objects.requireNonNull(hand, "hand must not be null");
        this.name = name;
        this.human = human;
        this.hand = hand;
    }

    public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.increase_container_depth();
        name.serialize(serializer);
        serializer.serialize_bool(human);
        TraitHelpers.serialize_vector_Card(hand, serializer);
        serializer.decrease_container_depth();
    }

    public byte[] bincodeSerialize() throws com.novi.serde.SerializationError {
        com.novi.serde.Serializer serializer = new com.novi.bincode.BincodeSerializer();
        serialize(serializer);
        return serializer.get_bytes();
    }

    public static Player deserialize(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        deserializer.increase_container_depth();
        Builder builder = new Builder();
        builder.name = PlayerName.deserialize(deserializer);
        builder.human = deserializer.deserialize_bool();
        builder.hand = TraitHelpers.deserialize_vector_Card(deserializer);
        deserializer.decrease_container_depth();
        return builder.build();
    }

    public static Player bincodeDeserialize(byte[] input) throws com.novi.serde.DeserializationError {
        if (input == null) {
             throw new com.novi.serde.DeserializationError("Cannot deserialize null array");
        }
        com.novi.serde.Deserializer deserializer = new com.novi.bincode.BincodeDeserializer(input);
        Player value = deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.length) {
             throw new com.novi.serde.DeserializationError("Some input bytes were not read");
        }
        return value;
    }

    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null) return false;
        if (getClass() != obj.getClass()) return false;
        Player other = (Player) obj;
        if (!java.util.Objects.equals(this.name, other.name)) { return false; }
        if (!java.util.Objects.equals(this.human, other.human)) { return false; }
        if (!java.util.Objects.equals(this.hand, other.hand)) { return false; }
        return true;
    }

    public int hashCode() {
        int value = 7;
        value = 31 * value + (this.name != null ? this.name.hashCode() : 0);
        value = 31 * value + (this.human != null ? this.human.hashCode() : 0);
        value = 31 * value + (this.hand != null ? this.hand.hashCode() : 0);
        return value;
    }

    public static final class Builder {
        public PlayerName name;
        public Boolean human;
        public java.util.List<Card> hand;

        public Player build() {
            return new Player(
                name,
                human,
                hand
            );
        }
    }
}
