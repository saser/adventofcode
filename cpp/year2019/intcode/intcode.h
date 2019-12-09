#ifndef ADVENTOFCODE_YEAR2019_INTCODE_INTCODE_H
#define ADVENTOFCODE_YEAR2019_INTCODE_INTCODE_H

#include <deque>
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
    9, // adjust relative base
    99, // halt
  };

  enum parameter_mode {
    position,
    immediate,
    relative,
  };

  // Memory is represented as a vector of ints.
  typedef std::vector<int64_t> memory;

  // Inputs to and outputs from the program are represented as double-ended
  // queues, to simulate that they can both be produced and consumed in no
  // particular order.
  typedef std::deque<int64_t> input;
  typedef std::deque<int64_t> output;

  enum execution_state {
    initialized,
    running,
    waiting,
    halted,
  };

  struct execution {
    memory m;
    input in;
    output out;

    size_t position = 0;
    size_t relative_base = 0;
    execution_state state = execution_state::initialized;

    execution(const memory& _m) : m(_m) {}
  };

  // Return a copy of the current execution memory.
  memory mem(const execution& e);

  // Write an input value to the input of the execution.
  void write(execution& e, const int64_t& i);

  // Write all input values to the input of the execution.
  void write_all(execution& e, const input& i);

  // Read an output value produced by the execution. The value is consumed by
  // reading it. Reading when no output is available is undefiend behavior.
  int64_t read(execution& e);

  // Read all output produced by the execution. All output values are
  // consumed.
  output read_all(execution& e);

  // Runs the instruction at the current position in memory. Returns the next
  // execution state. Running an instruction might cause the execution to
  // enter the `halted` state, from which it will never transition to another
  // state. If the current instruction is to read input, and no input is
  // available, nothing happens, and the returned state will be `waiting`.
  void run_instruction(execution& e);

  // Runs instructions until the execution halts or waits for input.
  void run(execution& e);

  // Determine the opcode for a given memory value.
  int opcode(int64_t instruction);

  // Determine which mode parameter number `n` is in.
  parameter_mode mode(int64_t instruction, int n);

  // Return the number of parameters for the given opcode.
  int n_params(int opcode);

  // Parse a string into a memory.
  memory parse(const std::string& s);

  // Run a program. The program is started at position 0, and the provided
  // inputs are read in order. The first return value is the state of the memory
  // at the time the program halts; the second return value is the output
  // produced by the program.
  std::pair<memory, output> run(const memory& initial, const input& inputs);
}

#endif
