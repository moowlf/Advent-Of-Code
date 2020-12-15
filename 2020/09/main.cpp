
// Cpp Includes
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

std::vector<uint64_t> getInput(std::string_view file) {

  std::ifstream input(file.data());
  std::vector<uint64_t> output;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string line; std::getline(input, line);) {
    output.emplace_back(std::stoull(line));
  }
  return output;
}

bool isValid(
    const std::vector<uint64_t> &input, std::vector<uint64_t>::const_iterator curr, uint64_t target, int window) {

  std::unordered_map<int64_t, bool> seen;
  for (auto it = curr - window + 1; it != curr + 1; ++it) {

    int64_t obj = target - *it;

    if (seen.contains(obj)) return true;

    seen[*it] = true;
  }

  return false;
}

uint64_t problem1(const std::vector<uint64_t> &input, int window) {

  auto currentIt = input.begin();

  for (auto it = currentIt + window; it != input.end(); ++it) {
    if (!isValid(input, it - 1, *it, window)) return *it;
  }

  throw std::runtime_error("Impossible state!");
}

struct data_t {
  bool isValid{false};
  uint64_t min, max;
};

data_t sumsTo(
    const std::vector<uint64_t>::const_iterator &it, const std::vector<uint64_t>::const_iterator &end,
    uint64_t target) {

  data_t result;

  result.min = *it;
  result.max = *it;

  auto soFar   = 0u;
  auto slideIt = it;
  while (slideIt != end) {

    if (*slideIt < result.min)
      result.min = *slideIt;
    else if (result.max < *slideIt)
      result.max = *slideIt;

    soFar += *slideIt;
    if (soFar == target) {
      result.isValid = true;
      break;
    }

    if (soFar > target) break;

    ++slideIt;
  }

  return result;
}

uint64_t problem2(const std::vector<uint64_t> &input, int window) {
  uint64_t target = problem1(input, window);

  for (auto i = 0u; i < input.size(); i++) {
    auto res = sumsTo(input.begin() + i, input.end(), target);

    if (res.isValid) return res.min + res.max;
  }

  throw std::runtime_error("Not expected");
}

int main() {
  auto input = getInput("2020/09/inputs/input.txt");
  std::cout << "Problem #1: : " << std::to_string(problem1(input, 25)) << '\n';
  std::cout << "Problem #2: : " << std::to_string(problem2(input, 25)) << '\n';
}
