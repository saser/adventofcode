#include "year2019/intcode/intcode.h"

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
