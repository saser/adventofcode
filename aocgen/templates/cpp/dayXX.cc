#include "{{.FullYear}}/{{.FullDay}}/{{.FullDay}}.h"

#include <istream>

#include "adventofcode.h"

adventofcode::answer_t solve(std::istream& is, int part);

namespace {{.FullDay}} {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  return adventofcode::err("not implemented yet");
}
