
// Cpp Includes
#include <fstream>
#include <functional>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

std::vector<int> getInput(std::string_view filepath) {

  std::ifstream file(filepath.data());
  std::vector<int> output;

  if (!file.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string line; std::getline(file, line);) {
    output.push_back(std::stoi(line));
  }

  std::sort(output.begin(), output.end());
  return output;
}

int problem1(const std::vector<int> &input) {

  auto sortedInput = input;

  std::unordered_map<int, int> differences = {{1, 0}, {2, 0}, {3, 1}};

  for (auto i = 1u; i < sortedInput.size(); i++) {
    auto deltaV = sortedInput.at(i) - sortedInput.at(i - 1);
    differences[deltaV]++;
  }

  return differences[1] * differences[3];
}

uint64_t problem2(const std::vector<int> &input) {

  std::unordered_map<int, uint64_t> dp;

  std::function<uint64_t(unsigned int)> recursiveCall = [&](unsigned int index) {
    if (dp.contains(index)) return dp[index];
    if (index == input.size() - 1) return static_cast<uint64_t>(1);

    uint64_t total = 0;

    for (auto next = index + 1; next < input.size(); next++)
      if (input[next] - input[index] <= 3) total += recursiveCall(next);

    dp[index] = total;
    return total;
  };

  return recursiveCall(0);
}

int main() {
  auto input = getInput("2020/10/inputs/input.txt");
  input.insert(input.begin(), 0);

  std::cout << "Problem #1: : " << std::to_string(problem1(input)) << '\n';
  std::cout << "Problem #2: : " << std::to_string(problem2(input)) << '\n';
}
