
// Cpp Includes
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

std::vector<int> getInputs(std::string_view inputFile) {
  std::ifstream input(inputFile.data());
  std::vector<int> content;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string value; std::getline(input, value);) {
    content.push_back(std::stoi(value));
  }

  return content;
}

int problem1(const std::vector<int> &values) {

  std::unordered_map<int, bool> registry;
  const int target = 2020;

  for (auto value : values) {

    auto missing_pair = target - value;

    if (registry.contains(missing_pair)) return missing_pair * value;

    registry.insert({value, true});
  }
  return -1;
}

int problem2(const std::vector<int> &values) {

  for (auto i = 0u; i < values.size() - 2; i++) {
    for (auto j = 1u; j < values.size() - 1; j++) {
      for (auto k = 2u; k < values.size(); k++) {
        if (values[i] + values[j] + values[k] == 2020) return values[i] * values[j] * values[k];
      }
    }
  }
  return -1;
}

int main() {
  auto input = getInputs("2020/01/inputs/input.txt");
  std::cout << "Solution #1: " << std::to_string(problem1(input)) << '\n';
  std::cout << "Solution #2: " << std::to_string(problem2(input)) << '\n';
  return 0;
}
