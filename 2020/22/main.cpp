
// CPP Includes
#include <deque>
#include <fstream>
#include <functional>
#include <iostream>
#include <unordered_map>
#include <vector>

using Hand  = std::deque<uint32_t>;
using Hands = std::vector<Hand>;

template <>
struct std::hash<Hand> {

  std::size_t operator()(Hand const &hand) const noexcept {

    // ! It's only valid for this problem since cards do not repeat in a hand
    uint64_t hash;

    for (auto value : hand) {
      hash |= (1 << value);
    }

    return hash;
  }
};

Hands getInput(std::string_view file) {
  std::ifstream input(file.data());

  Hands hands;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string value; std::getline(input, value);) {

    if (value.starts_with("Player")) {
      hands.emplace_back();
      continue;
    }

    if (value.empty()) continue;

    hands.back().push_back(std::stoul(value));
  }

  return hands;
}

uint64_t problem1(const Hands &_) {

  Hands hands{_};
  while (true) {

    if (hands.front().size() == 0 || hands.back().size() == 0) break;

    auto player_1_card = hands.front().front();
    auto player_2_card = hands.back().front();

    if (player_1_card > player_2_card) {
      hands.front().push_back(player_1_card);
      hands.front().push_back(player_2_card);

    } else {
      hands.back().push_back(player_2_card);
      hands.back().push_back(player_1_card);
    }

    hands.front().pop_front();
    hands.back().pop_front();
  }

  auto avaliate_hand = [](const std::deque<uint32_t> &hand) {
    uint64_t value{0};
    size_t hand_size{hand.size()};
    for (auto index = 0u; index < hand.size(); index++) {
      value += (hand.at(index) * (hand_size - index));
    }

    return value;
  };

  if (hands.front().size() != 0) {
    return avaliate_hand(hands.front());
  } else {
    return avaliate_hand(hands.back());
  }
}

using PartialHand = std::deque<uint32_t>;
struct RecursivePlay {
  PartialHand player1;
  PartialHand player2;
  bool forcedWin{false};
};

RecursivePlay play(RecursivePlay rec) {

  std::unordered_map<PartialHand, bool> player1_history;
  std::unordered_map<PartialHand, bool> player2_history;

  while (rec.player1.size() != 0 && rec.player2.size() != 0) {

    // ! If any of this hands were already seen, player 1 wins
    if (player1_history.contains(rec.player1) || player2_history.contains(rec.player2)) {
      rec.forcedWin = true;
      return rec;
    }

    player1_history.try_emplace(rec.player1, true);
    player2_history.try_emplace(rec.player2, true);

    auto player_1_top_card = rec.player1.front();
    auto player_2_top_card = rec.player2.front();
    bool player_1_has_won  = false;
    bool went_recursive    = false;

    // ! Recursive Combat
    auto player_1_left_cards = rec.player1.size() - 1;
    auto player_2_left_cards = rec.player2.size() - 1;

    if (player_1_top_card <= player_1_left_cards && player_2_top_card <= player_2_left_cards) {
      // * Indeed recursive combat

      auto new_round = play(
          {PartialHand(rec.player1.begin() + 1, rec.player1.begin() + 1 + player_1_top_card),
           PartialHand(rec.player2.begin() + 1, rec.player2.begin() + 1 + player_2_top_card)});

      player_1_has_won = !new_round.player1.empty();
      went_recursive   = true;
    }

    // ! Regular round

    if (player_1_has_won || (!went_recursive && player_1_top_card > player_2_top_card)) {
      rec.player1.push_back(player_1_top_card);
      rec.player1.push_back(player_2_top_card);
    } else {
      rec.player2.push_back(player_2_top_card);
      rec.player2.push_back(player_1_top_card);
    }

    // ! Remove from hand
    rec.player1.pop_front();
    rec.player2.pop_front();
  }

  return rec;
}

uint64_t problem2(const Hands &_) {
  auto endState = play({_.front(), _.back()});

  auto avaliate_hand = [](const std::deque<uint32_t> &hand) {
    uint64_t value{0};
    size_t hand_size{hand.size()};
    for (auto index = 0u; index < hand.size(); index++) {
      value += (hand.at(index) * (hand_size - index));
    }

    return value;
  };

  if (endState.player1.size() != 0) return avaliate_hand(endState.player1);
  return avaliate_hand(endState.player2);
}

int main() {

  auto input = getInput("2020/22/inputs/input.txt");

  std::cout << "Problem #1: : " << std::to_string(problem1(input)) << '\n';
  std::cout << "Problem #2: : " << std::to_string(problem2(input)) << '\n';
}
