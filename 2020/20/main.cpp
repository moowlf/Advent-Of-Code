
// CPP Includes
#include <cmath>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <unistd.h>
#include <vector>

enum Border { Top, Left, Bottom, Right, _size };

std::vector<std::vector<bool>> Rotate(const std::vector<std::vector<bool>> &matrix) {

  const auto matrix_witdh  = matrix.size();
  const auto matrix_height = matrix.front().size();

  std::vector<std::vector<bool>> rotated_matrix(matrix_height, std::vector<bool>(matrix_witdh, false));

  for (auto rowID = 0u; rowID < matrix.size(); rowID++) {
    for (auto columnID = 0u; columnID < matrix.at(rowID).size(); columnID++) {
      rotated_matrix.at(matrix_height - columnID - 1).at(rowID) = matrix.at(rowID).at(columnID);
    }
  }

  return rotated_matrix;
}

std::vector<std::vector<bool>> MirrorRotate(const std::vector<std::vector<bool>> &matrix) {

  const auto matrix_witdh  = matrix.front().size();
  const auto matrix_height = matrix.size();

  std::vector<std::vector<bool>> flipped(matrix_height, std::vector<bool>(matrix_witdh, false));

  for (auto rowID = 0u; rowID < matrix_height; rowID++) {
    for (auto columnID = 0u; columnID < matrix_witdh; columnID++) {
      flipped.at(rowID).at(columnID) = matrix.at(rowID).at(matrix_witdh - columnID - 1);
    }
  }
  return flipped;
}

std::vector<std::string> Rotate(std::vector<std::string> const &r) {

  std::vector<std::string> rotated(r.size(), std::string(r.at(0).size(), ' '));

  for (auto rowID = 0u; rowID < r.size(); rowID++) {
    for (auto columnID = 0u; columnID < r.at(rowID).size(); columnID++) {
      rotated.at(r.size() - columnID - 1).at(rowID) = r.at(rowID).at(columnID);
    }
  }

  return rotated;
}

std::vector<std::string> MirrorRotate(const std::vector<std::string> &r) {

  std::vector<std::string> flipped(r.size(), std::string(r.at(0).size(), ' '));

  for (auto rowID = 0u; rowID < r.size(); rowID++) {
    for (auto columnID = 0u; columnID < r.at(rowID).size(); columnID++) {
      flipped.at(rowID).at(columnID) = r.at(rowID).at(r.at(rowID).size() - columnID - 1);
    }
  }
  return flipped;
}

class Tile {

public:
  Tile() = default;

  Tile(const std::string &row) {

    std::regex r("\\w+\\s(\\d+):");
    std::smatch m;

    if (std::regex_match(row, m, r)) {
      _tid = std::stoul(m[1].str());
    } else {
      throw std::runtime_error("Failed to create object.");
    }
  }

  Tile(std::vector<std::vector<bool>> &&cont, unsigned int tid) : _tid{tid}, _content{std::move(cont)} {
    convertBordersToBitMasks();
  }

  void add_row(const std::string &row) {
    _content.emplace_back();

    for (char ch : row) {
      _content.back().emplace_back(ch == '#');
    }
  }

  Tile flip() const { return Tile(Rotate(_content), _tid); }

  Tile mirror_flip() const { return Tile(MirrorRotate(_content), _tid); }

  bool canMatch(const Tile *const tile, Border border) const {

    if (border == Top) return this->borderMask.at(border) == tile->borderMask.at(Bottom);

    if (border == Left) return this->borderMask.at(border) == tile->borderMask.at(Right);

    if (border == Bottom) return this->borderMask.at(border) == tile->borderMask.at(Top);

    if (border == Right) return this->borderMask.at(border) == tile->borderMask.at(Left);

    return false;
  }

  void setUsed(bool b) { _used = b; }

  bool getUsed() { return _used; }

