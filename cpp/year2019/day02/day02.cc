#include "year2019/day02/day02.h"

#include <istream>
#include <string>

#include "adventofcode.h"
#include "year2019/intcode/intcode.h"

adventofcode::answer_t solve(std::istream& is, int part);

namespace day02 {
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
  if (part == 1) {
    program[1] = 12;
    program[2] = 2;
    auto result = intcode::run(program, {});
    return adventofcode::ok(std::to_string(result.first[0]));
  }
  for (int noun = 0; noun <= 99; noun++) {
    for (int verb = 0; verb <= 99; verb++) {
      program[1] = noun;
      program[2] = verb;
      auto result = intcode::run(program, {});
      if (result.first[0] == 19690720) {
        return adventofcode::ok(std::to_string(100 * noun + verb));
      }
    }
  }
  return adventofcode::err("no solution found");
}
