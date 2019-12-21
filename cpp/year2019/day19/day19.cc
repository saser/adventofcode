#include "year2019/day19/day19.h"

#include <istream>
#include <string>

#include "adventofcode.h"
#include "year2019/intcode/intcode.h"

adventofcode::answer_t solve(std::istream& is, int part);

namespace day19 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  std::string line;
  std::getline(is, line);
  intcode::memory program = intcode::parse(line);
  intcode::input inputs;
  auto sum = 0;
  for (int64_t x = 0; x < 50; x++) {
    for (int64_t y = 0; y < 50; y++) {
      auto [_, output] = intcode::run(program, {x, y});
      sum += output[0];
    }
  }
  return adventofcode::ok(std::to_string(sum));
}
