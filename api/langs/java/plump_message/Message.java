package plump_message;


public abstract class Message {

    abstract public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError;

    public static Message deserialize(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
        int index = deserializer.deserialize_variant_index();
        switch (index) {
            case 0: return RequestGuessContext.load(deserializer);
            case 1: return Guesses.load(deserializer);
            case 2: return Turn.load(deserializer);
            case 3: return PlayRequestContext.load(deserializer);
            case 4: return Trick.load(deserializer);
            case 5: return Scoreboard.load(deserializer);
            case 6: return Winner.load(deserializer);
            case 7: return Winners.load(deserializer);
            case 8: return RequestPlayerName.load(deserializer);
            case 9: return PlayRequest.load(deserializer);
            case 10: return RequestGuess.load(deserializer);
            case 11: return GameOver.load(deserializer);
            default: throw new com.novi.serde.DeserializationError("Unknown variant index for Message: " + index);
        }
    }

    public byte[] bincodeSerialize() throws com.novi.serde.SerializationError {
        com.novi.serde.Serializer serializer = new com.novi.bincode.BincodeSerializer();
        serialize(serializer);
        return serializer.get_bytes();
    }

    public static Message bincodeDeserialize(byte[] input) throws com.novi.serde.DeserializationError {
        if (input == null) {
             throw new com.novi.serde.DeserializationError("Cannot deserialize null array");
        }
        com.novi.serde.Deserializer deserializer = new com.novi.bincode.BincodeDeserializer(input);
        Message value = deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.length) {
             throw new com.novi.serde.DeserializationError("Some input bytes were not read");
        }
        return value;
    }

    public static final class RequestGuessContext extends Message {
        public final Player player;
        public final java.util.List<Card> hand;
        public final java.util.List<@com.novi.serde.Unsigned Long> guesses;
        public final @com.novi.serde.Unsigned Long players;

        public RequestGuessContext(Player player, java.util.List<Card> hand, java.util.List<@com.novi.serde.Unsigned Long> guesses, @com.novi.serde.Unsigned Long players) {
            java.util.Objects.requireNonNull(player, "player must not be null");
            java.util.Objects.requireNonNull(hand, "hand must not be null");
            java.util.Objects.requireNonNull(guesses, "guesses must not be null");
            java.util.Objects.requireNonNull(players, "players must not be null");
            this.player = player;
            this.hand = hand;
            this.guesses = guesses;
            this.players = players;
        }

        public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
            serializer.increase_container_depth();
            serializer.serialize_variant_index(0);
            player.serialize(serializer);
            TraitHelpers.serialize_vector_Card(hand, serializer);
            TraitHelpers.serialize_vector_u64(guesses, serializer);
            serializer.serialize_u64(players);
            serializer.decrease_container_depth();
        }

        static RequestGuessContext load(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
            deserializer.increase_container_depth();
            Builder builder = new Builder();
            builder.player = Player.deserialize(deserializer);
            builder.hand = TraitHelpers.deserialize_vector_Card(deserializer);
            builder.guesses = TraitHelpers.deserialize_vector_u64(deserializer);
            builder.players = deserializer.deserialize_u64();
            deserializer.decrease_container_depth();
            return builder.build();
        }

        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null) return false;
            if (getClass() != obj.getClass()) return false;
            RequestGuessContext other = (RequestGuessContext) obj;
            if (!java.util.Objects.equals(this.player, other.player)) { return false; }
            if (!java.util.Objects.equals(this.hand, other.hand)) { return false; }
            if (!java.util.Objects.equals(this.guesses, other.guesses)) { return false; }
            if (!java.util.Objects.equals(this.players, other.players)) { return false; }
            return true;
        }

        public int hashCode() {
            int value = 7;
            value = 31 * value + (this.player != null ? this.player.hashCode() : 0);
            value = 31 * value + (this.hand != null ? this.hand.hashCode() : 0);
            value = 31 * value + (this.guesses != null ? this.guesses.hashCode() : 0);
            value = 31 * value + (this.players != null ? this.players.hashCode() : 0);
            return value;
        }

        public static final class Builder {
            public Player player;
            public java.util.List<Card> hand;
            public java.util.List<@com.novi.serde.Unsigned Long> guesses;
            public @com.novi.serde.Unsigned Long players;

            public RequestGuessContext build() {
                return new RequestGuessContext(
                    player,
                    hand,
                    guesses,
                    players
                );
            }
        }
    }

    public static final class Guesses extends Message {
        public final java.util.Map<PlayerName, PublicState> state;

        public Guesses(java.util.Map<PlayerName, PublicState> state) {
            java.util.Objects.requireNonNull(state, "state must not be null");
            this.state = state;
        }

        public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
            serializer.increase_container_depth();
            serializer.serialize_variant_index(1);
            TraitHelpers.serialize_map_PlayerName_to_PublicState(state, serializer);
            serializer.decrease_container_depth();
        }

        static Guesses load(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
            deserializer.increase_container_depth();
            Builder builder = new Builder();
            builder.state = TraitHelpers.deserialize_map_PlayerName_to_PublicState(deserializer);
            deserializer.decrease_container_depth();
            return builder.build();
        }

        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null) return false;
            if (getClass() != obj.getClass()) return false;
            Guesses other = (Guesses) obj;
            if (!java.util.Objects.equals(this.state, other.state)) { return false; }
            return true;
        }

        public int hashCode() {
            int value = 7;
            value = 31 * value + (this.state != null ? this.state.hashCode() : 0);
            return value;
        }

        public static final class Builder {
            public java.util.Map<PlayerName, PublicState> state;

            public Guesses build() {
                return new Guesses(
                    state
                );
            }
        }
    }

    public static final class Turn extends Message {
        public final Player whose;

        public Turn(Player whose) {
            java.util.Objects.requireNonNull(whose, "whose must not be null");
            this.whose = whose;
        }

        public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
            serializer.increase_container_depth();
            serializer.serialize_variant_index(2);
            whose.serialize(serializer);
            serializer.decrease_container_depth();
        }

        static Turn load(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
            deserializer.increase_container_depth();
            Builder builder = new Builder();
            builder.whose = Player.deserialize(deserializer);
            deserializer.decrease_container_depth();
            return builder.build();
        }

        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null) return false;
            if (getClass() != obj.getClass()) return false;
            Turn other = (Turn) obj;
            if (!java.util.Objects.equals(this.whose, other.whose)) { return false; }
            return true;
        }

        public int hashCode() {
            int value = 7;
            value = 31 * value + (this.whose != null ? this.whose.hashCode() : 0);
            return value;
        }

        public static final class Builder {
            public Player whose;

            public Turn build() {
                return new Turn(
                    whose
                );
            }
        }
    }

    public static final class PlayRequestContext extends Message {
        public final Player player;
        public final java.util.List<Card> hand;
        public final plump_message.Trick trick;
        public final java.util.Optional<java.util.List<@com.novi.serde.Unsigned Long>> valid_cards;

        public PlayRequestContext(Player player, java.util.List<Card> hand, plump_message.Trick trick, java.util.Optional<java.util.List<@com.novi.serde.Unsigned Long>> valid_cards) {
            java.util.Objects.requireNonNull(player, "player must not be null");
            java.util.Objects.requireNonNull(hand, "hand must not be null");
            java.util.Objects.requireNonNull(trick, "trick must not be null");
            java.util.Objects.requireNonNull(valid_cards, "valid_cards must not be null");
            this.player = player;
            this.hand = hand;
            this.trick = trick;
            this.valid_cards = valid_cards;
        }

        public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
            serializer.increase_container_depth();
            serializer.serialize_variant_index(3);
            player.serialize(serializer);
            TraitHelpers.serialize_vector_Card(hand, serializer);
            trick.serialize(serializer);
            TraitHelpers.serialize_option_vector_u64(valid_cards, serializer);
            serializer.decrease_container_depth();
        }

        static PlayRequestContext load(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
            deserializer.increase_container_depth();
            Builder builder = new Builder();
            builder.player = Player.deserialize(deserializer);
            builder.hand = TraitHelpers.deserialize_vector_Card(deserializer);
            builder.trick = plump_message.Trick.deserialize(deserializer);
            builder.valid_cards = TraitHelpers.deserialize_option_vector_u64(deserializer);
            deserializer.decrease_container_depth();
            return builder.build();
        }

        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null) return false;
            if (getClass() != obj.getClass()) return false;
            PlayRequestContext other = (PlayRequestContext) obj;
            if (!java.util.Objects.equals(this.player, other.player)) { return false; }
            if (!java.util.Objects.equals(this.hand, other.hand)) { return false; }
            if (!java.util.Objects.equals(this.trick, other.trick)) { return false; }
            if (!java.util.Objects.equals(this.valid_cards, other.valid_cards)) { return false; }
            return true;
        }

        public int hashCode() {
            int value = 7;
            value = 31 * value + (this.player != null ? this.player.hashCode() : 0);
            value = 31 * value + (this.hand != null ? this.hand.hashCode() : 0);
            value = 31 * value + (this.trick != null ? this.trick.hashCode() : 0);
            value = 31 * value + (this.valid_cards != null ? this.valid_cards.hashCode() : 0);
            return value;
        }

        public static final class Builder {
            public Player player;
            public java.util.List<Card> hand;
            public plump_message.Trick trick;
            public java.util.Optional<java.util.List<@com.novi.serde.Unsigned Long>> valid_cards;

            public PlayRequestContext build() {
                return new PlayRequestContext(
                    player,
                    hand,
                    trick,
                    valid_cards
                );
            }
        }
    }

    public static final class Trick extends Message {
        public final plump_message.Trick value;

        public Trick(plump_message.Trick value) {
            java.util.Objects.requireNonNull(value, "value must not be null");
            this.value = value;
        }

        public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
            serializer.increase_container_depth();
            serializer.serialize_variant_index(4);
            value.serialize(serializer);
            serializer.decrease_container_depth();
        }

        static Trick load(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
            deserializer.increase_container_depth();
            Builder builder = new Builder();
            builder.value = plump_message.Trick.deserialize(deserializer);
            deserializer.decrease_container_depth();
            return builder.build();
        }

        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null) return false;
            if (getClass() != obj.getClass()) return false;
            Trick other = (Trick) obj;
            if (!java.util.Objects.equals(this.value, other.value)) { return false; }
            return true;
        }

        public int hashCode() {
            int value = 7;
            value = 31 * value + (this.value != null ? this.value.hashCode() : 0);
            return value;
        }

        public static final class Builder {
            public plump_message.Trick value;

            public Trick build() {
                return new Trick(
                    value
                );
            }
        }
    }

    public static final class Scoreboard extends Message {
        public final java.util.Map<PlayerName, PublicState> state;

        public Scoreboard(java.util.Map<PlayerName, PublicState> state) {
            java.util.Objects.requireNonNull(state, "state must not be null");
            this.state = state;
        }

        public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
            serializer.increase_container_depth();
            serializer.serialize_variant_index(5);
            TraitHelpers.serialize_map_PlayerName_to_PublicState(state, serializer);
            serializer.decrease_container_depth();
        }

        static Scoreboard load(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
            deserializer.increase_container_depth();
            Builder builder = new Builder();
            builder.state = TraitHelpers.deserialize_map_PlayerName_to_PublicState(deserializer);
            deserializer.decrease_container_depth();
            return builder.build();
        }

        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null) return false;
            if (getClass() != obj.getClass()) return false;
            Scoreboard other = (Scoreboard) obj;
            if (!java.util.Objects.equals(this.state, other.state)) { return false; }
            return true;
        }

        public int hashCode() {
            int value = 7;
            value = 31 * value + (this.state != null ? this.state.hashCode() : 0);
            return value;
        }

        public static final class Builder {
            public java.util.Map<PlayerName, PublicState> state;

            public Scoreboard build() {
                return new Scoreboard(
                    state
                );
            }
        }
    }

    public static final class Winner extends Message {
        public final Player value;

        public Winner(Player value) {
            java.util.Objects.requireNonNull(value, "value must not be null");
            this.value = value;
        }

        public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
            serializer.increase_container_depth();
            serializer.serialize_variant_index(6);
            value.serialize(serializer);
            serializer.decrease_container_depth();
        }

        static Winner load(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
            deserializer.increase_container_depth();
            Builder builder = new Builder();
            builder.value = Player.deserialize(deserializer);
            deserializer.decrease_container_depth();
            return builder.build();
        }

        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null) return false;
            if (getClass() != obj.getClass()) return false;
            Winner other = (Winner) obj;
            if (!java.util.Objects.equals(this.value, other.value)) { return false; }
            return true;
        }

        public int hashCode() {
            int value = 7;
            value = 31 * value + (this.value != null ? this.value.hashCode() : 0);
            return value;
        }

        public static final class Builder {
            public Player value;

            public Winner build() {
                return new Winner(
                    value
                );
            }
        }
    }

    public static final class Winners extends Message {
        public final java.util.List<Player> players;
        public final java.util.List<@com.novi.serde.Unsigned Long> winner_indices;

        public Winners(java.util.List<Player> players, java.util.List<@com.novi.serde.Unsigned Long> winner_indices) {
            java.util.Objects.requireNonNull(players, "players must not be null");
            java.util.Objects.requireNonNull(winner_indices, "winner_indices must not be null");
            this.players = players;
            this.winner_indices = winner_indices;
        }

        public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
            serializer.increase_container_depth();
            serializer.serialize_variant_index(7);
            TraitHelpers.serialize_vector_Player(players, serializer);
            TraitHelpers.serialize_vector_u64(winner_indices, serializer);
            serializer.decrease_container_depth();
        }

        static Winners load(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
            deserializer.increase_container_depth();
            Builder builder = new Builder();
            builder.players = TraitHelpers.deserialize_vector_Player(deserializer);
            builder.winner_indices = TraitHelpers.deserialize_vector_u64(deserializer);
            deserializer.decrease_container_depth();
            return builder.build();
        }

        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null) return false;
            if (getClass() != obj.getClass()) return false;
            Winners other = (Winners) obj;
            if (!java.util.Objects.equals(this.players, other.players)) { return false; }
            if (!java.util.Objects.equals(this.winner_indices, other.winner_indices)) { return false; }
            return true;
        }

        public int hashCode() {
            int value = 7;
            value = 31 * value + (this.players != null ? this.players.hashCode() : 0);
            value = 31 * value + (this.winner_indices != null ? this.winner_indices.hashCode() : 0);
            return value;
        }

        public static final class Builder {
            public java.util.List<Player> players;
            public java.util.List<@com.novi.serde.Unsigned Long> winner_indices;

            public Winners build() {
                return new Winners(
                    players,
                    winner_indices
                );
            }
        }
    }

    public static final class RequestPlayerName extends Message {
        public RequestPlayerName() {
        }

        public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
            serializer.increase_container_depth();
            serializer.serialize_variant_index(8);
            serializer.decrease_container_depth();
        }

        static RequestPlayerName load(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
            deserializer.increase_container_depth();
            Builder builder = new Builder();
            deserializer.decrease_container_depth();
            return builder.build();
        }

        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null) return false;
            if (getClass() != obj.getClass()) return false;
            RequestPlayerName other = (RequestPlayerName) obj;
            return true;
        }

        public int hashCode() {
            int value = 7;
            return value;
        }

        public static final class Builder {
            public RequestPlayerName build() {
                return new RequestPlayerName(
                );
            }
        }
    }

    public static final class PlayRequest extends Message {
        public final Player value;

        public PlayRequest(Player value) {
            java.util.Objects.requireNonNull(value, "value must not be null");
            this.value = value;
        }

        public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
            serializer.increase_container_depth();
            serializer.serialize_variant_index(9);
            value.serialize(serializer);
            serializer.decrease_container_depth();
        }

        static PlayRequest load(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
            deserializer.increase_container_depth();
            Builder builder = new Builder();
            builder.value = Player.deserialize(deserializer);
            deserializer.decrease_container_depth();
            return builder.build();
        }

        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null) return false;
            if (getClass() != obj.getClass()) return false;
            PlayRequest other = (PlayRequest) obj;
            if (!java.util.Objects.equals(this.value, other.value)) { return false; }
            return true;
        }

        public int hashCode() {
            int value = 7;
            value = 31 * value + (this.value != null ? this.value.hashCode() : 0);
            return value;
        }

        public static final class Builder {
            public Player value;

            public PlayRequest build() {
                return new PlayRequest(
                    value
                );
            }
        }
    }

    public static final class RequestGuess extends Message {
        public RequestGuess() {
        }

        public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
            serializer.increase_container_depth();
            serializer.serialize_variant_index(10);
            serializer.decrease_container_depth();
        }

        static RequestGuess load(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
            deserializer.increase_container_depth();
            Builder builder = new Builder();
            deserializer.decrease_container_depth();
            return builder.build();
        }

        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null) return false;
            if (getClass() != obj.getClass()) return false;
            RequestGuess other = (RequestGuess) obj;
            return true;
        }

        public int hashCode() {
            int value = 7;
            return value;
        }

        public static final class Builder {
            public RequestGuess build() {
                return new RequestGuess(
                );
            }
        }
    }

    public static final class GameOver extends Message {
        public GameOver() {
        }

        public void serialize(com.novi.serde.Serializer serializer) throws com.novi.serde.SerializationError {
            serializer.increase_container_depth();
            serializer.serialize_variant_index(11);
            serializer.decrease_container_depth();
        }

        static GameOver load(com.novi.serde.Deserializer deserializer) throws com.novi.serde.DeserializationError {
            deserializer.increase_container_depth();
            Builder builder = new Builder();
            deserializer.decrease_container_depth();
            return builder.build();
        }

        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null) return false;
            if (getClass() != obj.getClass()) return false;
            GameOver other = (GameOver) obj;
            return true;
        }

        public int hashCode() {
            int value = 7;
            return value;
        }

        public static final class Builder {
            public GameOver build() {
                return new GameOver(
                );
            }
        }
    }
}

