
// CPP Includes
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

uint64_t problem1(const std::vector<uint64_t> &input) {

  auto &card_pubkey = input.front();
  auto &door_pubkey = input.back();

  auto calculate_loop_size = [](uint64_t value) {
    uint64_t current_value = 1u;
    uint64_t loop_size     = 0u;

    while (current_value != value) {

      current_value *= 7u;
      current_value %= 20201227;

      loop_size++;
    }

    return loop_size;
  };

  std::cout << calculate_loop_size(card_pubkey) << '\n';
  std::cout << calculate_loop_size(door_pubkey) << '\n';

  uint64_t value = 1;
  uint64_t c     = calculate_loop_size(door_pubkey);
  for (auto loopID = 0u; loopID < c; loopID++) {
    value *= card_pubkey;
    value %= 20201227;
  }

  std::cout << value << '\n';

  return 0;
}

int main() {

  std::vector<uint64_t> input = {3418282, 8719412};
  std::cout << "Problem #1: : " << std::to_string(problem1(input)) << '\n';
}
