#ifndef ADVENTOFCODE_YEAR{{.Year}}_DAY{{.PaddedDay}}_H
#define ADVENTOFCODE_YEAR{{.Year}}_DAY{{.PaddedDay}}_H

#include <istream>

#include "cpp/adventofcode.h"

namespace {{.FullDay}} {
  adventofcode::answer_t part1(std::istream& is);
  adventofcode::answer_t part2(std::istream& is);
}

#endif
