package plump_message;


public final class PublicState {
    public final java.util.Optional<@com.novi.serde.Unsigned Long> guess;
    public final @com.novi.serde.Unsigned Long wins;
    public final @com.novi.serde.Unsigned Long score;

    public PublicState(java.util.Optional<@com.novi.serde.Unsigned Long> guess, @com.novi.serde.Unsigned Long wins, @com.novi.serde.Unsigned Long score) {
        java.util.Objects.requireNonNull(guess, "guess must not be null");
        java.util.Objects.requireNonNull(wins, "wins must not be null");
        java.util.Objects.requireNonNull(score, "score must not be null");
        this.guess = guess;
        this.wins = wins;
        this.score = score;
    }

    public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
        serializer.increase_container_depth();
        TraitHelpers.serialize_option_u64(guess, serializer);
        serializer.serialize_u64(wins);
        serializer.serialize_u64(score);
        serializer.decrease_container_depth();
    }

    public byte[] bincodeSerialize() throws com.novi.serde.SerializationError {
        com.novi.serde.Serializer serializer = new com.novi.bincode.BincodeSerializer();
        serialize(serializer);
        return serializer.get_bytes();
    }

    public static PublicState deserialize(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        deserializer.increase_container_depth();
        Builder builder = new Builder();
        builder.guess = TraitHelpers.deserialize_option_u64(deserializer);
        builder.wins = deserializer.deserialize_u64();
        builder.score = deserializer.deserialize_u64();
        deserializer.decrease_container_depth();
        return builder.build();
    }

    public static PublicState bincodeDeserialize(byte[] input) throws com.novi.serde.DeserializationError {
        if (input == null) {
             throw new com.novi.serde.DeserializationError("Cannot deserialize null array");
        }
        com.novi.serde.Deserializer deserializer = new com.novi.bincode.BincodeDeserializer(input);
        PublicState value = deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.length) {
             throw new com.novi.serde.DeserializationError("Some input bytes were not read");
        }
        return value;
    }

    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null) return false;
        if (getClass() != obj.getClass()) return false;
        PublicState other = (PublicState) obj;
        if (!java.util.Objects.equals(this.guess, other.guess)) { return false; }
        if (!java.util.Objects.equals(this.wins, other.wins)) { return false; }
        if (!java.util.Objects.equals(this.score, other.score)) { return false; }
        return true;
    }

    public int hashCode() {
        int value = 7;
        value = 31 * value + (this.guess != null ? this.guess.hashCode() : 0);
        value = 31 * value + (this.wins != null ? this.wins.hashCode() : 0);
        value = 31 * value + (this.score != null ? this.score.hashCode() : 0);
        return value;
    }

    public static final class Builder {
        public java.util.Optional<@com.novi.serde.Unsigned Long> guess;
        public @com.novi.serde.Unsigned Long wins;
        public @com.novi.serde.Unsigned Long score;

        public PublicState build() {
            return new PublicState(
                guess,
                wins,
                score
            );
        }
    }
}
