#include "year2019/intcode/intcode.h"

#include <string>
#include <utility>
#include <vector>

#include "absl/strings/str_split.h"

namespace intcode {
  int opcode(int instruction) {
    return instruction % 100;
  }
  bool write_param(int opcode, int n) {
    bool b = false;
    switch (opcode) {
    case 1:
    case 2:
      b = n == 3;
      break;
    case 3:
      b = n == 1;
    }
    return b;
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
  memory parse(const std::string& s) {
    std::vector<std::string> parts = absl::StrSplit(s, ",");
    memory m;
    m.reserve(parts.size());
    for (auto part : parts) {
      m.push_back(std::stoi(part));
    }
    return m;
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
      int n = n_params(op);
      std::vector<int> params;
      params.reserve(n);
      for (int param = 1; param <= n; param++) {
        int value = memory[position + param];
        if (position_mode(instruction, param) && !write_param(op, param)) {
          value = memory[value];
        }
        params.push_back(value);
      }
      int operand1, operand2, destination, value;
      std::string operation;
      switch (op) {
      // addition, multiplication
      case 1:
      case 2:
        operand1 = params[0];
        operand2 = params[1];
        destination = params[2];
        if (op == 1) {
          value = operand1 + operand2;
          operation = "+";
        } else {
          value = operand1 * operand2;
          operation = "*";
        }
        memory[destination] = value;
        break;
      case 3:
        value = *input_it;
        input_it++;
        destination = params[0];
        memory[destination] = value;
        break;
      case 4:
        value = params[0];
        output.push_back(value);
        break;
      }
      position += n_params(op) + 1;
    }
    return {memory, output};
  }
}
