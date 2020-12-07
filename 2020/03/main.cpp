
// Cpp Includes
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <cstdint>

using puzzle = std::vector<std::vector<char>>;

puzzle getInput(std::string_view inputFile) {
  std::ifstream input(inputFile.data());
  puzzle content;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string value; std::getline(input, value);) {
    content.emplace_back();
    for (char ch : value) {
      content.back().push_back(ch);
    }
    std::cout << '\n';
  }

  return content;
}

uint64_t transverseWithSlope(const puzzle &map, unsigned int xSlope, unsigned ySlope) {
  // Map variables
  const char treeChar               = '#';
  const unsigned int xPatternLength = map.at(0).size();
  const unsigned int yMax           = map.size();

  // Hero Variables
  uint64_t numOfTrees         = 0;
  unsigned int yPosition = 0, xPosition = 0;

  while (yPosition < yMax) {
    if (map.at(yPosition).at(xPosition) == treeChar) numOfTrees++;

    yPosition += ySlope;
    xPosition = (xPosition + xSlope) % xPatternLength;
  }

  return numOfTrees;
}

uint64_t problem1(const puzzle &input) { return transverseWithSlope(input, 3, 1); }

uint64_t problem2(const puzzle &input) {

  uint64_t s11 = transverseWithSlope(input, 1, 1);
  uint64_t s31 = transverseWithSlope(input, 3, 1);
  uint64_t s51 = transverseWithSlope(input, 5, 1);
  uint64_t s71 = transverseWithSlope(input, 7, 1);
  uint64_t s12 = transverseWithSlope(input, 1, 2);

  return s11 * s31 * s51 * s71 * s12;
}

int main() {
  auto input = getInput("2020/03/inputs/input.txt");
  std::cout << "Solution #1: " << std::to_string(problem1(input)) << '\n';
  std::cout << "Solution #2: " << std::to_string(problem2(input)) << '\n';
}
