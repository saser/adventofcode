#include "year2019/day04/day04_internal.h"

#include <vector>

namespace day04 {
  bool has_double(const std::vector<int>& digits) {
    for (size_t i = 0; i < digits.size() - 1; i++) {
      if (digits[i] == digits[i + 1]) {
        return true;
      }
    }
    return false;
  }

  bool non_decreasing(const std::vector<int>& digits) {
    int current = digits[0];
    for (auto iterator = digits.begin(); iterator != digits.end(); ++iterator) {
      int i = *iterator;
      if (i < current) {
        return false;
      }
      current = i;
    }
    return true;
  }
}
