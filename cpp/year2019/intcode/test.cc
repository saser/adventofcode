#include "year2019/intcode/intcode.h"

#include <string>
#include <vector>

#include "gtest/gtest.h"

TEST(Intcode, Opcode) {
  for (auto it = intcode::OPCODES.begin(); it != intcode::OPCODES.end(); it++) {
    int opcode = *it;
    // All parameters in position mode.
    EXPECT_EQ(opcode, intcode::opcode(opcode));
    // First parameter in immediate mode.
    EXPECT_EQ(opcode, intcode::opcode(opcode + 100));
    // Second parameter in immediate mode.
    EXPECT_EQ(opcode, intcode::opcode(opcode + 1000));
    // Third parameter in immediate mode.
    EXPECT_EQ(opcode, intcode::opcode(opcode + 10000));
    // First and second parameter in immediate mode.
    EXPECT_EQ(opcode, intcode::opcode(opcode + 100 + 1000));
    // First and third parameter in immediate mode.
    EXPECT_EQ(opcode, intcode::opcode(opcode + 100 + 10000));
    // Second and third parameter in immediate mode.
    EXPECT_EQ(opcode, intcode::opcode(opcode + 1000 + 10000));
    // All parameters in immediate mode.
    EXPECT_EQ(opcode, intcode::opcode(opcode + 100 + 1000 + 10000));
  }
}

TEST(Intcode, ImmediateMode) {
  for (auto it = intcode::OPCODES.begin(); it != intcode::OPCODES.end(); it++) {
    int opcode = *it;
    // All parameters in position mode.
    for (int n = 1; n <= 3; n++) {
      EXPECT_FALSE(intcode::immediate_mode(opcode, n));
    }
    // First parameter in immediate mode.
    EXPECT_TRUE(intcode::immediate_mode(opcode + 100, 1));
    // Second parameter in immediate mode.
    EXPECT_TRUE(intcode::immediate_mode(opcode + 1000, 2));
    // Third parameter in immediate mode.
    EXPECT_TRUE(intcode::immediate_mode(opcode + 10000, 3));
    // First and second parameter in immediate mode.
    EXPECT_TRUE(intcode::immediate_mode(opcode + 100 + 1000, 1));
    EXPECT_TRUE(intcode::immediate_mode(opcode + 100 + 1000, 2));
    // First and third parameter in immediate mode.
    EXPECT_TRUE(intcode::immediate_mode(opcode + 100 + 10000, 1));
    EXPECT_TRUE(intcode::immediate_mode(opcode + 100 + 10000, 3));
    // Second and third parameter in immediate mode.
    EXPECT_TRUE(intcode::immediate_mode(opcode + 1000 + 10000, 2));
    EXPECT_TRUE(intcode::immediate_mode(opcode + 1000 + 10000, 3));
    // All parameters in immediate mode.
    EXPECT_TRUE(intcode::immediate_mode(opcode + 100 + 1000 + 10000, 1));
    EXPECT_TRUE(intcode::immediate_mode(opcode + 100 + 1000 + 10000, 2));
    EXPECT_TRUE(intcode::immediate_mode(opcode + 100 + 1000 + 10000, 3));
  }
}

TEST(Intcode, NParams) {
  const std::vector<std::pair<int, size_t>> pairs {
    {1, 3}, // addition
    {2, 3}, // multiplication
    {3, 1}, // read input
    {4, 1}, // produce output
    {99, 0}, // halt
  };
  for (auto it = pairs.begin(); it != pairs.end(); it++) {
    auto pair = *it;
    EXPECT_EQ(pair.second, intcode::n_params(pair.first));
  }
}

TEST(Intcode, Parse) {
  std::vector<std::pair<std::string, intcode::memory>> cases {
    {"1,0,0,0,99", {1, 0, 0, 0, 99}},
    {"2,3,0,3,99", {2, 3, 0, 3, 99}},
    {"2,-1,0,3,99", {2, -1, 0, 3, 99}},
  };
  for (auto c : cases) {
    EXPECT_EQ(c.second, intcode::parse(c.first));
  }
}

// Simple addition; from example in day 2.
TEST(IntcodeRun, SimpleAddition) {
  const intcode::memory program = {1, 0, 0, 0, 99};
  const intcode::memory end_state = {2, 0, 0, 0, 99};
  const intcode::input input = {};
  const intcode::output output = {};
  auto p = intcode::run(program, input);
  EXPECT_EQ(end_state, p.first);
  EXPECT_EQ(output, p.second);
}

// Simple multiplication; from example in day 2.
TEST(IntcodeRun, SimpleMultiplication1) {
  const intcode::memory program = {2, 3, 0, 3, 99};
  const intcode::memory end_state = {2, 3, 0, 6, 99};
  const intcode::input input = {};
  const intcode::output output = {};
  auto p = intcode::run(program, input);
  EXPECT_EQ(end_state, p.first);
  EXPECT_EQ(output, p.second);
}

// Another multiplication; from example in day 2.
TEST(IntcodeRun, SimpleMultiplication2) {
  const intcode::memory program = {2, 4, 4, 5, 99, 0};
  const intcode::memory end_state = {2, 4, 4, 5, 99, 9801};
  const intcode::input input = {};
  const intcode::output output = {};
  auto p = intcode::run(program, input);
  EXPECT_EQ(end_state, p.first);
  EXPECT_EQ(output, p.second);
}

// Addition and multiplication; from example in day 2.
TEST(IntcodeRun, AdditionAndMultiplication1) {
  const intcode::memory program = {1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50};
  const intcode::memory end_state = {3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50};
  const intcode::input input = {};
  const intcode::output output = {};
  auto p = intcode::run(program, input);
  EXPECT_EQ(end_state, p.first);
  EXPECT_EQ(output, p.second);
}

// Addition and multiplication; from example in day 2.
TEST(IntcodeRun, AdditionAndMultiplication2) {
  const intcode::memory program = {1, 1, 1, 4, 99, 5, 6, 0, 99};
  const intcode::memory end_state = {30, 1, 1, 4, 2, 5, 6, 0, 99};
  const intcode::input input = {};
  const intcode::output output = {};
  auto p = intcode::run(program, input);
  EXPECT_EQ(end_state, p.first);
  EXPECT_EQ(output, p.second);
}

// An "echo" program; outputs whatever it reads as input. From example in day 5.
TEST(IntcodeRun, Echo) {
  const intcode::input input = {123};
  const intcode::output output = {123};
  const intcode::memory program = {3, 0, 4, 0, 99};
  const intcode::memory end_state = {123, 0, 4, 0, 99};
  auto p = intcode::run(program, input);
  EXPECT_EQ(end_state, p.first);
  EXPECT_EQ(output, p.second);
}
