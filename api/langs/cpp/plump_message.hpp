#pragma once

#include "serde.hpp"
#include "bincode.hpp"

namespace plump_message {

    struct Card {
        uint64_t suit;
        uint64_t value;

        friend bool operator==(const Card&, const Card&);
        std::vector<uint8_t> bincodeSerialize() const;
        static Card bincodeDeserialize(std::vector<uint8_t>);
    };

    struct PlayerName {
        std::string value;

        friend bool operator==(const PlayerName&, const PlayerName&);
        std::vector<uint8_t> bincodeSerialize() const;
        static PlayerName bincodeDeserialize(std::vector<uint8_t>);
    };

    struct Player {
        plump_message::PlayerName name;
        bool human;
        std::vector<plump_message::Card> hand;

        friend bool operator==(const Player&, const Player&);
        std::vector<uint8_t> bincodeSerialize() const;
        static Player bincodeDeserialize(std::vector<uint8_t>);
    };

    struct PublicState {
        std::optional<uint64_t> guess;
        uint64_t wins;
        uint64_t score;

        friend bool operator==(const PublicState&, const PublicState&);
        std::vector<uint8_t> bincodeSerialize() const;
        static PublicState bincodeDeserialize(std::vector<uint8_t>);
    };

    struct Trick {
        std::vector<plump_message::Card> value;

        friend bool operator==(const Trick&, const Trick&);
        std::vector<uint8_t> bincodeSerialize() const;
        static Trick bincodeDeserialize(std::vector<uint8_t>);
    };

    struct Message {

        struct RequestGuessContext {
            plump_message::Player player;
            std::vector<plump_message::Card> hand;
            std::vector<uint64_t> guesses;
            uint64_t players;

            friend bool operator==(const RequestGuessContext&, const RequestGuessContext&);
            std::vector<uint8_t> bincodeSerialize() const;
            static RequestGuessContext bincodeDeserialize(std::vector<uint8_t>);
        };

        struct Guesses {
            std::map<plump_message::PlayerName, plump_message::PublicState> state;

            friend bool operator==(const Guesses&, const Guesses&);
            std::vector<uint8_t> bincodeSerialize() const;
            static Guesses bincodeDeserialize(std::vector<uint8_t>);
        };

        struct Turn {
            plump_message::Player whose;

            friend bool operator==(const Turn&, const Turn&);
            std::vector<uint8_t> bincodeSerialize() const;
            static Turn bincodeDeserialize(std::vector<uint8_t>);
        };

        struct PlayRequestContext {
            plump_message::Player player;
            std::vector<plump_message::Card> hand;
            plump_message::Trick trick;
            std::optional<std::vector<uint64_t>> valid_cards;

            friend bool operator==(const PlayRequestContext&, const PlayRequestContext&);
            std::vector<uint8_t> bincodeSerialize() const;
            static PlayRequestContext bincodeDeserialize(std::vector<uint8_t>);
        };

        struct Trick {
            plump_message::Trick value;

            friend bool operator==(const Trick&, const Trick&);
            std::vector<uint8_t> bincodeSerialize() const;
            static Trick bincodeDeserialize(std::vector<uint8_t>);
        };

        struct Scoreboard {
            std::map<plump_message::PlayerName, plump_message::PublicState> state;

            friend bool operator==(const Scoreboard&, const Scoreboard&);
            std::vector<uint8_t> bincodeSerialize() const;
            static Scoreboard bincodeDeserialize(std::vector<uint8_t>);
        };

        struct Winner {
            plump_message::Player value;

            friend bool operator==(const Winner&, const Winner&);
            std::vector<uint8_t> bincodeSerialize() const;
            static Winner bincodeDeserialize(std::vector<uint8_t>);
        };

        struct Winners {
            std::vector<plump_message::Player> players;
            std::vector<uint64_t> winner_indices;

            friend bool operator==(const Winners&, const Winners&);
            std::vector<uint8_t> bincodeSerialize() const;
            static Winners bincodeDeserialize(std::vector<uint8_t>);
        };

