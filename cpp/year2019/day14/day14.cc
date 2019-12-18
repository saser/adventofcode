#include "year2019/day14/day14.h"

#include <iostream>
#include <regex>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

#include "adventofcode.h"

struct reagent {
  unsigned int amount;
  std::string chemical;
};

using productions_t = std::unordered_map<std::string, std::pair<unsigned int, std::vector<reagent>>>;

adventofcode::answer_t solve(std::istream& is, int part);
productions_t parse(std::istream& is);

namespace day14 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  auto productions = parse(is);
  for (auto [chemical, pair] : productions) {
    auto [amount, reagents] = pair;
    std::cout << "To produce " << amount << " of " << chemical << ": ";
    for (auto reagent : reagents) {
      std::cout << reagent.amount << " " << reagent.chemical << ", ";
    }
    std::cout << std::endl;
  }
  return adventofcode::err("not implemented yet");
}

productions_t parse(std::istream& is) {
  std::string line;
  const std::regex re(R"((\d+) (\w+))");
  productions_t productions;
  productions["ORE"] = {1, {}};
  while (std::getline(is, line)) {
    std::vector<reagent> requirements;
    auto matches_it = std::sregex_iterator(line.begin(), line.end(), re);
    auto matches_end = std::sregex_iterator();
    while (matches_it != matches_end) {
      auto match = *matches_it;
      auto amount = (unsigned int) std::stoul(match[1]);
      auto chemical = match[2];
      requirements.push_back(reagent {amount, chemical});
      matches_it++;
    }
    auto result = requirements.back();
    requirements.pop_back();
    productions[result.chemical] = {result.amount, requirements};
  }
  return productions;
}
