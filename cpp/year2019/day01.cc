#include "year2019/day01.h"

#include <istream>

#include "adventofcode.h"

adventofcode::answer_t solve(std::istream& is, int part);

namespace day01 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  int sum = 0;
  for (int i; is >> i;) {
    while (i > 0) {
      int new_i = std::max(0, i / 3 - 2);
      sum += new_i;
      if (part == 1) {
        break;
      }
      i = new_i;
    }
  }
  return adventofcode::ok(std::to_string(sum));
}