        struct RequestPlayerName {
            friend bool operator==(const RequestPlayerName&, const RequestPlayerName&);
            std::vector<uint8_t> bincodeSerialize() const;
            static RequestPlayerName bincodeDeserialize(std::vector<uint8_t>);
        };

        struct PlayRequest {
            plump_message::Player value;

            friend bool operator==(const PlayRequest&, const PlayRequest&);
            std::vector<uint8_t> bincodeSerialize() const;
            static PlayRequest bincodeDeserialize(std::vector<uint8_t>);
        };

        struct RequestGuess {
            friend bool operator==(const RequestGuess&, const RequestGuess&);
            std::vector<uint8_t> bincodeSerialize() const;
            static RequestGuess bincodeDeserialize(std::vector<uint8_t>);
        };

        struct GameOver {
            friend bool operator==(const GameOver&, const GameOver&);
            std::vector<uint8_t> bincodeSerialize() const;
            static GameOver bincodeDeserialize(std::vector<uint8_t>);
        };

        std::variant<RequestGuessContext, Guesses, Turn, PlayRequestContext, Trick, Scoreboard, Winner, Winners, RequestPlayerName, PlayRequest, RequestGuess, GameOver> value;

        friend bool operator==(const Message&, const Message&);
        std::vector<uint8_t> bincodeSerialize() const;
        static Message bincodeDeserialize(std::vector<uint8_t>);
    };

} // end of namespace plump_message


namespace plump_message {

    inline bool operator==(const Card &lhs, const Card &rhs) {
        if (!(lhs.suit == rhs.suit)) { return false; }
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Card::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Card>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Card Card::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Card>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Card>::serialize(const plump_message::Card &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.suit)>::serialize(obj.suit, serializer);
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
plump_message::Card serde::Deserializable<plump_message::Card>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    plump_message::Card obj;
    obj.suit = serde::Deserializable<decltype(obj.suit)>::deserialize(deserializer);
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}

namespace plump_message {

