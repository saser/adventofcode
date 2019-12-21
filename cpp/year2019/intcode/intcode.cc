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

int64_t src_param(intcode::execution& e, int64_t instruction, int n) {
  auto value = e.at(e.position + n);
  auto m = intcode::mode(instruction, n);
  switch (m) {
  case intcode::parameter_mode::position:
    value = e.at(value);
    break;
  case intcode::parameter_mode::relative:
    value = e.at(e.relative_base + value);
  default:
    break;
  }
  return value;
}

size_t dst_param(intcode::execution& e, int64_t instruction, int n) {
  auto dst = e.at(e.position + n);
  auto m = intcode::mode(instruction, n);
  switch (m) {
  case intcode::parameter_mode::relative:
    dst = e.relative_base + dst;
    break;
  default:
    break;
  }
  return dst;
}

std::vector<int64_t> parse_params(intcode::execution& e, int64_t instruction) {
  std::vector<int64_t> params;
  switch (intcode::opcode(instruction)) {
  case 1:
  case 2:
  case 7:
  case 8:
    params.push_back(src_param(e, instruction, 1));
    params.push_back(src_param(e, instruction, 2));
    params.push_back(dst_param(e, instruction, 3));
    break;
  case 3:
    params.push_back(dst_param(e, instruction, 1));
    break;
  case 4:
    params.push_back(src_param(e, instruction, 1));
    break;
  case 5:
  case 6:
    params.push_back(src_param(e, instruction, 1));
    params.push_back(src_param(e, instruction, 2));
    break;
  case 9:
    params.push_back(src_param(e, instruction, 1));
  }
  return params;
}

namespace intcode {
  int64_t& execution::at(size_t position) {
    if (position >= m.size()) {
      m.resize(position + 1);
    }
    return m.at(position);
  }

  memory execution::mem() const {
    return m;
  }

  void execution::write(const int64_t& input) {
    in.push_back(input);
  }

  void execution::write_all(const input& inputs) {
    in.insert(in.end(), inputs.begin(), inputs.end());
  }

  int64_t execution::read() {
    auto v = out.front();
    out.pop_front();
    return v;
  }

  output execution::read_all() {
    auto copy = out;
    out.clear();
    return copy;
  }

  void execution::run_instruction() {
    auto instruction = at(position);
    auto op = opcode(instruction);
    if (op == 99) {
      state = execution_state::halted;
      return;
    }
    auto n = n_params(op);
    auto params = parse_params(*this, instruction);
    int64_t operand1, operand2, destination, value;
    auto new_position = position + n + 1;
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
      at(destination) = value;
      break;
    // read input
    case 3:
      if (in.empty()) {
        state = execution_state::waiting;
        return;
      }
      value = in.front();
      in.pop_front();
      destination = params[0];
      at(destination) = value;
      break;
    // produce output
    case 4:
      value = params[0];
      out.push_back(value);
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
        at(destination) = 1;
      } else {
        at(destination) = 0;
      }
      break;
    // adjust relative base
    case 9:
      value = params[0];
      relative_base += value;
      break;
    }
    position = new_position;
    state = execution_state::running;
  }

  void execution::run() {
    do {
      run_instruction();
    } while (state == execution_state::running);
  }

  int opcode(int64_t instruction) {
    return instruction % 100;
  }

  parameter_mode mode(int64_t instruction, int n) {
    auto mask = 10;
    for (auto i = 1; i <= n; i++) {
      mask *= 10;
    }
    auto mode_num = (instruction % (mask * 10)) / mask;
    // Just some default value, to silence the warnings of `m` being used
    // possible uninitialized.
    parameter_mode m = parameter_mode::position;
    switch (mode_num) {
    case 0:
      m = parameter_mode::position;
      break;
    case 1:
      m = parameter_mode::immediate;
      break;
    case 2:
      m = parameter_mode::relative;
      break;
    }
    return m;
  }

  memory parse(const std::string& s) {
    std::vector<std::string> parts = absl::StrSplit(s, ",");
    memory m;
    m.reserve(parts.size());
    for (auto part : parts) {
      m.push_back(std::stol(part));
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
    // read input, produce output, adjust relative base
    case 3:
    case 4:
    case 9:
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
    e.write_all(input);
    e.run();
    return {e.mem(), e.read_all()};
  }
}
