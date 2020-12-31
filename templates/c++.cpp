
// CPP Includes
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

std::vector<uint64_t> getInput(std::string_view file) {
  std::ifstream input(file.data());
  std::vector<uint64_t> content;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string value; std::getline(input, value);) {
    content.push_back(std::stoi(value));
  }

  return content;
}

uint64_t problem1(const std::vector<uint64_t> &input) { return 0; }

uint64_t problem2(const std::vector<uint64_t> &input) { return 0; }

int main() {

  auto input = getInput("PROBLEM_FOLDER/inputs/input.txt");

  std::cout << "Problem #1: : " << std::to_string(problem1(input)) << '\n';
  std::cout << "Problem #2: : " << std::to_string(problem2(input)) << '\n';
}