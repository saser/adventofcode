#include "cpp/year2019/day04/day04.h"

#include <istream>
#include <string>

#include "absl/strings/str_split.h"

#include "cpp/adventofcode.h"
#include "cpp/year2019/day04/internal.h"

adventofcode::answer_t solve(std::istream& is, bool strict);

namespace day04 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, false);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, true);
  }
}

adventofcode::answer_t solve(std::istream& is, bool strict) {
  std::string input;
  std::getline(is, input);
  std::vector<std::string> parts = absl::StrSplit(input, "-");
  int start = std::stoi(parts[0]);
  int end = std::stoi(parts[1]);
  int count = 0;
  for (int password = start; password <= end; password++) {
    auto digits = day04::digits(password);
    if (day04::has_double(digits, strict) && day04::non_decreasing(digits)) {
      count++;
    }
  }
  return adventofcode::ok(std::to_string(count));
}
