#ifndef ADVENTOFCODE_YEAR2019_INTCODE_INTCODE_H
#define ADVENTOFCODE_YEAR2019_INTCODE_INTCODE_H

#include <vector>

namespace intcode {
  static const std::vector<int> OPCODES {
    1, // addition
    2, // multiplication
    3, // read input
    4, // produce output
    99, // halt
  };
  // Determine the opcode for a given memory value.
  int opcode(int instruction);
}

#endif
