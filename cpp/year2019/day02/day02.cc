#include "year2019/day02/day02.h"

#include <istream>
#include <string>
#include <vector>

#include "absl/strings/str_split.h"

#include "adventofcode.h"

namespace day02 {
  adventofcode::answer_t part1(std::istream& is) {
    std::string line;
    std::getline(is, line);
    std::vector<std::string> parts = absl::StrSplit(line, ",");
    std::vector<int> program;
    for (auto part : parts) {
      program.push_back(std::stoi(part));
    }
    return adventofcode::err("not implemented yet");
  }
}
