
// Cpp Includes
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>

struct command_t {
  enum class Type { acc, jmp, nop } type;
  int value;

  explicit command_t(const std::string &str) {
    std::regex re("(\\w{3})\\s([+|-]\\d+)");
    std::smatch match;

    if (std::regex_match(str, match, re)) {

      for (auto i = 0u; i < match.size(); i++) {
        this->value = std::stoi(match[2]);

        if (match[1] == "acc")
          this->type = command_t::Type::acc;
        else if (match[1] == "jmp")
          this->type = command_t::Type::jmp;
        else if (match[1] == "nop")
          this->type = command_t::Type::nop;
        else
          throw std::runtime_error("Invalid Command!");
      }
    }
  }
};

std::vector<command_t> getInput(std::string_view filepath) {

  std::ifstream file(filepath.data());
  std::vector<command_t> commands;

  if (!file.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string line; std::getline(file, line);) {
    commands.emplace_back(line);
  }

  return commands;
}

struct exit_t {
  bool successful{true};
  int acc{0};
};

exit_t runBootCode(const std::vector<command_t> &input) {

  exit_t exit;

  std::vector<bool> executed(input.size(), false);
  for (auto i = 0u; i < input.size(); i++) {

    if (executed.at(i)) {
      exit.successful = false;
      break;
    }

    executed.at(i) = true;
    switch (input.at(i).type) {
      case command_t::Type::acc:
        exit.acc += input.at(i).value;
        break;
      case command_t::Type::jmp:
        i += -1 + input.at(i).value;
        break;
      case command_t::Type::nop:
        break;
      default:
        break;
    }
  }

  return exit;
}

int problem1(const std::vector<command_t> &input) { return runBootCode(input).acc; }

int problem2(const std::vector<command_t> &input) {

  auto inputCP = input;
  for (auto i = 0u; i < input.size(); i++) {
    if (inputCP.at(i).type == command_t::Type::nop && inputCP.at(i).value != 0) {
      inputCP.at(i).type = command_t::Type::jmp;
      auto exit          = runBootCode(inputCP);

      if (exit.successful) return exit.acc;
      inputCP.at(i).type = command_t::Type::nop;
    } else if (inputCP.at(i).type == command_t::Type::jmp) {
      inputCP.at(i).type = command_t::Type::nop;
      auto exit          = runBootCode(inputCP);

      if (exit.successful) return exit.acc;
      inputCP.at(i).type = command_t::Type::jmp;
    }
  }
  return -1;
}

int main() {

  auto input = getInput("2020/08/inputs/input.txt");
  std::cout << "Problem #1: : " << std::to_string(problem1(input)) << '\n';
  std::cout << "Problem #2: : " << std::to_string(problem2(input)) << '\n';
}