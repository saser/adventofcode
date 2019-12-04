#ifndef ADVENTOFCODE_YEAR2019_DAY04_INTERNAL_H
#define ADVENTOFCODE_YEAR2019_DAY04_INTERNAL_H

#include <vector>

namespace day04 {
  std::vector<int> digits(int password);
  bool has_double(const std::vector<int>& digits);
  bool non_decreasing(const std::vector<int>& digits);
}

#endif
