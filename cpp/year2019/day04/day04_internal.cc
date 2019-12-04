#include "year2019/day04/day04_internal.h"

#include <algorithm>
#include <vector>

namespace day04 {
  std::vector<int> digits(int password) {
    std::vector<int> d;
    while (password != 0) {
      d.push_back(password % 10);
      password /= 10;
    }
    std::reverse(d.begin(), d.end());
    return d;
  }

  bool has_double(const std::vector<int>& digits, bool strict) {
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
