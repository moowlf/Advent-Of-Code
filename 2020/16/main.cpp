
// CPP Includes
#include <cstdint>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <numeric>
#include <regex>
#include <sstream>
#include <string>
#include <vector>

class field_t {

  struct rule_t {
    uint16_t min;
    uint16_t max;

    rule_t(uint16_t min, uint16_t max) : min(min), max(max){};

    bool isValid(uint16_t value) const { return min <= value && value <= max; }
  };

  std::string name;
  std::vector<rule_t> rules;
  std::vector<uint16_t> possiblePositions;
  uint16_t id;

public:
  explicit field_t(const std::string &line) {

    std::regex re(R"((.+):\s(\d+)-(\d+)\sor\s(\d+)-(\d+))");
    std::smatch matches;

    if (std::regex_search(line, matches, re)) {
      name = matches[1];
      rules.emplace_back(std::stoul(matches[2]), std::stoul(matches[3]));
      rules.emplace_back(std::stoul(matches[4]), std::stoul(matches[5]));
    }

    id = 0;
  };

  bool allowed(uint16_t value) const {
    return std::any_of(rules.begin(), rules.end(), [value](auto &rule) { return rule.isValid(value); });
  }

  std::vector<uint16_t> &getPossiblePositions() { return possiblePositions; }

  void setID(uint16_t id) { this->id = id; };

  uint16_t getID() { return id; }

  std::string_view getName() { return name; }
};

class Ticket {

  std::vector<uint16_t> fields;
  bool valid;

public:
  Ticket() = default;

  explicit Ticket(std::string ticket) {
    std::regex re(R"((\d+))");

    for (std::smatch match; std::regex_search(ticket, match, re); ticket = match.suffix()) {
      fields.push_back(std::stoul(match[1]));
    }

    valid = true;
  }

  uint16_t getFieldErrorValue(const std::vector<field_t> &fieldRules) {

    for (auto it = fields.begin(); it != fields.end(); ++it) {

      bool isValid = false;
      for (auto ruleIt = fieldRules.begin(); ruleIt != fieldRules.end(); ++ruleIt) {
        isValid = ruleIt->allowed(*it);

        if (isValid) break;
      }

      if (!isValid) {
        valid = false;
        return *it;
      }
    }

    return 0;
  }

  bool isValid() const { return valid; }

  uint16_t getTicketField(int id) const { return fields.at(id); }
};

struct Data {
  std::vector<field_t> fieldValidation;
  std::vector<Ticket> tickets;
  Ticket myTicket;
};

Data getInput(std::string_view file) {

  std::ifstream input(file.data());
  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  Data data;

  // Parse fields
  std::string line;
  while (std::getline(input, line)) {
    if (line.empty()) break;
    data.fieldValidation.emplace_back(line);
  }

  std::getline(input, line); // your ticket
  std::getline(input, line); // your ticket
  data.myTicket = Ticket(line);

  std::getline(input, line); // empty line
  std::getline(input, line); // nearby tickets

  while (std::getline(input, line)) {
    if (line.empty()) break;
    data.tickets.emplace_back(line);
  }

  return data;
}

uint64_t problem1(Data &input) {

  return std::accumulate(
      input.tickets.begin(), input.tickets.end(), 0, [&rules = input.fieldValidation](uint64_t soFar, Ticket &ticket) {
        return soFar + ticket.getFieldErrorValue(rules);
      });
}

uint64_t problem2(Data &input) {

  // Clean Invalid Tickets
  auto isInvalid = [](const Ticket &ticket) { return !ticket.isValid(); };
  input.tickets.erase(std::remove_if(input.tickets.begin(), input.tickets.end(), isInvalid), input.tickets.end());

  // Get all possible states

  for (auto ruleID = 0u; ruleID < input.fieldValidation.size(); ruleID++) {
    for (auto fieldID = 0u; fieldID < input.fieldValidation.size(); fieldID++) {
      
      bool possible = std::all_of(input.tickets.begin(), input.tickets.end(), [&](const Ticket &ticket) {
        return input.fieldValidation[ruleID].allowed(ticket.getTicketField(fieldID));
      });

      if (possible) input.fieldValidation.at(ruleID).getPossiblePositions().push_back(fieldID);
    }
  }

  std::sort(input.fieldValidation.begin(), input.fieldValidation.end(), [](auto lhs, auto rhs) {
    return lhs.getPossiblePositions().size() < rhs.getPossiblePositions().size();
  });

  // Determine each position
  std::vector<bool> determineState(input.fieldValidation.size(), false);
  uint64_t departureValue = 1;
  for (auto &field : input.fieldValidation) {

    field.getPossiblePositions().erase(
        std::remove_if(
            field.getPossiblePositions().begin(), field.getPossiblePositions().end(),
            [&](int i) { return determineState[i]; }),
        field.getPossiblePositions().end());

    if (field.getPossiblePositions().size() != 1) throw std::runtime_error("Something went wrong!");

    determineState[field.getPossiblePositions().front()] = true;
    field.setID(field.getPossiblePositions().front());

    if (field.getName().starts_with("departure")) {
      departureValue *= input.myTicket.getTicketField(field.getID());
    }
  }

  return departureValue;
}

int main() {

  auto input = getInput("2020/16/inputs/input.txt");

  std::cout << "Problem #1: : " << std::to_string(problem1(input)) << '\n';
  std::cout << "Problem #2: : " << std::to_string(problem2(input)) << '\n';
}