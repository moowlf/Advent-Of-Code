// Cpp Includes
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>

bool contains(const std::string &str, const std::regex &re) { return std::regex_search(str, re); }

std::string getRuleBagColor(const std::string &str) {
  const std::regex color("(.*)\\sbags\\scontain.*");
  std::smatch match;

  if (std::regex_match(str, match, color)) {
    return match[1].str();
  }

  throw std::runtime_error("Failed to match color bag.");
}

struct bag_t {
  std::string color;
  unsigned int number;
};

std::vector<bag_t> getContainedBags(const std::string &str) {

  std::vector<bag_t> bags;
  std::regex re("(\\d)\\s(\\w+\\s\\w+)");
  std::smatch sm;

  auto x = str.cbegin();
  while (std::regex_search(x, str.end(), sm, re)) {

    for (auto i = 0u; i < sm.size(); i += 3) {
      std::string color   = sm[i + 2];
      unsigned int number = std::stoi(sm[i + 1].str());
      bags.emplace_back(color, number);
    }

    x = sm.suffix().first;
  }

  return bags;
}

struct rule_t {
  std::string color;
  std::vector<bag_t> containedColors;

  static rule_t buildFromString(const std::string &str) {

    rule_t rule;
    rule.color = getRuleBagColor(str);

    const std::regex noOtherBag("no\\sother\\sbags");
    if (!contains(str.data(), noOtherBag)) {
      rule.containedColors = getContainedBags(str);
    }

    return rule;
  };
};

std::unordered_map<std::string, rule_t> getInput(std::string_view file) {
  std::ifstream input(file.data());
  std::unordered_map<std::string, rule_t> rules;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string line; std::getline(input, line);) {
    auto rule = rule_t::buildFromString(line);
    rules.insert({rule.color, rule});
  }
  return rules;
}

unsigned int problem1(const std::unordered_map<std::string, rule_t> &rules) {

  using r_map  = std::unordered_map<std::string, rule_t>;
  using r_bags = std::vector<bag_t>;

  std::unordered_map<std::string, bool> dp;

  std::function<bool(const r_map &, const r_bags &)> checkForGolden = [&](const r_map &rules, const r_bags &bags) {
    bool possibility = false;
    for (auto &bag : bags) {
      if (!dp.contains(bag.color)) {
        dp[bag.color] = checkForGolden(rules, rules.at(bag.color).containedColors);
      }
      possibility |= dp[bag.color] || bag.color == "shiny gold";
    }

    return possibility;
  };

  for (auto &[color, bagc] : rules) {
    if (dp.contains(bagc.color)) continue;
    dp[bagc.color] = checkForGolden(rules, bagc.containedColors);
  }

  return std::count_if(dp.begin(), dp.end(), [](auto &entry) { return entry.second; });
};

int problem2(const std::unordered_map<std::string, rule_t> &rules) {

  using r_map = std::unordered_map<std::string, rule_t>;
  using r_dp  = std::function<int(const r_map &, std::string)>;
  std::unordered_map<std::string, int> dp;

  r_dp getAllNecessaryBags = [&](const r_map &rules, std::string colorToSearch) {
    int bags{0};

    for (auto &[color, number] : rules.at(colorToSearch).containedColors) {
      auto necessaryBags = dp.contains(color) ? dp[color] : getAllNecessaryBags(rules, color);
      bags += number * (1 + necessaryBags);
    }

    dp[colorToSearch] = bags;
    return bags;
  };

  return getAllNecessaryBags(rules, "shiny gold");
}

int main() {
  auto input = getInput("2020/07/inputs/input.txt");
  std::cout << "Problem #1: : " << std::to_string(problem1(input)) << '\n';
  std::cout << "Problem #2: : " << std::to_string(problem2(input)) << '\n';
}