  void convertBordersToBitMasks() {

    borderMask.resize(Border::_size);

    for (auto row = 0u; row < _content.size(); row++) {
      for (auto column = 0u; column < _content[row].size(); column++) {

        if (!_content.at(row).at(column)) continue;

        if (column == 0u) borderMask.at(Border::Left) |= (1 << row);

        if (row == 0u) borderMask.at(Border::Top) |= (1 << column);

        if (column == _content.at(row).size() - 1) borderMask.at(Border::Right) |= (1 << row);

        if (row == _content.size() - 1) borderMask.at(Border::Bottom) |= (1 << column);
      }
    }
  }

  uint64_t getID() const { return _tid; }

  char atPos(int x, int y) const {
    if (_content.at(y).at(x)) return '#';
    return '.';
  }

  uint32_t getSize() const { return _content.size(); }

private:
  unsigned int _tid;
  std::vector<std::vector<bool>> _content;
  std::vector<uint32_t> borderMask;
  bool _used{false};
};

std::vector<Tile> getInput(std::string_view file) {
  std::ifstream input(file.data());
  std::vector<Tile> content;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string value; std::getline(input, value);) {

    if (value.empty()) {
      content.back().convertBordersToBitMasks();
      // ok, time to rotate
      // content.push_back(content.back().flip());

      // -90
      content.emplace_back(content.back().flip());

      // -180
      content.emplace_back(content.back().flip());

      // -270
      content.emplace_back(content.back().flip());

      // Need to mirror flip
      content.emplace_back(content.at(content.size() - 4).mirror_flip());

      // flipped -90
      content.emplace_back(content.back().flip());

      // flipped -180
      content.emplace_back(content.back().flip());

      // flipped -270
      content.emplace_back(content.back().flip());

      continue;
    }

    if (value.starts_with('T')) {
      content.emplace_back(value);
      continue;
    }

    content.back().add_row(value);
  }

  content.back().convertBordersToBitMasks();

  return content;
}

std::vector<std::vector<Tile *>> problem1(std::vector<Tile> &input) {

  using Matrix = std::vector<std::vector<Tile *>>;
  using UsedID = std::unordered_map<uint16_t, bool>;

  // Supposing a final square image
  const uint16_t width    = static_cast<int16_t>(std::sqrt(input.size() / 8));

  // Start test all hipothesis
  Matrix matrix(width, std::vector<Tile *>(width, nullptr));
  UsedID m;

  auto can_match = [&matrix](const Tile &tile, int x, int y, Border border) {
    bool has_valid_coordinates = x >= 0 && x < static_cast<int>(matrix.size()) && y >= 0 && y < static_cast<int>(matrix.size());

    return !has_valid_coordinates || (has_valid_coordinates && matrix.at(y).at(x) == nullptr) ||
           (has_valid_coordinates && tile.canMatch(matrix.at(y).at(x), border));
  };

  std::function<bool(Matrix &, UsedID &, int)> backtracking = [&](Matrix &matrix, UsedID &m, unsigned int positionTile) {
    for (auto tileID = 0u; tileID < input.size(); tileID++) {

      if (positionTile == (matrix.size() * matrix.size())) return true;

      int y = positionTile / width;
      int x = positionTile % width;

      if (input.at(tileID).getUsed() || m.contains(input.at(tileID).getID())) continue;

      bool top_match    = can_match(input.at(tileID), x, y - 1, Border::Top);
      bool right_match  = can_match(input.at(tileID), x + 1, y, Border::Right);
      bool bottom_match = can_match(input.at(tileID), x, y + 1, Border::Bottom);
      bool left_match   = can_match(input.at(tileID), x - 1, y, Border::Left);

      if (!(top_match && right_match && bottom_match && left_match)) continue;

      matrix.at(y).at(x) = &input[tileID];
      input[tileID].setUsed(true);
      m[input[tileID].getID()] = true;
      bool gotPlaced           = backtracking(matrix, m, positionTile + 1);

      if (!gotPlaced) {
        m.erase(matrix.at(y).at(x)->getID());
        matrix.at(y).at(x) = nullptr;
        input.at(tileID).setUsed(false);
      } else
        return true;
    }

    return false;
  };

  backtracking(matrix, m, 0);
  return matrix;
}