    inline bool operator==(const Message &lhs, const Message &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Message::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Message>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Message Message::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Message>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Message>::serialize(const plump_message::Message &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
plump_message::Message serde::Deserializable<plump_message::Message>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    plump_message::Message obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}

namespace plump_message {

    inline bool operator==(const Message::RequestGuessContext &lhs, const Message::RequestGuessContext &rhs) {
        if (!(lhs.player == rhs.player)) { return false; }
        if (!(lhs.hand == rhs.hand)) { return false; }
        if (!(lhs.guesses == rhs.guesses)) { return false; }
        if (!(lhs.players == rhs.players)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Message::RequestGuessContext::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Message::RequestGuessContext>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Message::RequestGuessContext Message::RequestGuessContext::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Message::RequestGuessContext>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Message::RequestGuessContext>::serialize(const plump_message::Message::RequestGuessContext &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.player)>::serialize(obj.player, serializer);
    serde::Serializable<decltype(obj.hand)>::serialize(obj.hand, serializer);
    serde::Serializable<decltype(obj.guesses)>::serialize(obj.guesses, serializer);
    serde::Serializable<decltype(obj.players)>::serialize(obj.players, serializer);
}

template <>
template <typename Deserializer>
plump_message::Message::RequestGuessContext serde::Deserializable<plump_message::Message::RequestGuessContext>::deserialize(Deserializer &deserializer) {
    plump_message::Message::RequestGuessContext obj;
    obj.player = serde::Deserializable<decltype(obj.player)>::deserialize(deserializer);
    obj.hand = serde::Deserializable<decltype(obj.hand)>::deserialize(deserializer);
    obj.guesses = serde::Deserializable<decltype(obj.guesses)>::deserialize(deserializer);
    obj.players = serde::Deserializable<decltype(obj.players)>::deserialize(deserializer);
    return obj;
}

namespace plump_message {

    inline bool operator==(const Message::Guesses &lhs, const Message::Guesses &rhs) {
        if (!(lhs.state == rhs.state)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Message::Guesses::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Message::Guesses>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Message::Guesses Message::Guesses::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Message::Guesses>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Message::Guesses>::serialize(const plump_message::Message::Guesses &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.state)>::serialize(obj.state, serializer);
}

template <>
template <typename Deserializer>
plump_message::Message::Guesses serde::Deserializable<plump_message::Message::Guesses>::deserialize(Deserializer &deserializer) {
    plump_message::Message::Guesses obj;
    obj.state = serde::Deserializable<decltype(obj.state)>::deserialize(deserializer);
    return obj;
}

namespace plump_message {

    inline bool operator==(const Message::Turn &lhs, const Message::Turn &rhs) {
        if (!(lhs.whose == rhs.whose)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Message::Turn::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Message::Turn>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Message::Turn Message::Turn::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Message::Turn>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Message::Turn>::serialize(const plump_message::Message::Turn &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.whose)>::serialize(obj.whose, serializer);
}

template <>
template <typename Deserializer>
plump_message::Message::Turn serde::Deserializable<plump_message::Message::Turn>::deserialize(Deserializer &deserializer) {
    plump_message::Message::Turn obj;
    obj.whose = serde::Deserializable<decltype(obj.whose)>::deserialize(deserializer);
    return obj;
}

namespace plump_message {

    inline bool operator==(const Message::PlayRequestContext &lhs, const Message::PlayRequestContext &rhs) {
        if (!(lhs.player == rhs.player)) { return false; }
        if (!(lhs.hand == rhs.hand)) { return false; }
        if (!(lhs.trick == rhs.trick)) { return false; }
        if (!(lhs.valid_cards == rhs.valid_cards)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Message::PlayRequestContext::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Message::PlayRequestContext>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Message::PlayRequestContext Message::PlayRequestContext::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Message::PlayRequestContext>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Message::PlayRequestContext>::serialize(const plump_message::Message::PlayRequestContext &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.player)>::serialize(obj.player, serializer);
    serde::Serializable<decltype(obj.hand)>::serialize(obj.hand, serializer);
    serde::Serializable<decltype(obj.trick)>::serialize(obj.trick, serializer);
    serde::Serializable<decltype(obj.valid_cards)>::serialize(obj.valid_cards, serializer);
}

template <>
template <typename Deserializer>
plump_message::Message::PlayRequestContext serde::Deserializable<plump_message::Message::PlayRequestContext>::deserialize(Deserializer &deserializer) {
    plump_message::Message::PlayRequestContext obj;
    obj.player = serde::Deserializable<decltype(obj.player)>::deserialize(deserializer);
    obj.hand = serde::Deserializable<decltype(obj.hand)>::deserialize(deserializer);
    obj.trick = serde::Deserializable<decltype(obj.trick)>::deserialize(deserializer);
    obj.valid_cards = serde::Deserializable<decltype(obj.valid_cards)>::deserialize(deserializer);
    return obj;
}

namespace plump_message {

    inline bool operator==(const Message::Trick &lhs, const Message::Trick &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Message::Trick::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Message::Trick>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Message::Trick Message::Trick::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Message::Trick>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Message::Trick>::serialize(const plump_message::Message::Trick &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
}

template <>
template <typename Deserializer>
plump_message::Message::Trick serde::Deserializable<plump_message::Message::Trick>::deserialize(Deserializer &deserializer) {
    plump_message::Message::Trick obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    return obj;
}

namespace plump_message {

    inline bool operator==(const Message::Scoreboard &lhs, const Message::Scoreboard &rhs) {
        if (!(lhs.state == rhs.state)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Message::Scoreboard::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Message::Scoreboard>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Message::Scoreboard Message::Scoreboard::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Message::Scoreboard>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Message::Scoreboard>::serialize(const plump_message::Message::Scoreboard &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.state)>::serialize(obj.state, serializer);
}

template <>
template <typename Deserializer>
plump_message::Message::Scoreboard serde::Deserializable<plump_message::Message::Scoreboard>::deserialize(Deserializer &deserializer) {
    plump_message::Message::Scoreboard obj;
    obj.state = serde::Deserializable<decltype(obj.state)>::deserialize(deserializer);
    return obj;
}

namespace plump_message {

    inline bool operator==(const Message::Winner &lhs, const Message::Winner &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Message::Winner::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Message::Winner>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Message::Winner Message::Winner::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Message::Winner>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Message::Winner>::serialize(const plump_message::Message::Winner &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
}

template <>
template <typename Deserializer>
plump_message::Message::Winner serde::Deserializable<plump_message::Message::Winner>::deserialize(Deserializer &deserializer) {
    plump_message::Message::Winner obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    return obj;
}

namespace plump_message {

    inline bool operator==(const Message::Winners &lhs, const Message::Winners &rhs) {
        if (!(lhs.players == rhs.players)) { return false; }
        if (!(lhs.winner_indices == rhs.winner_indices)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Message::Winners::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Message::Winners>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Message::Winners Message::Winners::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Message::Winners>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Message::Winners>::serialize(const plump_message::Message::Winners &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.players)>::serialize(obj.players, serializer);
    serde::Serializable<decltype(obj.winner_indices)>::serialize(obj.winner_indices, serializer);
}

template <>
template <typename Deserializer>
plump_message::Message::Winners serde::Deserializable<plump_message::Message::Winners>::deserialize(Deserializer &deserializer) {
    plump_message::Message::Winners obj;
    obj.players = serde::Deserializable<decltype(obj.players)>::deserialize(deserializer);
    obj.winner_indices = serde::Deserializable<decltype(obj.winner_indices)>::deserialize(deserializer);
    return obj;
}

namespace plump_message {

    inline bool operator==(const Message::RequestPlayerName &lhs, const Message::RequestPlayerName &rhs) {
        return true;
    }

    inline std::vector<uint8_t> Message::RequestPlayerName::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Message::RequestPlayerName>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Message::RequestPlayerName Message::RequestPlayerName::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Message::RequestPlayerName>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Message::RequestPlayerName>::serialize(const plump_message::Message::RequestPlayerName &obj, Serializer &serializer) {
}

template <>
template <typename Deserializer>
plump_message::Message::RequestPlayerName serde::Deserializable<plump_message::Message::RequestPlayerName>::deserialize(Deserializer &deserializer) {
    plump_message::Message::RequestPlayerName obj;
    return obj;
}

namespace plump_message {

    inline bool operator==(const Message::PlayRequest &lhs, const Message::PlayRequest &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Message::PlayRequest::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Message::PlayRequest>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Message::PlayRequest Message::PlayRequest::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Message::PlayRequest>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Message::PlayRequest>::serialize(const plump_message::Message::PlayRequest &obj, Serializer &serializer) {
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
}

template <>
template <typename Deserializer>
plump_message::Message::PlayRequest serde::Deserializable<plump_message::Message::PlayRequest>::deserialize(Deserializer &deserializer) {
    plump_message::Message::PlayRequest obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    return obj;
}

namespace plump_message {

    inline bool operator==(const Message::RequestGuess &lhs, const Message::RequestGuess &rhs) {
        return true;
    }

    inline std::vector<uint8_t> Message::RequestGuess::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Message::RequestGuess>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Message::RequestGuess Message::RequestGuess::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Message::RequestGuess>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Message::RequestGuess>::serialize(const plump_message::Message::RequestGuess &obj, Serializer &serializer) {
}

template <>
template <typename Deserializer>
plump_message::Message::RequestGuess serde::Deserializable<plump_message::Message::RequestGuess>::deserialize(Deserializer &deserializer) {
    plump_message::Message::RequestGuess obj;
    return obj;
}

namespace plump_message {

    inline bool operator==(const Message::GameOver &lhs, const Message::GameOver &rhs) {
        return true;
    }

    inline std::vector<uint8_t> Message::GameOver::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Message::GameOver>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Message::GameOver Message::GameOver::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Message::GameOver>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Message::GameOver>::serialize(const plump_message::Message::GameOver &obj, Serializer &serializer) {
}

template <>
template <typename Deserializer>
plump_message::Message::GameOver serde::Deserializable<plump_message::Message::GameOver>::deserialize(Deserializer &deserializer) {
    plump_message::Message::GameOver obj;
    return obj;
}

namespace plump_message {

    inline bool operator==(const Player &lhs, const Player &rhs) {
        if (!(lhs.name == rhs.name)) { return false; }
        if (!(lhs.human == rhs.human)) { return false; }
        if (!(lhs.hand == rhs.hand)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Player::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Player>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Player Player::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Player>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Player>::serialize(const plump_message::Player &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.name)>::serialize(obj.name, serializer);
    serde::Serializable<decltype(obj.human)>::serialize(obj.human, serializer);
    serde::Serializable<decltype(obj.hand)>::serialize(obj.hand, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
plump_message::Player serde::Deserializable<plump_message::Player>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    plump_message::Player obj;
    obj.name = serde::Deserializable<decltype(obj.name)>::deserialize(deserializer);
    obj.human = serde::Deserializable<decltype(obj.human)>::deserialize(deserializer);
    obj.hand = serde::Deserializable<decltype(obj.hand)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}

namespace plump_message {

    inline bool operator==(const PlayerName &lhs, const PlayerName &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> PlayerName::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<PlayerName>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline PlayerName PlayerName::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<PlayerName>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::PlayerName>::serialize(const plump_message::PlayerName &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
plump_message::PlayerName serde::Deserializable<plump_message::PlayerName>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    plump_message::PlayerName obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}

namespace plump_message {

    inline bool operator==(const PublicState &lhs, const PublicState &rhs) {
        if (!(lhs.guess == rhs.guess)) { return false; }
        if (!(lhs.wins == rhs.wins)) { return false; }
        if (!(lhs.score == rhs.score)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> PublicState::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<PublicState>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline PublicState PublicState::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<PublicState>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::PublicState>::serialize(const plump_message::PublicState &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.guess)>::serialize(obj.guess, serializer);
    serde::Serializable<decltype(obj.wins)>::serialize(obj.wins, serializer);
    serde::Serializable<decltype(obj.score)>::serialize(obj.score, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
plump_message::PublicState serde::Deserializable<plump_message::PublicState>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    plump_message::PublicState obj;
    obj.guess = serde::Deserializable<decltype(obj.guess)>::deserialize(deserializer);
    obj.wins = serde::Deserializable<decltype(obj.wins)>::deserialize(deserializer);
    obj.score = serde::Deserializable<decltype(obj.score)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}

namespace plump_message {

    inline bool operator==(const Trick &lhs, const Trick &rhs) {
        if (!(lhs.value == rhs.value)) { return false; }
        return true;
    }

    inline std::vector<uint8_t> Trick::bincodeSerialize() const {
        auto serializer = serde::BincodeSerializer();
        serde::Serializable<Trick>::serialize(*this, serializer);
        return std::move(serializer).bytes();
    }

    inline Trick Trick::bincodeDeserialize(std::vector<uint8_t> input) {
        auto deserializer = serde::BincodeDeserializer(input);
        auto value = serde::Deserializable<Trick>::deserialize(deserializer);
        if (deserializer.get_buffer_offset() < input.size()) {
            throw serde::deserialization_error("Some input bytes were not read");
        }
        return value;
    }

} // end of namespace plump_message

template <>
template <typename Serializer>
void serde::Serializable<plump_message::Trick>::serialize(const plump_message::Trick &obj, Serializer &serializer) {
    serializer.increase_container_depth();
    serde::Serializable<decltype(obj.value)>::serialize(obj.value, serializer);
    serializer.decrease_container_depth();
}

template <>
template <typename Deserializer>
plump_message::Trick serde::Deserializable<plump_message::Trick>::deserialize(Deserializer &deserializer) {
    deserializer.increase_container_depth();
    plump_message::Trick obj;
    obj.value = serde::Deserializable<decltype(obj.value)>::deserialize(deserializer);
    deserializer.decrease_container_depth();
    return obj;
}
