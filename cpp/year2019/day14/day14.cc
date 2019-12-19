#include "year2019/day14/day14.h"

#include <istream>
#include <regex>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

#include "adventofcode.h"

struct reagent {
  unsigned long amount;
  std::string chemical;
};

using productions_t = std::unordered_map<std::string, std::pair<unsigned long, std::vector<reagent>>>;
using produce_t = std::unordered_map<std::string, unsigned long>;

adventofcode::answer_t solve(std::istream& is, int part);
productions_t parse(std::istream& is);
std::pair<produce_t, produce_t> produce(const reagent& r, const productions_t& productions);

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
  reagent fuel {1, "FUEL"};
  auto [produced, _] = produce(fuel, productions);
  if (part == 1) {
    return adventofcode::ok(std::to_string(produced["ORE"]));
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
      auto amount = std::stoul(match[1]);
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

void produce_aux(const reagent& r, const productions_t& productions, produce_t& produced_chemicals, produce_t& available_chemicals) {
  auto to_produce = r.amount;
  auto& available = available_chemicals[r.chemical];
  if (available >= to_produce) {
    return;
  }
  to_produce -= available;
  auto [result_amount, requirements] = productions.at(r.chemical);
  auto times = to_produce / result_amount;
  if (to_produce % result_amount != 0) {
    times++;
  }
  for (auto requirement : requirements) {
    requirement.amount *= times;
    produce_aux(requirement, productions, produced_chemicals, available_chemicals);
    available_chemicals[requirement.chemical] -= requirement.amount;
  }
  auto total = result_amount * times;
  produced_chemicals[r.chemical] += total;
  available += total;
}

std::pair<produce_t, produce_t> produce(const reagent& r, const productions_t& productions) {
  produce_t produced_chemicals;
  produce_t available_chemicals;
  produce_aux(r, productions, produced_chemicals, available_chemicals);
  return {produced_chemicals, available_chemicals};
}
