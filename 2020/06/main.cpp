
// Cpp Includes
#include <bitset>
#include <fstream>
#include <iostream>
#include <numeric>
#include <vector>

class Group {
protected:
  std::bitset<26> questions;

public:
  Group() { questions = 0; }

  virtual void addPersonAnswers(std::string_view answ) = 0;
  uint getUniqueAnswers() const { return questions.count(); }
};

class Problem1 : public Group {

public:
  Problem1() : Group(){};

  void addPersonAnswers(std::string_view answ) final {
    for (auto answerID : answ) {
      questions.set(static_cast<uint>(answerID) - 97);
    }
  }
};

class Problem2 : public Group {

  bool firstTime = true;

public:
  Problem2() : Group(){};

  void addPersonAnswers(std::string_view answ) final {

    decltype(questions) temp = 0;
    for (auto answerID : answ) {
      temp.set(static_cast<uint>(answerID) - 97);
    }

    if (firstTime) {
      questions = temp;
      firstTime = false;
    } else
      questions &= temp;
  }
};

std::vector<std::string> getInput(std::string_view file) {

  std::ifstream input(file.data());
  std::vector<std::string> people;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  for (std::string line; std::getline(input, line);) {
    people.push_back(line);
  }

  return people;
}

int problem1(const std::vector<std::string> &input) {

  std::vector<Problem1> groups(1);

  for (std::string_view line : input) {
    if (line.empty()) {
      groups.emplace_back();
      continue;
    }

    groups.back().addPersonAnswers(line);
  }

  return std::accumulate(groups.begin(), groups.end(), 0u, [](unsigned int acc, const Problem1 &group) {
    return acc + group.getUniqueAnswers();
  });
}

int problem2(const std::vector<std::string> &input) {

  std::vector<Problem2> groups(1);

  for (std::string_view line : input) {
    if (line.empty()) {
      groups.emplace_back();
      continue;
    }
    groups.back().addPersonAnswers(line);
  }

  return std::accumulate(groups.begin(), groups.end(), 0u, [](unsigned int acc, const Problem2 &group) {
    return acc + group.getUniqueAnswers();
  });
}

int main() {
  auto input = getInput("2020/06/inputs/input.txt");
  std::cout << "Problem #1: : " << std::to_string(problem1(input)) << '\n';
  std::cout << "Problem #2: : " << std::to_string(problem2(input)) << '\n';
}
