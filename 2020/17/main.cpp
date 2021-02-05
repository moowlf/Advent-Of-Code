
// CPP Includes
#include <algorithm>
#include <cstdint>
#include <execution>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

class AdventUnorderedMap {

  struct Node {
    std::vector<int32_t> coordinates{};
    bool currentState;
    bool nextState;

    Node(std::vector<int32_t> coord, bool status = false)
        : coordinates(std::move(coord)), currentState(status), nextState(status){};

    void update() { currentState = nextState; }

    friend std::ostream &operator<<(std::ostream &os, const Node &node) {
      os << "Node [";
      for (auto i = 0u; i < node.coordinates.size() - 1; i++) os << node.coordinates[i] << ", ";
      os << node.coordinates.back() << "] : " << std::boolalpha << node.currentState;
      return os;
    }

    friend inline bool operator<(const Node &lhs, const std::vector<int32_t> &rhs) { return lhs.coordinates < rhs; }

    friend inline bool operator<(const Node &lhs, const Node &rhs) { return lhs.coordinates < rhs.coordinates; }

    friend inline bool operator==(const Node &lhs, const Node &rhs) { return lhs.coordinates == rhs.coordinates; }
  };

  struct NearbyPoints {
    uint32_t active{0};
    uint32_t inactive{0};
    std::vector<std::vector<int32_t>> points_to_add;
  };

  std::vector<Node> points;

public:
  bool isPointActive(const std::vector<int> &args) { return getPoint(args)->currentState; }

  Node *createPoint(const std::vector<int> &coordinates, bool status) {
    auto node = points.emplace_back(coordinates, status);
    std::sort(points.begin(), points.end());
    return &(*std::lower_bound(points.begin(), points.end(), coordinates));
  }

  Node *getPoint(const std::vector<int> &coordinates) {

    auto loc = std::lower_bound(points.begin(), points.end(), coordinates);

    if (loc != points.end() && loc->coordinates == coordinates) return &(*loc);

    return nullptr;
  }

  bool hasPoint(
      std::vector<Node>::const_iterator begin, std::vector<Node>::const_iterator end,
      const std::vector<int> &coordinates) const {
    bool tem = std::binary_search(begin, end, coordinates);
    return tem;
  }

  uint32_t run() {

    const auto pointsSize = points.size();
    for (auto i = 0u; i < pointsSize; i++) {
      auto nearestPoints = getNearbyStatus(points.at(i));
      std::copy_if(
          nearestPoints.points_to_add.begin(), nearestPoints.points_to_add.end(), std::back_inserter(points),
          [this, &pointsSize](auto coordinates) {
            return !hasPoint(points.cbegin(), points.cbegin() + pointsSize, coordinates);
          });
    }

    std::sort(points.begin(), points.end());
    points.erase(std::unique(points.begin(), points.end()), points.end());

    std::for_each(points.begin(), points.end(), [this](Node &node) {
      auto nearestPoints = getNearbyStatus(node);
      if (node.currentState) {
        node.nextState = nearestPoints.active == 2 || nearestPoints.active == 3;
      } else {
        node.nextState = nearestPoints.active == 3;
      }
    });

    uint32_t active = 0u;
    std::for_each(points.begin(), points.end(), [&active](Node &node) {
      node.update();
      active += node.currentState;
    });

    return active;
  }

private:
  NearbyPoints getNearbyStatus(const Node &node) {

    std::vector<std::vector<int>> nearestPoints = {node.coordinates};

    for (auto i = 0u; i < node.coordinates.size(); i++) {

      auto currentSize = nearestPoints.size();

      for (auto nodeIdx = 0u; nodeIdx < currentSize; nodeIdx++) {

        nearestPoints.push_back(nearestPoints[nodeIdx]);
        nearestPoints.back()[i] -= 1;

        nearestPoints.push_back(nearestPoints[nodeIdx]);
        nearestPoints.back()[i] += 1;
      }
    }

    NearbyPoints nearbyPoints;

    for (auto &point : nearestPoints) {

      if (point == node.coordinates) continue;

      auto pointObj = getPoint(point);

      if (pointObj == nullptr) {
        nearbyPoints.points_to_add.push_back(point);
        nearbyPoints.inactive += 1;
        continue;
      }

      if (pointObj->currentState)
        nearbyPoints.active += 1;
      else
        nearbyPoints.inactive += 1;
    }

    return nearbyPoints;
  }
};

AdventUnorderedMap getInput(std::string_view file, size_t size) {
  std::ifstream input(file.data());
  AdventUnorderedMap map;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  std::vector<int> coords(size, 0);
  uint32_t row = 0;
  for (std::string value; std::getline(input, value);) {
    coords[0] = row;
    for (uint32_t column = 0u; column < value.size(); column++) {
      coords[1] = column;
      map.createPoint(coords, value.at(column) == '#');
    }
    row++;
  }

  return map;
}

uint64_t problem1(AdventUnorderedMap &input) {

  auto value = 0;
  for (auto i = 0u; i < 6; i++) {
    value = input.run();
  }
  return value;
}

uint64_t problem2(AdventUnorderedMap &input) {
  auto value = 0;
  for (auto i = 0u; i < 6; i++) {
    value = input.run();
  }
  return value;
}

int main() {

  auto input = getInput("2020/17/inputs/input.txt", 3);
  std::cout << "Problem #1: : " << std::to_string(problem1(input)) << '\n';

  input = getInput("2020/17/inputs/input.txt", 4);
  std::cout << "Problem #2: : " << std::to_string(problem2(input)) << '\n';
  return 0;
}