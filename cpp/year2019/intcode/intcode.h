#ifndef ADVENTOFCODE_YEAR2019_INTCODE_INTCODE_H
#define ADVENTOFCODE_YEAR2019_INTCODE_INTCODE_H

#include <string>
#include <utility>
#include <vector>

namespace intcode {
  static const std::vector<int> OPCODES {
    1, // addition
    2, // multiplication
    3, // read input
    4, // produce output
    5, // jump-if-true
    6, // jump-if-false
    7, // less than
    8, // equals
    99, // halt
  };
  // Determine the opcode for a given memory value.
  int opcode(int instruction);
  // Determine whether parameter number `n` is in immediate mode.
  bool immediate_mode(int instruction, int n);
  bool position_mode(int instruction, int n);
  // Return the number of parameters for the given opcode.
  size_t n_params(int opcode);
  // Memory is represented as a vector of ints.
  typedef std::vector<int> memory;
  // As are input and output of the program.
  typedef std::vector<int> input;
  typedef std::vector<int> output;
  // Parse a string into a memory.
  memory parse(const std::string& s);
  // Run a program. The program is started at position 0, and the provided
  // inputs are read in order. The first return value is the state of the memory
  // at the time the program halts; the second return value is the output
  // produced by the program.
  std::pair<memory, output> run(const memory& initial, const input& inputs);
}

#endif
