#include "year2019/day01.h"

#include <istream>

#include "adventofcode.h"

namespace day01 {
  adventofcode::answer_t part1(std::istream& is) {
    int sum = 0;
    for (int i; is >> i;) {
      sum += i / 3 - 2;
    }
    return adventofcode::ok(std::to_string(sum));
  }

  adventofcode::answer_t part2(std::istream& is) {
    return adventofcode::err("not implemented yet");
  }
}
