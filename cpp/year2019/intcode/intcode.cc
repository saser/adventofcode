#include "year2019/intcode/intcode.h"

#include <utility>

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
  std::pair<memory, output> run(const memory& initial, const input& input) {
    intcode::memory memory = initial;
    size_t position = 0;
    int instruction = 0;
    auto input_it = input.begin();
    intcode::output output;
    while ((instruction = memory[position])) {
      int op = opcode(instruction);
      if (op == 99) {
        break;
      }
      int operand1, operand2, destination, value;
      switch (op) {
      // addition, multiplication
      case 1:
      case 2:
        operand1 = memory[position + 1];
        if (position_mode(instruction, 1)) {
          operand1 = memory[operand1];
        }
        operand2 = memory[position + 2];
        if (position_mode(instruction, 2)) {
          operand2 = memory[operand2];
        }
        destination = memory[position + 3];
        if (op == 1) {
          value = operand1 + operand2;
        } else {
          value = operand1 * operand2;
        }
        memory[destination] = value;
        break;
      case 3:
        value = *input_it;
        input_it++;
        destination = memory[position + 1];
        memory[destination] = value;
        break;
      case 4:
        value = memory[position + 1];
        if (position_mode(instruction, 1)) {
          value = memory[value];
        }
        output.push_back(value);
        break;
      }
      position += n_params(op) + 1;
    }
    return {memory, output};
  }
}
