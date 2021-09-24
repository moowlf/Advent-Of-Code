
// CPP Includes
#include <cstdint>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <unordered_map>

class RuleNode {

public:
  explicit RuleNode(std::string data) {

    // Check if is final
    is_final = data.starts_with('\"');
    if (is_final) {
      final_char = data[1];
      return;
    }

    // Children Rules

    // options will have atleast one "option"
    options.emplace_back();

    std::regex re("(\\d+)|(\\|)");
    std::smatch smatch;

    while (std::regex_search(data, smatch, re)) {

      auto current_match = smatch.str();
      if (current_match != "|") {
        options.back().emplace_back(std::stoi(current_match));
      } else {
        options.emplace_back();
      }

      data = smatch.suffix();
    }
  }

  const std::vector<std::vector<uint16_t>> &getChildren() const { return options; }
  bool isFinal() const { return is_final; }
  char getFinalChar() const { return final_char; }

private:
  bool is_final{false};
  char final_char{'\0'};
  std::vector<std::vector<uint16_t>> options;
};

class MonsterMessages {

public:
  using RuleBook = std::unordered_map<uint16_t, RuleNode>;
  using Messages = std::vector<std::string>;

  explicit MonsterMessages(std::string_view file) {

    std::ifstream input(file.data());
    std::unordered_map<uint16_t, RuleNode> content;

    if (!input.is_open()) throw std::runtime_error("File opening failed.");

    bool parsing_rules{true};
    for (std::string line; std::getline(input, line);) {

      //  Switch condition: Rules -> Strings
      if (line.empty()) {
        parsing_rules = false;
        continue;
      }

      if (parsing_rules) {
        parse_rule(line);
        continue;
      }

      messages.push_back(line);
    }
  }

  const RuleBook &getRuleBook() const { return book; }

  uint64_t run() const {
    auto r = compile_regex(0);

    std::regex re(r);
    std::smatch match;

    uint64_t incr = 0;
    for (const auto &message : messages) {

      if (std::regex_match(message, match, re)) incr++;
    }
    return incr;
  }

private:
  void parse_rule(std::string &line) {

    std::regex regex("(\\d+):\\s(.*)");
    std::smatch base;

    if (std::regex_match(line, base, regex)) {
      uint16_t ruleID = std::stoi(base[1]);

      if (!book.contains(ruleID)) {
        book.try_emplace(ruleID, base[2].str());
      }
    }
  }

  std::string compile_regex(uint16_t ruleID, uint16_t current_level = 0) const {
    std::string currentRegex = "";

    if (book.at(ruleID).isFinal()) return std::string(1, book.at(ruleID).getFinalChar());

    const auto &children = book.at(ruleID).getChildren();
    bool isOrBranch      = children.size() == 2;
    if (isOrBranch) {
      currentRegex += "(";
    }

    bool firstRun = true;
    for (auto &option : children) {

      for (auto &rule : option) {
        currentRegex += compile_regex(rule, current_level++);

        if (current_level > 40) break; // try and error
      }

      if (isOrBranch && firstRun) currentRegex += "|";
      firstRun = false;
    }

    if (isOrBranch) {
      currentRegex += ")";
    }

    return currentRegex;
  }

private:
  RuleBook book;
  Messages messages;
};

uint64_t problem1(const MonsterMessages &input) { return input.run(); }

uint64_t problem2(const MonsterMessages &input) { return input.run(); }

int main() {

  auto input = MonsterMessages("2020/19/inputs/input_1.txt");
  std::cout << "Problem #1: " << std::to_string(problem1(input)) << '\n';

  input = MonsterMessages("2020/19/inputs/input_2.txt");
  std::cout << "Problem #2: " << std::to_string(problem2(input)) << '\n';
}