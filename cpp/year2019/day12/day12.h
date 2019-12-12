#ifndef ADVENTOFCODE_YEAR2019_DAY12_H
#define ADVENTOFCODE_YEAR2019_DAY12_H

#include <istream>

#include "adventofcode.h"

namespace day12 {
  constexpr unsigned int STEPS = 1000;
  adventofcode::answer_t part1(std::istream& is, unsigned int steps);
  adventofcode::answer_t part2(std::istream& is);
}

#endif
