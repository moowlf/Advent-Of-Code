
// Cpp Includes
#include <algorithm>
#include <fstream>
#include <iostream>
#include <regex>
#include <vector>

std::vector<std::string> getInputs(std::string_view inputFile) {
  std::ifstream input(inputFile.data());
  std::vector<std::string> content;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string value; std::getline(input, value);) {
    content.push_back(value);
  }

  return content;
}

int cntBasedOnPolicy(const std::vector<std::string> &input, std::function<bool(int, int, char, std::string)> policy) {

  std::regex re("(\\d+)-(\\d+)\\s(\\w):\\s(\\w+)");
  std::smatch match;
  int numberOfPasswordThatMatch = 0;

  for (auto &m : input) {
    if (std::regex_match(m, match, re)) {

      auto min  = stoi(match[1]);
      auto max  = stoi(match[2]);
      auto ch   = match[3].str()[0];
      auto pswd = match[4].str();

      if (policy(min, max, ch, pswd)) {
        numberOfPasswordThatMatch++;
      }
    }
  }
  return numberOfPasswordThatMatch;
}

int problem1(const std::vector<std::string> &input) {

  auto checkPolicy = [](int minimum, int maximum, char ch, std::string_view password) {
    auto numberOfChInPassword = std::count(password.cbegin(), password.cend(), ch);
    return minimum <= numberOfChInPassword && numberOfChInPassword <= maximum;
  };

  return cntBasedOnPolicy(input, checkPolicy);
}

int problem2(const std::vector<std::string> &input) {

  auto checkPolicy = [](int id1, int id2, char ch, std::string_view password) {
    return (password.at(id1 - 1) == ch && password.at(id2 - 1) != ch) ||
           (password.at(id1 - 1) != ch && password.at(id2 - 1) == ch);
  };

  return cntBasedOnPolicy(input, checkPolicy);
}

int main() {

  auto input = getInputs("2020/02/inputs/input.txt");
  std::cout << "Solution #1: " << std::to_string(problem1(input)) << '\n';
  std::cout << "Solution #2: " << std::to_string(problem2(input)) << '\n';
}
