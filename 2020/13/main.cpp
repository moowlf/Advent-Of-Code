
// Cpp Includes
#include <cstdint>
#include <fstream>
#include <iostream>
#include <regex>
#include <vector>

struct bus {
  int id;
  int deltaT;
};

struct Stop {
  std::vector<bus> buses;
  uint64_t min_timestamp;
};

Stop getBuses(std::string_view filepath) {

  std::ifstream inputFile(filepath.data());
  if (!inputFile.is_open()) throw std::runtime_error("File couldn't be open!");

  Stop stop;
  std::string line;
  std::getline(inputFile, line);

  // Get Min Timestamp
  stop.min_timestamp = std::stoull(line);

  // Get All buses IDs
  std::regex re("(\\d+|x)");
  std::getline(inputFile, line);

  int deltaT = 0;
  for (std::smatch match; std::regex_search(line, match, re); line = match.suffix()) {
    if (match[0] != 'x') stop.buses.emplace_back(std::stoull(match[0]), deltaT);
    deltaT++;
  }

  return stop;
}

uint64_t problem1(const Stop &stop) {

  for (auto i = stop.min_timestamp;; i++) {
    for (auto &bus : stop.buses) {
      if (i % bus.id == 0) {
        return bus.id * (i - stop.min_timestamp);
      }
    }
  }
  return 0;
}

uint64_t problem2(const Stop &stop) {

  uint64_t increment        = stop.buses.front().id;
  uint64_t currentTimestamp = 0;
  uint64_t currentBusID     = 1;

  while (currentBusID != stop.buses.size()) {

    auto nextStop = currentTimestamp + stop.buses.at(currentBusID).deltaT;

    if (nextStop % stop.buses.at(currentBusID).id == 0) {
      increment *= stop.buses.at(currentBusID).id;
      currentBusID++;
    }

    currentTimestamp += increment;
  }

  return currentTimestamp - increment;
}

int main() {

  auto stop = getBuses("2020/13/inputs/input.txt");

  std::cout << "Problem #1: : " << std::to_string(problem1(stop)) << '\n';
  std::cout << "Problem #2: : " << std::to_string(problem2(stop)) << '\n';
}