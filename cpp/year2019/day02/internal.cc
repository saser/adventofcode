#include "year2019/day02/internal.h"

#include <vector>

namespace day02 {
  int run(const std::vector<int>& program) {
    std::vector<int> memory = program;
    int position = 0;
    int opcode = 0;
    while ((opcode = memory[position]) != 99) {
      int operand1 = memory[memory[position + 1]];
      int operand2 = memory[memory[position + 2]];
      int destination = memory[position + 3];
      int value = 0;
      switch (opcode) {
      case 1:
        value = operand1 + operand2;
        break;
      case 2:
        value = operand1 * operand2;
        break;
      }
      memory[destination] = value;
      position += 4;
    }
    return memory[0];
  }
}
