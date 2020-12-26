
// Cpp Includes
#include <cmath>
#include <fstream>
#include <iostream>
#include <numbers>
#include <string>
#include <tuple>
#include <vector>

class Ship {

protected:
  struct instruction_t {
    enum class type { N, S, E, W, L, R, F } type;
    int value;
  };

  std::vector<instruction_t> instructions;

  struct Position {
    double x;
    double y;
  } position;

  std::tuple<int, int> direction;

public:
  explicit Ship(std::string_view filepath) {
    std::ifstream inputFile(filepath.data());
    if (!inputFile.is_open()) throw std::runtime_error("File couldn't be open!");

    for (std::string line; std::getline(inputFile, line);) {

      instructions.emplace_back();

      switch (line[0]) {
        case 'N':
          instructions.back().type = instruction_t::type::N;
          break;
        case 'S':
          instructions.back().type = instruction_t::type::S;
          break;
        case 'E':
          instructions.back().type = instruction_t::type::E;
          break;
        case 'W':
          instructions.back().type = instruction_t::type::W;
          break;
        case 'L':
          instructions.back().type = instruction_t::type::L;
          break;
        case 'R':
          instructions.back().type = instruction_t::type::R;
          break;
        case 'F':
          instructions.back().type = instruction_t::type::F;
          break;
        default:
          throw std::runtime_error("Failed to read instructions");
          break;
      }

      instructions.back().value = std::stoi(line.substr(1));
    }

    position  = {.x = 0, .y = 0};
    direction = {1, 0};
  }

  virtual int executeInstructions() {
    std::for_each(instructions.begin(), instructions.end(), [&](const instruction_t &instr) {
      switch (instr.type) {
        case Ship::instruction_t::type::N:
          position.y += instr.value;
          break;
        case Ship::instruction_t::type::S:
          position.y -= instr.value;
          break;
        case Ship::instruction_t::type::E:
          position.x += instr.value;
          break;
        case Ship::instruction_t::type::W:
          position.x -= instr.value;
          break;
        case Ship::instruction_t::type::L:
          rotateDegrees(instr.value);
          break;
        case Ship::instruction_t::type::R:
          rotateDegrees(-instr.value);
          break;
        case Ship::instruction_t::type::F:
          position.x += std::get<0>(direction) * instr.value;
          position.y += std::get<1>(direction) * instr.value;
          break;
        default:
          break;
      }
    });

    return std::abs(position.x) + std::abs(position.y);
  }

protected:
  virtual void rotateDegrees(int value) {
    auto degInRads = value * std::numbers::pi / 180;

    auto [curX, curY] = direction;
    direction         = {
        curX * std::cos(degInRads) - curY * std::sin(degInRads),
        curY * std::cos(degInRads) + curX * std::sin(degInRads)};
  }
};

class Ship2 : public Ship {

  Ship::Position waypoint;

public:
  explicit Ship2(std::string_view filepath) : Ship(filepath) { waypoint = {10, 1}; }

  virtual int executeInstructions() {

    std::for_each(instructions.begin(), instructions.end(), [&](const instruction_t &instr) {
      switch (instr.type) {
        case Ship::instruction_t::type::N:
          waypoint.y += instr.value;
          break;
        case Ship::instruction_t::type::S:
          waypoint.y -= instr.value;
          break;
        case Ship::instruction_t::type::E:
          waypoint.x += instr.value;
          break;
        case Ship::instruction_t::type::W:
          waypoint.x -= instr.value;
          break;
        case Ship::instruction_t::type::L:
          rotateDegrees(instr.value);
          break;
        case Ship::instruction_t::type::R:
          rotateDegrees(360 - instr.value);
          break;
        case Ship::instruction_t::type::F:
          position.x += waypoint.x * instr.value;
          position.y += waypoint.y * instr.value;
          break;
        default:
          break;
      }
    });

    return std::abs(position.x) + std::abs(position.y);
  }

protected:
  virtual void rotateDegrees(int value) override {

    double rads = value * std::numbers::pi / 180;

    Position newWaypoint = {
        .x = waypoint.x * std::cos(rads) - waypoint.y * std::sin(rads),
        .y = waypoint.x * std::sin(rads) + waypoint.y * std::cos(rads)};

    waypoint = newWaypoint;
  };
};

int problem1(Ship &ship) { return ship.executeInstructions(); }

int problem2(Ship2 &ship) { return ship.executeInstructions(); }

std::pair<int, int> d(int x, int y, int angle) {
  double rads = angle * std::numbers::pi / 180;

  return {x * std::cos(rads) - y * std::sin(rads), x * std::sin(rads) + y * std::cos(rads)};
}

int main() {

  auto ship = Ship("2020/12/inputs/input.txt");
  std::cout << "Problem #1: : " << std::to_string(problem1(ship)) << '\n';

  auto ship2 = Ship2("2020/12/inputs/input.txt");
  std::cout << "Problem #2: : " << std::to_string(problem2(ship2)) << '\n';
}
