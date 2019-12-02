#include "year2019/day02/day02.h"

#include <istream>
#include <string>
#include <vector>

#include "absl/strings/str_split.h"

#include "adventofcode.h"
#include "year2019/day02/day02_internal.h"

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
  std::vector<std::string> parts = absl::StrSplit(line, ",");
  std::vector<int> program;
  for (auto part : parts) {
    program.push_back(std::stoi(part));
  }
  if (part == 1) {
    program[1] = 12;
    program[2] = 2;
    return adventofcode::ok(std::to_string(day02::run(program)));
  }
  return adventofcode::err("not implemented yet");
}
