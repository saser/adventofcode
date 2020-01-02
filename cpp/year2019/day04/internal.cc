#include "cpp/year2019/day04/internal.h"

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
    int current = digits[0];
    int count = 0;
    for (auto it = digits.begin(); it != digits.end(); it++) {
      int digit = *it;
      if (digit == current) {
        count++;
      } else {
        if (strict ? count == 2 : count >= 2) {
          return true;
        }
        current = digit;
        count = 1;
      }
    }
    return strict ? count == 2 : count >= 2;
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
