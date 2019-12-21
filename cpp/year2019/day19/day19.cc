#include "year2019/day19/day19.h"

#include <istream>
#include <string>

#include "adventofcode.h"
#include "year2019/intcode/intcode.h"

struct point_t {
  int64_t x;
  int64_t y;
};

struct tractor_t {
  intcode::memory program;

  int64_t test(point_t p);
};

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
  tractor_t tractor {program};
  if (part == 1) {
    auto sum = 0;
    for (int64_t x = 0; x < 50; x++) {
      for (int64_t y = 0; y < 50; y++) {
        sum += tractor.test(point_t {x, y});
      }
    }
    return adventofcode::ok(std::to_string(sum));
  }
  return adventofcode::err("not implemented yet");
}

int64_t tractor_t::test(point_t p) {
  auto [_, output] = intcode::run(program, {p.x, p.y});
  return output[0];
}
