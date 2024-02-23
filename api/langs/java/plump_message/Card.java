package plump_message;


public final class Card {
    public final @com.novi.serde.Unsigned Long suit;
    public final @com.novi.serde.Unsigned Long value;

    public Card(@com.novi.serde.Unsigned Long suit, @com.novi.serde.Unsigned Long value) {
        java.util.Objects.requireNonNull(suit, "suit must not be null");
        java.util.Objects.requireNonNull(value, "value must not be null");
        this.suit = suit;
        this.value = value;
    }

    public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.increase_container_depth();
        serializer.serialize_u64(suit);
        serializer.serialize_u64(value);
        serializer.decrease_container_depth();
    }

    public byte[] bincodeSerialize() throws com.novi.serde.SerializationError {
        com.novi.serde.Serializer serializer = new com.novi.bincode.BincodeSerializer();
        serialize(serializer);
        return serializer.get_bytes();
    }

    public static Card deserialize(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        deserializer.increase_container_depth();
        Builder builder = new Builder();
        builder.suit = deserializer.deserialize_u64();
        builder.value = deserializer.deserialize_u64();
        deserializer.decrease_container_depth();
        return builder.build();
    }

    public static Card bincodeDeserialize(byte[] input) throws com.novi.serde.DeserializationError {
        if (input == null) {
             throw new com.novi.serde.DeserializationError("Cannot deserialize null array");
        }
        com.novi.serde.Deserializer deserializer = new com.novi.bincode.BincodeDeserializer(input);
        Card value = deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.length) {
             throw new com.novi.serde.DeserializationError("Some input bytes were not read");
        }
        return value;
    }

    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null) return false;
        if (getClass() != obj.getClass()) return false;
        Card other = (Card) obj;
        if (!java.util.Objects.equals(this.suit, other.suit)) { return false; }
        if (!java.util.Objects.equals(this.value, other.value)) { return false; }
        return true;
    }

    public int hashCode() {
        int value = 7;
        value = 31 * value + (this.suit != null ? this.suit.hashCode() : 0);
        value = 31 * value + (this.value != null ? this.value.hashCode() : 0);
        return value;
    }

    public static final class Builder {
        public @com.novi.serde.Unsigned Long suit;
        public @com.novi.serde.Unsigned Long value;

        public Card build() {
            return new Card(
                suit,
                value
            );
        }
    }
}
