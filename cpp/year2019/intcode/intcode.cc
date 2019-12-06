#include "year2019/intcode/intcode.h"

namespace intcode {
  int opcode(int instruction) {
    return instruction % 100;
  }
}
