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
