
// Cpp Includes
#include <bitset>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>

class Passport {

  std::bitset<8> fieldsPresent;

public:
  enum class PassportField { byr, iyr, eyr, hgt, hcl, ecl, pid, cid };

  Passport() { fieldsPresent = 0; }

  bool isValid() const {
    auto bitsSet = fieldsPresent.count();
    return bitsSet == 8 || (bitsSet == 7 && !fieldsPresent.test(static_cast<size_t>(PassportField::cid)));
  }

  void addMatch(const std::ssub_match &key, const std::ssub_match &value) {

    auto numberLimitComparator = [](const std::string &yearStr, int min, int max) {
      auto year = std::stoi(yearStr);
      return min <= year && year <= max;
    };

    if (key.str() == "byr") {
      if (numberLimitComparator(value.str(), 1920, 2002)) fieldPresent(PassportField::byr);
    } else if (key.str() == "iyr") {
      if (numberLimitComparator(value.str(), 2010, 2020)) fieldPresent(PassportField::iyr);
    } else if (key.str() == "eyr") {
      if (numberLimitComparator(value.str(), 2020, 2030)) fieldPresent(PassportField::eyr);
    } else if (key.str() == "hgt") {
      bool isValid = false;
      auto size    = std::stoi(value.str());
      if (value.str().ends_with("cm")) {
        isValid = 150 <= size && size <= 193;
      } else if (value.str().ends_with("in")) {
        isValid = 59 <= size && size <= 76;
      }
      if (isValid) fieldPresent(PassportField::hgt);
    } else if (key.str() == "hcl") {

      std::regex hairColor("#[0-9a-f]{6}");
      std::smatch base_match;
      auto color = value.str();
      if (std::regex_match(color, base_match, hairColor)) fieldPresent(PassportField::hcl);
    } else if (key.str() == "ecl") {
      bool isValid                                = false;
      std::array<std::string, 7> allowedHairColor = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"};
      isValid                                     = std::any_of(
          allowedHairColor.begin(), allowedHairColor.end(), [&value](auto &color) { return color == value.str(); });
      if (isValid) fieldPresent(PassportField::ecl);
    } else if (key.str() == "pid") {
      std::regex pidRe("[0-9]{9}");
      std::smatch base_match;
      auto pid = value.str();
      if (std::regex_match(pid, base_match, pidRe)) fieldPresent(PassportField::pid);
    } else if (key.str() == "cid") {
      fieldPresent(PassportField::cid);
    } else
      ;
  }

private:
  void fieldPresent(PassportField field) { fieldsPresent.set(static_cast<int>(field)); }
};

std::vector<Passport> getInput(std::string_view inputFile) {
  std::ifstream input(inputFile.data());
  std::vector<Passport> content(1);

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  const std::regex re("(\\w+):([^\\s]+)");
  std::smatch matches;

  for (std::string line; std::getline(input, line);) {

    if (line.empty()) {
      content.emplace_back();
      continue;
    }

    while (std::regex_search(line, matches, re)) {
      content.back().addMatch(matches[1], matches[2]);
      line = matches.suffix();
    }
  }

  return content;
}

int solution(const std::vector<Passport> &input) {
  return std::count_if(input.begin(), input.end(), [](auto &passport) { return passport.isValid(); });
}

int main() {
  auto input = getInput("2020/04/inputs/input.txt");
  std::cout << "Solution: " << std::to_string(solution(input)) << '\n';
}
