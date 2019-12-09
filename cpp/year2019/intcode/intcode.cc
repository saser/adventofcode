#include "year2019/intcode/intcode.h"

#include <string>
#include <utility>
#include <vector>

#include "absl/strings/str_split.h"

bool write_param(int opcode, int n) {
  bool b = false;
  switch (opcode) {
  case 1:
  case 2:
  case 7:
  case 8:
    b = n == 3;
    break;
  case 3:
    b = n == 1;
  }
  return b;
}

namespace intcode {
  memory mem(const execution& e) {
    return e.m;
  }

  void write(execution& e, const int64_t& i) {
    e.in.push_back(i);
  }

  void write_all(execution& e, const input& i) {
    e.in.insert(e.in.end(), i.begin(), i.end());
  }

  int64_t read(execution& e) {
    auto v = e.out.front();
    e.out.pop_front();
    return v;
  }

  output read_all(execution& e) {
    auto copy = e.out;
    e.out.clear();
    return copy;
  }

  void run_instruction(execution& e) {
    auto instruction = e.m[e.position];
    auto op = opcode(instruction);
    if (op == 99) {
      e.state = execution_state::halted;
      return;
    }
    auto n = n_params(op);
    std::vector<int64_t> params;
    params.reserve(n);
    for (int param = 1; param <= n; param++) {
      auto value = e.m[e.position + param];
      if (position_mode(instruction, param) && !write_param(op, param)) {
        value = e.m[value];
      }
      params.push_back(value);
    }
    int64_t operand1, operand2, destination, value;
    auto new_position = e.position + n + 1;
    switch (op) {
    // addition, multiplication
    case 1:
    case 2:
      operand1 = params[0];
      operand2 = params[1];
      destination = params[2];
      if (op == 1) {
        value = operand1 + operand2;
      } else {
        value = operand1 * operand2;
      }
      e.m[destination] = value;
      break;
    // read input
    case 3:
      if (e.in.empty()) {
        e.state = execution_state::waiting;
        return;
      }
      value = e.in.front();
      e.in.pop_front();
      destination = params[0];
      e.m[destination] = value;
      break;
    // produce output
    case 4:
      value = params[0];
      e.out.push_back(value);
      break;
    // jump-if-true, jump-if-false
    case 5:
    case 6:
      operand1 = params[0];
      value = params[1];
      if (op == 5 ? operand1 != 0 : operand1 == 0) {
        new_position = value;
      }
      break;
    // less than, equals
    case 7:
    case 8:
      operand1 = params[0];
      operand2 = params[1];
      destination = params[2];
      if (op == 7 ? operand1 < operand2 : operand1 == operand2) {
        e.m[destination] = 1;
      } else {
        e.m[destination] = 0;
      }
      break;
    }
    e.position = new_position;
    e.state = execution_state::running;
  }

  void run(execution& e) {
    auto state = e.state;
    do {
      run_instruction(e);
      state = e.state;
    } while (state == execution_state::running);
  }

  int opcode(int64_t instruction) {
    return instruction % 100;
  }

  bool immediate_mode(int64_t instruction, int n) {
    int mask = 10;
    for (int i = 1; i <= n; i++) {
      mask *= 10;
    }
    return (instruction % (mask * 10)) / mask == 1;
  }

  bool position_mode(int64_t instruction, int n) {
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

  int n_params(int opcode) {
    int n;
    switch (opcode) {
    // addition, multiplication, less than, equals
    case 1:
    case 2:
    case 7:
    case 8:
      n = 3;
      break;
    // read input, produce output
    case 3:
    case 4:
      n = 1;
      break;
    // jump-if-true, jump-if-false
    case 5:
    case 6:
      n = 2;
      break;
    default:
      n = 0;
      break;
    }
    return n;
  }

  std::pair<memory, output> run(const memory& initial, const input& input) {
    execution e {initial};
    write_all(e, input);
    run(e);
    return {mem(e), read_all(e)};
  }
}
