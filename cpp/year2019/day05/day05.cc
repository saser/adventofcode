#include "year2019/day05/day05.h"

#include <istream>
#include <string>

#include "absl/strings/str_format.h"

#include "adventofcode.h"
#include "year2019/intcode/intcode.h"

namespace day05 {
  adventofcode::answer_t part1(std::istream& is) {
    std::string line;
    std::getline(is, line);
    intcode::memory program = intcode::parse(line);
    auto result = intcode::run(program, {1});
    auto output = result.second;
    for (size_t i = 0; i < output.size() - 1; i++) {
      if (output[i] != 0) {
        return adventofcode::err(absl::StrFormat("failure in test %d: output = %d", i, output[i]));
      }
    }
    int diagnostic_code = output[output.size() - 1];
    return adventofcode::ok(std::to_string(diagnostic_code));
  }
}
