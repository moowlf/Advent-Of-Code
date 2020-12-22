
// Cpp Includes
#include <fstream>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

class SeatLayout {

protected:
  std::vector<std::vector<char>> layout;

public:
  SeatLayout(std::string_view file) {

    std::ifstream inputFile(file.data());
    if (!inputFile.is_open()) throw std::runtime_error("File couldn't be open!");

    for (std::string line; std::getline(inputFile, line);) {
      layout.emplace_back();
      std::copy(line.begin(), line.end(), std::back_inserter(layout.back()));
    }
  }

  virtual bool update() {
    auto preChangeLayout = layout;
    for (auto row = 0u; row < layout.size(); row++) {
      for (auto column = 0u; column < layout.back().size(); column++) {
        auto numOfOcuppiedSeats = getNumOfOccupiedAdjacentSeats(row, column);
        auto currentState       = layout.at(row).at(column);

        if (currentState == 'L' && numOfOcuppiedSeats == 0)
          preChangeLayout.at(row).at(column) = '#';
        else if (currentState == '#' && numOfOcuppiedSeats >= 4)
          preChangeLayout.at(row).at(column) = 'L';
      }
    }

    bool isEqual = layout == preChangeLayout;
    layout       = preChangeLayout;
    return isEqual;
  };

  int countOccupiedSeats() const {
    auto occ = 0u;
    for (auto row = 0u; row < layout.size(); row++) {
      occ += std::count(layout.at(row).begin(), layout.at(row).end(), '#');
    }
    return occ;
  }

protected:
  virtual unsigned int getNumOfOccupiedAdjacentSeats(int row, int column) const {

    int total{0};
    const int maxColumn = layout.back().size();

    if (row - 1 >= 0) {
      // Upper row
      if (column - 1 >= 0 && layout[row - 1][column - 1] == '#') total++;
      if (layout[row - 1][column] == '#') total++;
      if (column + 1 < maxColumn && layout[row - 1][column + 1] == '#') total++;
    }

    // Sides
    if (column - 1 >= 0 && layout[row][column - 1] == '#') total++;
    if (column + 1 < maxColumn && layout[row][column + 1] == '#') total++;

    if (row + 1 < layout.size()) {
      // Lower Row
      if (column - 1 >= 0 && layout[row + 1][column - 1] == '#') total++;
      if (layout[row + 1][column] == '#') total++;
      if (column + 1 < maxColumn && layout[row + 1][column + 1] == '#') total++;
    }

    return total;
  }

  friend std::ostream &operator<<(std::ostream &, const SeatLayout &);
};

class SeatLayoutProb2 : public SeatLayout {

public:
  using SeatLayout::SeatLayout;

  bool update() override {
    auto preChangeLayout = layout;
    for (auto row = 0u; row < layout.size(); row++) {
      for (auto column = 0u; column < layout.back().size(); column++) {
        auto numOfOcuppiedSeats = getNumOfOccupiedAdjacentSeats(row, column);
        auto currentState       = layout.at(row).at(column);

        if (currentState == 'L' && numOfOcuppiedSeats == 0)
          preChangeLayout.at(row).at(column) = '#';
        else if (currentState == '#' && numOfOcuppiedSeats >= 5)
          preChangeLayout.at(row).at(column) = 'L';
      }
    }

    bool isEqual = layout == preChangeLayout;
    layout       = preChangeLayout;
    return isEqual;
  }

protected:
  unsigned int getNumOfOccupiedAdjacentSeats(int row, int column) const override {
    std::function<int(int, int, int, int)> propagateVision = [&](int curRow, int curCol, int deltaRow,
                                                                 int deltaColumn) {
      if (curRow + deltaRow < 0 || curRow + deltaRow >= layout.size())
        return 0;
      else if (curCol + deltaColumn < 0 || curCol + deltaColumn >= layout.back().size())
        return 0;

      char seatSeen = layout.at(curRow + deltaRow).at(curCol + deltaColumn);
      if (seatSeen == 'L')
        return 0;
      else if (seatSeen == '#')
        return 1;
      else
        return propagateVision(curRow + deltaRow, curCol + deltaColumn, deltaRow, deltaColumn);
    };

    unsigned int total{0};

    // TOP-LEFT
    total += propagateVision(row, column, -1, -1);

    // TOP-CENTER
    total += propagateVision(row, column, -1, 0);

    // TOP-RIGHT
    total += propagateVision(row, column, -1, 1);

    // MID-LEFT
    total += propagateVision(row, column, 0, -1);

    // MID-RIGHT
    total += propagateVision(row, column, 0, 1);

    // BOTTOM-LEFT
    total += propagateVision(row, column, 1, -1);

    // BOTTOM-CENTER
    total += propagateVision(row, column, 1, 0);

    // BOTTOM-RIGHT
    total += propagateVision(row, column, 1, 1);

    return total;
  }
};

std::ostream &operator<<(std::ostream &out, const SeatLayout &layout) {

  for (auto row = 0u; row < layout.layout.size(); row++) {
    for (auto column = 0u; column < layout.layout.size(); column++) {
      out << layout.layout.at(row).at(column);
    }
    out << '\n';
  }
  return out;
}

int problem1(SeatLayout layout) {
  while (!layout.update()) {
  }
  return layout.countOccupiedSeats();
}

int problem2(SeatLayoutProb2 layout) {
  while (!layout.update()) {
  }
  return layout.countOccupiedSeats();
}

int main() {

  SeatLayout layout("2020/11/inputs/input.txt");
  std::cout << "Problem #1: : " << std::to_string(problem1(layout)) << '\n';

  SeatLayoutProb2 prob2("2020/11/inputs/input.txt");
  std::cout << "Problem #2: : " << std::to_string(problem2(prob2)) << '\n';
}