std::vector<std::string> retrieveTileWithoutBorders(const std::vector<std::vector<Tile *>> &input) {

  // Prepare data
  const uint32_t matrix_height = (input.front().front()->getSize() - 2) * input.size();
  std::vector<std::string> matrix(matrix_height);

  // Populate strings in vectors
  const auto tileWidth  = input.front().front()->getSize();
  const auto tileHeight = input.front().front()->getSize();

  const auto noOfColumns = tileWidth * input.front().size();
  const auto noOfRows    = tileHeight * input.front().size();

  uint32_t skipped = 0u;
  for (auto rowID = 0u; rowID < noOfRows; rowID++) {

    if (rowID % tileHeight == 0 || rowID % tileHeight == tileHeight - 1) {
      skipped++;
      continue;
    }

    matrix[rowID - skipped].reserve(matrix_height);

    for (auto colId = 0u; colId < noOfColumns; colId++) {

      if (colId % tileWidth == 0 || colId % tileWidth == tileWidth - 1) continue;

      char c = input.at(rowID / tileHeight).at(colId / tileWidth)->atPos(colId % tileHeight, rowID % tileWidth);

      matrix[rowID - skipped].push_back(c);
    }
  }

  return matrix;
}

uint64_t problem2(const std::vector<std::vector<Tile *>> &input) {

  // Prepare data
  std::vector<std::vector<std::string>> all_possible_tiles = {retrieveTileWithoutBorders(input)};

  all_possible_tiles.push_back(Rotate(all_possible_tiles.back()));
  all_possible_tiles.push_back(Rotate(all_possible_tiles.back()));
  all_possible_tiles.push_back(Rotate(all_possible_tiles.back()));
  all_possible_tiles.push_back(MirrorRotate(all_possible_tiles.front()));
  all_possible_tiles.push_back(Rotate(all_possible_tiles.back()));
  all_possible_tiles.push_back(Rotate(all_possible_tiles.back()));
  all_possible_tiles.push_back(Rotate(all_possible_tiles.back()));

  std::vector<std::pair<int, int>> dragon{{0, 18}, {1, 0}, {1, 5}, {1, 6}, {1, 11}, {1, 12}, {1, 17}, {1, 18},
                                          {1, 19}, {2, 1}, {2, 4}, {2, 7}, {2, 10}, {2, 13}, {2, 16}};

  for (auto &matrixWithoutBorders : all_possible_tiles) {
    int amountOfDragons = 0;
    for (auto rowID = 1u; rowID < matrixWithoutBorders.size() - 1; rowID++) {

      for (auto columnID = 0u; columnID + 19 < matrixWithoutBorders.at(rowID).size(); columnID++) {

        bool found = true;
        for (auto &[r, c] : dragon) {
          if (matrixWithoutBorders.at(r + rowID - 1).at(c + columnID) != '#') {
            found = false;
            break;
          }
        }

        if (found) amountOfDragons++;
      }
    }

    if (amountOfDragons != 0) {

      uint32_t amountofCardinals = 0u;
      for (auto &row : matrixWithoutBorders) {
        amountofCardinals += std::count(row.begin(), row.end(), '#');
      }

      return amountofCardinals - (15 * amountOfDragons);
    }
  }

  return 0u;
}

int main() {

  auto input = getInput("2020/20/inputs/input.txt");

  auto matrix = problem1(input);
  auto prob_1 = matrix[0][0]->getID() * matrix[0][11]->getID() * matrix[11][0]->getID() * matrix[11][11]->getID();

  std::cout << "Problem #1: " << std::to_string(prob_1) << '\n';
  std::cout << "Problem #2: " << std::to_string(problem2(matrix)) << '\n';
}