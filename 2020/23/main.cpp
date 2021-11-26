
// CPP Includes
#include <algorithm>
#include <cstdint>
#include <fstream>
#include <functional>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

struct Node {
  uint64_t value;
  Node *next;
};

using Searcher    = std::function<bool(std::vector<uint64_t> const &, uint64_t)>;
using InitialCups = std::vector<uint64_t> const &;

void play(Node *initial_cup, InitialCups input, std::unordered_map<int, Node> &nodes, Searcher s, uint64_t moves, uint64_t max) {
  uint64_t moveID   = 0;
  Node *current_cup = initial_cup;

  while (moveID != moves) {

    // ! Current cup
    uint32_t current_cup_value = current_cup->value;

    uint32_t next_f_cup_value = current_cup->next->value;
    uint32_t next_s_cup_value = current_cup->next->next->value;
    uint32_t next_t_cup_value = current_cup->next->next->next->value;

    // ! Update linked list to remove the first three
    current_cup->next = current_cup->next->next->next->next;

    // ! Select Destination cup
    uint32_t desired_destination_cup = current_cup_value - 1;

    while (true) {

      // * If it's impossible to find a lower one, pick the hightest on
      if (desired_destination_cup < input.front()) {
        desired_destination_cup = max;
      }

      // * Case in which the desired cup was just extracted
      if (desired_destination_cup == next_f_cup_value || desired_destination_cup == next_s_cup_value ||
          desired_destination_cup == next_t_cup_value) {
        desired_destination_cup--;
        continue;
      }

      // * Try to find the desired cup
      auto cup_found = s(input, desired_destination_cup);

      if (!cup_found) {
        desired_destination_cup--;
        continue;
      }

      nodes.at(next_t_cup_value).next        = nodes.at(desired_destination_cup).next;
      nodes.at(desired_destination_cup).next = &(nodes.at(next_f_cup_value));

      moveID++;
      break;
    }

    current_cup = current_cup->next;
  }
}

std::string problem1(const std::vector<uint64_t> &_) {

  auto input = _;

  std::unordered_map<int, Node> nodes;
  nodes.try_emplace(input.at(0), input.at(0), nullptr);

  for (auto i = 1u; i < input.size(); i++) {
    auto it = nodes.try_emplace(input.at(i), input.at(i), nullptr);

    if (it.second) nodes.at(input.at(i - 1u)).next = &(*it.first).second;
  }

  nodes.at(input.back()).next = &nodes.at(input.front());

  std::ranges::sort(input);

  // ---
  auto searcher = [&nodes](std::vector<uint64_t> const &h, uint64_t n) { return nodes.contains(n); };

  play(&nodes.at(_.front()), input, nodes, searcher, 100, input.back());

  Node *current_cup = nodes.at(1).next;

  std::string result(input.size() - 1, ' ');
  auto current_index = 0u;
  do {
    result.at(current_index++) = static_cast<char>(current_cup->value) + 48;
    current_cup                = current_cup->next;
  } while (current_cup->value != 1);

  return result;
}

uint64_t problem2(const std::vector<uint64_t> &_) {

  auto input = _;

  std::unordered_map<int, Node> nodes;
  nodes.reserve(1'000'000);
  nodes.try_emplace(input.at(0), input.at(0), nullptr);

  for (auto i = 1u; i < input.size(); i++) {
    auto it = nodes.try_emplace(input.at(i), input.at(i), nullptr);

    if (it.second) nodes.at(input.at(i - 1u)).next = &(*it.first).second;
  }

  std::ranges::sort(input);

  for (auto i = input.back() + 1; i <= 1'000'000; i++) {
    auto it = nodes.try_emplace(i, i, nullptr);
    if (it.second && i != input.back() + 1) nodes.at(i - 1u).next = &(*it.first).second;
  }

  nodes.at(_.back()).next = &(nodes.at(input.back() + 1));
  nodes.at(1'000'000).next    = &nodes.at(_.front());

  auto searcher = [&nodes](std::vector<uint64_t> const &h, uint64_t n) {
    return nodes.contains(n);
  };

  play(&nodes.at(_.front()), input, nodes, searcher, 10'000'000, 1'000'000);

  return nodes.at(1).next->value * nodes.at(1).next->next->value;
}

int main() {

  std::cout << "Problem #1: : " << problem1({6, 8, 5, 9, 7, 4, 2, 1, 3}) << '\n';
  std::cout << "Problem #2: : " << problem2({6, 8, 5, 9, 7, 4, 2, 1, 3}) << '\n';
}
