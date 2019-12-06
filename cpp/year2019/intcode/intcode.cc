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
}
