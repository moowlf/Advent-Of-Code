
// CPP Includes
#include <cstdint>
#include <fstream>
#include <iostream>
#include <numeric>
#include <queue>
#include <regex>
#include <stack>
#include <string>
#include <vector>

// ----------------------------------------------------------------------------
// Math Parser
// ----------------------------------------------------------------------------
enum class OperandType { OpenParenthesis, CloseParenthesis, Number, Sum, Subtract, Multiplication, Division };

struct Operand {
  OperandType type;
  std::optional<int64_t> value;
};

class MathExpression {
  std::vector<Operand> operands;

public:
  int64_t compute(bool precedence = false) const {

    std::queue<Operand> output;
    std::stack<Operand> operators;

    for (const auto &operand : operands) {

      if (operand.type == OperandType::Number) {
        output.push(operand);
        continue;
      } else if (operand.type == OperandType::OpenParenthesis) {
        operators.push(operand);
        continue;
      } else if (operand.type == OperandType::CloseParenthesis) {

        while (!operators.empty() && operators.top().type != OperandType::OpenParenthesis) {
          output.push(operators.top());
          operators.pop();
        }

        if (operators.empty() || operators.top().type != OperandType::OpenParenthesis)
          throw std::runtime_error("Parenthesis mismatch.");
        else [[likely]]
          operators.pop();
      } else {
        // it's one of +-/*
        while (!operators.empty() && operators.top().type != OperandType::OpenParenthesis) {

          if (precedence && operators.top().type == OperandType::Multiplication && operand.type == OperandType::Sum) {
            // if top has lower precedence
            break;
          }

          output.push(operators.top());
          operators.pop();
        }
        operators.push(operand);
      }
    }

    while (!operators.empty()) {
      output.push(operators.top());
      operators.pop();
    }

    std::stack<int64_t> calculator;

    while (!output.empty()) {

      if (output.front().type == OperandType::Number) {
        calculator.push(output.front().value.value());
        output.pop();
        continue;
      }

      auto value_1 = calculator.top();
      calculator.pop();
      auto value_2 = calculator.top();
      calculator.pop();

      switch (output.front().type) {
        case OperandType::Sum:
          output.pop();
          calculator.push(value_1 + value_2);
          break;
        case OperandType::Subtract:
          output.pop();
          calculator.push(value_2 - value_1);
          break;
        case OperandType::Multiplication:
          output.pop();
          calculator.push(value_1 * value_2);
          break;
        case OperandType::Division:
          output.pop();
          calculator.push(value_2 / value_1);
          break;
        default:
          throw std::runtime_error("Unexpected value!.");
      }

    }

    return calculator.top();
  }

  static void parse_strings(MathExpression &math, const std::string &expr) {

    std::regex re("(\\d+|[\\*\\\\+-]|[\\(\\)])");

    static std::unordered_map<char, OperandType> lookup_table{
        {'+', OperandType::Sum},       {'-', OperandType::Subtract},        {'*', OperandType::Multiplication},
        {'\\', OperandType::Division}, {'(', OperandType::OpenParenthesis}, {')', OperandType::CloseParenthesis},
    };

    // Looping through all the groups
    auto operands_begin = std::sregex_iterator(expr.begin(), expr.end(), re);

    while (operands_begin != std::sregex_iterator()) {
      const auto &match = (*operands_begin).str();

      if (match.size() != 1) {
        // Only numbers can have a match with more than one character
        math.emplace_back(OperandType::Number, std::stoi(match));
      } else {
        // In this case, it means the match can either be a number or a mathematical symbol.
        if (lookup_table.contains(match[0]))
          math.emplace_back(lookup_table[match[0]]);
        else
          math.emplace_back(OperandType::Number, std::stoi(match));
      }

      ++operands_begin;
    }

    return;
  }

private:
  void emplace_back(OperandType op) {
    if (op == OperandType::Number) throw std::runtime_error("OperandType::Number expects a value.");

    operands.emplace_back(op);
  }

  void emplace_back(OperandType op, int64_t value) { operands.emplace_back(op, value); }
};

std::vector<MathExpression> getInput(std::string_view file) {
  std::ifstream input(file.data());
  std::vector<MathExpression> content;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string value; std::getline(input, value);) {
    content.emplace_back();
    MathExpression::parse_strings(content.back(), value);
  }

  return content;
}

int64_t problem1(const std::vector<MathExpression> &input) {

  auto sum = [](int64_t soFar, const MathExpression& expr) {
    return soFar + expr.compute();
  };

  return std::accumulate(input.begin(), input.end(), (int64_t)0, sum);
}

uint64_t problem2(const std::vector<MathExpression> &input) {

  bool withPrecedence = true;
  auto sum = [withPrecedence](int64_t soFar, const MathExpression& expr) {
    return soFar + expr.compute(withPrecedence);
  };

  return std::accumulate(input.begin(), input.end(), (int64_t)0, sum);

}

int main() {

  auto input = getInput("2020/18/inputs/input.txt");

  std::cout << "Problem #1: : " << std::to_string(problem1(input)) << '\n';
  std::cout << "Problem #2: : " << std::to_string(problem2(input)) << '\n';
}