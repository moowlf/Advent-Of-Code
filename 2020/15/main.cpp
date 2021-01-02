
// CPP Includes
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

std::vector<uint64_t> getInput(std::string_view file) {
  std::ifstream input(file.data());
  std::vector<uint64_t> content;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string value; std::getline(input, value);) {
    content.push_back(std::stoi(value));
  }

  return content;
}

uint64_t solver(const std::vector<uint64_t> &input, uint64_t nthNumber) {

  uint64_t untilNum = nthNumber - 1;

  // From the input
  std::unordered_map<int, int> spokenNumbers;
  for (auto i = 0u; i < input.size() - 1; i++) spokenNumbers[input[i]] = i + 1;

  // Continue now
  uint64_t turn                = input.size();
  uint64_t currentSpokenNumber = input.back();

  do {

    if (!spokenNumbers.contains(currentSpokenNumber)) {
      spokenNumbers[currentSpokenNumber] = turn;
      currentSpokenNumber                = 0;
      continue;
    }

    auto nextSpokenNumber              = turn - spokenNumbers[currentSpokenNumber];
    spokenNumbers[currentSpokenNumber] = turn;
    currentSpokenNumber                = nextSpokenNumber;

  } while (turn++ < untilNum);

  return currentSpokenNumber;
}

uint64_t problem1(const std::vector<uint64_t> &input) { return solver(input, 2020); }

uint64_t problem2(const std::vector<uint64_t> &input) { return solver(input, 30000000); }

int main() {

  auto input = getInput("2020/15/inputs/input.txt");

  std::cout << "Problem #1: : " << std::to_string(problem1(input)) << '\n';
  std::cout << "Problem #2: : " << std::to_string(problem2(input)) << '\n';
}