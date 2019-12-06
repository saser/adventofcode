#include "year2019/intcode/intcode.h"

namespace intcode {
  int opcode(int instruction) {
    return instruction % 100;
  }
  bool immediate_mode(int instruction, int n) {
    int mask = 10;
    for (int i = 1; i <= n; i++) {
      mask *= 10;
    }
    return (instruction % (mask * 10)) / mask == 1;
  }
  bool position_mode(int instruction, int n) {
    return !immediate_mode(instruction, n);
  }
  size_t n_params(int opcode) {
    size_t n;
    switch (opcode) {
    // addition, multiplication
    case 1:
    case 2:
      n = 3;
      break;
    // read input, produce output
    case 3:
    case 4:
      n = 1;
      break;
    default:
      n = 0;
      break;
    }
    return n;
  }
}
