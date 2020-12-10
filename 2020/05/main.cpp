
// Cpp Includes
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

std::vector<std::string> getInput(std::string_view inputFile) {
  std::ifstream input(inputFile.data());
  std::vector<std::string> content;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string line; std::getline(input, line);) {
    content.emplace_back(line);
  }
  return content;
}

uint decodeSeatID(std::string_view str) {

  // Row
  std::string_view codedRow = str.substr(0, 7);
  uint min = 0, max = 127;
  for (char ch : codedRow) {

    if (ch == 'F')
      max = min + (max - min) / 2;
    else if (ch == 'B')
      min += (max - min) / 2 + 1;
  }

  uint row = std::min(min, max);

  // Column
  std::string_view codedColumn = str.substr(7, 10);
  min = 0, max = 7;

  for (char ch : codedColumn) {
    if (ch == 'L')
      max = min + (max - min) / 2;
    else if (ch == 'R')
      min += (max - min) / 2 + 1;
  }

  uint column = std::max(min, max);
  return row * 8 + column;
}

int problem1(const std::vector<std::string> &input) {

  uint max = 0;
  for (auto &codedSeatID : input) {
    auto seatID = decodeSeatID(codedSeatID);
    if (seatID > max) max = seatID;
  }
  return max;
}

int problem2(const std::vector<std::string> &input) {

  std::vector<int> orderedSeats;
  orderedSeats.reserve(input.size());

  for (auto &codedID : input) {
    orderedSeats.push_back(decodeSeatID(codedID));
  }
  std::sort(orderedSeats.begin(), orderedSeats.end());

  for (auto i = 1u; i < orderedSeats.size() - 2; i++) {
    auto previousID = orderedSeats.at(i - 1);
    auto currentID  = orderedSeats.at(i);
    if (currentID == previousID + 1) continue;
    if (previousID + 2 == currentID) return previousID + 1;
  }
  return -1;
}

int main() {
  auto input = getInput("2020/05/inputs/input.txt");
  std::cout << "Problem #1: : " << std::to_string(problem1(input)) << '\n';
  std::cout << "Problem #2: : " << std::to_string(problem2(input)) << '\n';
}
