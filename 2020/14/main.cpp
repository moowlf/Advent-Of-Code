
// Cpp Includes
#include <bitset>
#include <cmath>
#include <fstream>
#include <iostream>
#include <numeric>
#include <queue>
#include <regex>
#include <string>
#include <unordered_map>
#include <variant>
#include <vector>

class DecoderShip {

protected:
  struct Mask {
    struct BitValue {
      int index;
      bool set;

      explicit BitValue(int index, bool v) : index(index), set(v){};
    };
    std::vector<BitValue> bitValues;
    std::string mask;
  };

  struct Operation {
    enum operation_t { UpdateMask, OverrideMemory } op;
    std::variant<Mask, std::pair<uint64_t, uint64_t>> value;
  };

private:
  std::unordered_map<uint64_t, uint64_t> addresses;
  std::vector<Operation> operations;
  Mask mask;

public:
  DecoderShip(std::string_view filepath) {

    std::ifstream input(filepath.data());
    if (!input.is_open()) throw std::runtime_error("File opening failed.");

    std::regex updateMemory(R"(mem\[(\d+)\]\s=\s(\d+))");
    std::regex updateMask(R"(mask\s=\s([X|\d]{36}))");

    std::smatch match;
    for (std::string line; std::getline(input, line);) {

      Operation op;
      if (std::regex_match(line, match, updateMask)) {
        op.op = Operation::operation_t::UpdateMask;

        Mask mask;

        mask.mask = match[1].str();
        for (int i = mask.mask.size() - 1; i >= 0; i--) {
          if (mask.mask[i] != 'X') mask.bitValues.emplace_back(mask.mask.size() - i - 1, mask.mask[i] == '1');
        }

        op.value = std::move(mask);

      } else if (std::regex_match(line, match, updateMemory)) {
        op.op = Operation::operation_t::OverrideMemory;

        std::pair<uint64_t, uint64_t> pair;
        std::regex_search(line, match, updateMemory);

        pair.first  = std::stoull(match[1].str());
        pair.second = std::stoull(match[2].str());
        op.value    = std::move(pair);

      } else {
        throw std::runtime_error("Invalid operation found.");
      }
      operations.emplace_back(std::move(op));
    }
  }

  uint64_t runOperations() {

    for (auto &operation : operations) {
      if (operation.op == Operation::operation_t::UpdateMask)
        runUpdateMask(operation);
      else
        runMemoryUpdate(operation);
    }

    double total = 0;
    for (auto i = addresses.begin(); i != addresses.end(); ++i) {
      total += i->second;
    }
    return total;
  };

private:
  void runUpdateMask(const Operation &operation) { mask = std::get<Mask>(operation.value); }

  virtual void runMemoryUpdate(const Operation &operation) {

    auto memoryData = std::get<std::pair<uint64_t, uint64_t>>(operation.value);

    std::bitset<36> currentValue = memoryData.second;
    for (auto &forcedBit : mask.bitValues) {
      currentValue[forcedBit.index] = forcedBit.set;
    }

    for (auto index : getIndexs(operation)) setMemory(index, currentValue.to_ullong());
  }

  virtual std::vector<uint64_t> getIndexs(const Operation &operation) {
    auto memoryData = std::get<std::pair<uint64_t, uint64_t>>(operation.value);
    return std::vector<uint64_t>{memoryData.first};
  }

protected:
  const Mask &getMask() const { return mask; }

  void setMemory(uint64_t index, uint64_t value) { addresses[index] = value; }
};

class DecoderShipV2 : public DecoderShip {

public:
  using DecoderShip::DecoderShip;

private:
  void runMemoryUpdate(const Operation &operation) override {
    auto memoryData = std::get<std::pair<uint64_t, uint64_t>>(operation.value).second;

    auto vec = getIndexs(operation);
    for (auto index : vec) setMemory(index, memoryData);
  }

  std::vector<uint64_t> getIndexs(const Operation &operation) override {

    std::string_view m          = getMask().mask;
    std::bitset<36> baseAddress = std::get<std::pair<uint64_t, uint64_t>>(operation.value).first;

    for (auto i = 0u; i < m.size(); i++) {
      if (m[i] == '0') continue;
      if (m[i] == '1') baseAddress[m.size() - i - 1] = 1u;
    }

    std::queue<uint64_t> queue;
    queue.emplace(baseAddress.to_ullong());

    for (auto i = 0u, xFound = 0u; i < 36u; i++) {
      if (m[i] != 'X') continue;

      auto elementsToRemove = std::pow(2, xFound++);

      while (elementsToRemove-- > 0) {

        uint64_t value = queue.front();
        uint64_t j     = static_cast<uint64_t>(1) << (m.size() - i - 1u);
        queue.push(value | j);
        queue.push(value & ~(j));
        queue.pop();
      }
    }

    std::vector<uint64_t> indexs;

    while (!queue.empty()) {
      indexs.push_back(queue.front());
      queue.pop();
    }

    return indexs;
  }
};

uint64_t problem1(DecoderShip &memory) { return memory.runOperations(); }

uint64_t problem2(DecoderShipV2 &memory) { return memory.runOperations(); }

int main() {

  auto decoderV1 = DecoderShip("2020/14/inputs/input.txt");
  auto decoderV2 = DecoderShipV2("2020/14/inputs/input.txt");

  std::cout << "Problem #1: : " << std::to_string(problem1(decoderV1)) << '\n';
  std::cout << "Problem #2: : " << std::to_string(problem2(decoderV2)) << '\n';
}