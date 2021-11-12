
// CPP Includes
#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

struct Allergen {
  std::string name;
  std::vector<std::string> foodPossibilities;
  bool assigned{false};

  Allergen(std::string _name) : name(std::move(_name)) {}
};

struct InputData {
  std::vector<Allergen> allergens;
  std::unordered_map<std::string, uint32_t> uset_ingredients;
  uint64_t individual_ingredients{0};
};

std::vector<std::string> split(std::string_view s, std::string_view delimeter = " ") {
  size_t pos_start = 0, pos_end, delim_len = delimeter.size();
  std::vector<std::string> res;

  while ((pos_end = s.find(delimeter, pos_start)) != std::string::npos) {
    res.emplace_back(s.substr(pos_start, pos_end - pos_start));
    pos_start = pos_end + delim_len;
  }

  res.emplace_back(s.substr(pos_start));
  return res;
}

InputData getInput(std::string_view file) {
  std::ifstream input(file.data());
  std::vector<uint64_t> content;

  if (!input.is_open()) throw std::runtime_error("File opening failed.");

  InputData data;
  std::map<std::string, Allergen> umap_allergens;

  for (std::string value; std::getline(input, value);) {
    auto parenthisesPos = value.find_first_of('(');

    auto ingredients = split(value.substr(0, parenthisesPos - 1));
    auto allergens   = split(value.substr(parenthisesPos + 10, value.size() - (parenthisesPos + 10) - 1), ", ");

    data.individual_ingredients += ingredients.size();

    for (const auto &ingr : ingredients) {

      if (!data.uset_ingredients.contains(ingr)) {
        data.uset_ingredients.try_emplace(ingr, 1u);
        continue;
      }
      data.uset_ingredients.at(ingr)++;
    }

    for (const auto &allergen : allergens) {

      std::sort(ingredients.begin(), ingredients.end());

      if (!umap_allergens.contains(allergen)) {
        umap_allergens.try_emplace(allergen, allergen);
        umap_allergens.at(allergen).foodPossibilities = ingredients;
        continue;
      }

      std::vector<std::string> intersected;
      std::ranges::set_intersection(ingredients, umap_allergens.at(allergen).foodPossibilities, std::back_inserter(intersected));
      umap_allergens.at(allergen).foodPossibilities = std::move(intersected);
    }
  }

  std::ranges::for_each(umap_allergens, [&data](auto &&it) { data.allergens.push_back(it.second); });

  std::ranges::sort(data.allergens, [](const Allergen &lhs, const Allergen &rhs) {
    return lhs.foodPossibilities.size() < rhs.foodPossibilities.size();
  });

  std::vector<std::string> foundFood;
  auto add_food = [&foundFood](std::string food) {
    foundFood.push_back(std::move(food));
    std::ranges::sort(foundFood);
  };

  while (true) {

    if (foundFood.size() == data.allergens.size()) break;

    for (auto &allergenObj : data.allergens) {

      if (allergenObj.assigned) continue;

      if (allergenObj.foodPossibilities.size() == 1) {
        add_food(allergenObj.foodPossibilities.front());
        allergenObj.assigned = true;
        continue;
      }

      std::vector<std::string> attempt_to_match;
      std::ranges::set_difference(allergenObj.foodPossibilities, foundFood, std::back_inserter(attempt_to_match));

      if (attempt_to_match.size() == 1) {
        add_food(attempt_to_match.front());
        allergenObj.foodPossibilities = std::move(attempt_to_match);
        allergenObj.assigned          = true;
        continue;
      }
    }
  }

  std::ranges::sort(data.allergens, [](const Allergen &lhs, const Allergen &rhs) { return lhs.name < rhs.name; });

  return data;
}

uint64_t problem1(const InputData &input) {

  auto ingredientsToRemove = 0u;

  for (const auto &allergen : input.allergens) {
    ingredientsToRemove += input.uset_ingredients.at(allergen.foodPossibilities.front());
  }

  return input.individual_ingredients - ingredientsToRemove;
}

std::string problem2(const InputData &input) {

  std::stringstream result;

  for (auto &allergen : input.allergens) {
    result << allergen.foodPossibilities.front() << ",";
  }

  std::string r = result.str();

  return r.substr(0, r.size() - 1);
}

int main() {

  auto input = getInput("2020/21/inputs/input.txt");

  std::cout << "Problem #1: : " << std::to_string(problem1(input)) << '\n';
  std::cout << "Problem #2: : " << problem2(input) << '\n';
}