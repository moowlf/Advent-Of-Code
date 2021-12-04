
// CPP Includes
#include <algorithm>
#include <cstdint>
#include <fstream>
#include <functional>
#include <iostream>
#include <set>
#include <string>
#include <unordered_map>
#include <vector>

// --------------------------------------------------------------------------------------------------------------------

template <typename T, typename U>
struct std::hash<std::pair<T, U>> {

  size_t operator()(const std::pair<T, U> &pair) const {
    auto hash1 = hash<T>{}(pair.first);
    auto hash2 = hash<U>{}(pair.second);
    return hash1 ^ hash2;
  }
};

template <typename T, typename U>
std::pair<T, U> &operator+=(std::pair<T, U> &lhs, const std::pair<T, U> &rhs) {

  lhs.first += rhs.first;
  lhs.second += rhs.second;
  return lhs;
}

// --------------------------------------------------------------------------------------------------------------------

enum Color { White, Black };

enum class Coordinates {
  E,  // East
  Se, // South East
  Sw, // South West
  W,  // West
  Nw, // North West
  Ne  // North East
};

constexpr std::pair<int, int> convert_coordinates(Coordinates c) {
  // X, Y
  std::pair<int, int> translated_coordinates;

  switch (c) {
    case Coordinates::E:
      translated_coordinates.first  = 1;
      translated_coordinates.second = 0;
      break;
    case Coordinates::Se:
      translated_coordinates.first  = 0;
      translated_coordinates.second = 1;
      break;
    case Coordinates::Ne:
      translated_coordinates.first  = 1;
      translated_coordinates.second = -1;
      break;
    case Coordinates::W:
      translated_coordinates.first  = -1;
      translated_coordinates.second = 0;
      break;
    case Coordinates::Sw:
      translated_coordinates.first  = -1;
      translated_coordinates.second = 1;
      break;
    case Coordinates::Nw:
      translated_coordinates.first  = 0;
      translated_coordinates.second = -1;
      break;
    default:
      break;
  }

  return translated_coordinates;
}

std::unordered_map<std::pair<int, int>, bool> parse_tiles(std::vector<std::string> const &_) {

  static std::unordered_map<std::string, std::pair<int, int>> coordinates = {
      {"e", convert_coordinates(Coordinates::E)},   {"se", convert_coordinates(Coordinates::Se)},
      {"sw", convert_coordinates(Coordinates::Sw)}, {"w", convert_coordinates(Coordinates::W)},
      {"nw", convert_coordinates(Coordinates::Nw)}, {"ne", convert_coordinates(Coordinates::Ne)},
  };

  std::unordered_map<std::pair<int, int>, bool> tiles;

  for (const auto &directions : _) {

    std::pair<int, int> current_tile;

    for (auto ch = 0; ch < directions.size(); ch++) {

      char current_char = directions.at(ch);
      if (current_char == 'e' || current_char == 'w') {
        std::string test;
        test += current_char;
        current_tile += coordinates.at(test);
        continue;
      }

      char next_char = directions.at(ch + 1);
      std::string complex_dir;
      complex_dir += current_char;
      complex_dir += next_char;

      current_tile += coordinates.at(complex_dir);
      ch++;
    }

    if (tiles.contains(current_tile)) {
      tiles.at(current_tile) = !tiles.at(current_tile);
    } else {
      tiles.try_emplace(current_tile, true);
    }
  }

  return tiles;
}

uint64_t problem1(const std::vector<std::string> &input) {

  auto tiles = parse_tiles(input);

  return std::ranges::count_if(tiles, [](auto it) { return it.second; });
}

uint64_t problem2(const std::vector<std::string> &input) {

  auto tiles = parse_tiles(input);

  auto count_neighbour_black = [&tiles](std::pair<int, int> const &pos) {
    uint8_t count = 0;

    auto &x = pos.first;
    auto &y = pos.second;

    // E
    if (tiles.contains({x + 1, y}) && tiles[{x + 1, y}]) {
      count++;
    }

    // NE
    if (tiles.contains({x + 1, y - 1}) && tiles[{x + 1, y - 1}]) {
      count++;
    }

    // NW
    if (tiles.contains({x, y - 1}) && tiles[{x, y - 1}]) {
      count++;
    }

    // W
    if (tiles.contains({x - 1, y}) && tiles[{x - 1, y}]) {
      count++;
    }

    // SE
    if (tiles.contains({x, y + 1}) && tiles[{x, y + 1}]) {
      count++;
    }

    // SW
    if (tiles.contains({x - 1, y + 1}) && tiles[{x - 1, y + 1}]) {
      count++;
    }

    return count;
  };

  auto generate_new_tils_positions = [](std::pair<int, int> const &pos) {
    auto &x = pos.first;
    auto &y = pos.second;

    std::set<std::pair<int, int>> tiles{
        {x + 1, y},     // E
        {x, y + 1},     // Se
        {x + 1, y - 1}, // Ne
        {x - 1, y},     // W
        {x - 1, y + 1}, // Sw
        {x, y - 1},     // Nw
        pos};

    return tiles;
  };

  for (auto i = 0; i < 100; i++) {

    std::set<std::pair<int, int>> new_tiles;
    decltype(tiles) next_tiles;

    // find new tiles
    for (auto &tile : tiles) {
      new_tiles.merge(generate_new_tils_positions(tile.first));
    }

    // update our new map with new tiles
    for (auto &tile : new_tiles) {

      auto black_neighbours = count_neighbour_black(tile);
      Color current_color;

      if (!tiles.contains(tile))
        current_color = Color::White;
      else
        current_color = static_cast<Color>(tiles[tile]);

      if (current_color == Color::White && black_neighbours == 0) continue;

      next_tiles[tile] = static_cast<int>(current_color);

      if (current_color == Color::Black && (black_neighbours == 0 || black_neighbours > 2)) {
        // if it is black and black_neighbours is 0 or greater than 2, tile
        // flips its color
        next_tiles[tile] = static_cast<int>(Color::White);
        continue;
      }

      if (current_color == Color::White && black_neighbours == 2) {
        next_tiles[tile] = true;
      }
    }

    tiles = next_tiles;
  }

  return std::ranges::count_if(tiles, [](auto it) { return it.second; });
  ;
}

// --------------------------------------------------------------------------------------------------------------------

std::vector<std::string> get_input(std::string_view inputFile) {
  std::ifstream input(inputFile.data());
  std::vector<std::string> content;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string value; std::getline(input, value);) {
    content.push_back(value);
  }

  return content;
}

int main() {

  auto input = get_input("2020/24/inputs/input.txt");
  std::cout << "Problem #1: : " << problem1(input) << '\n';
  std::cout << "Problem #2: : " << problem2(input) << '\n';
}
