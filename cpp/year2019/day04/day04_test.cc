#include "year2019/day04/day04.h"

#include <fstream>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day04, Part1Actual) {
  std::ifstream input("year2019/testdata/04");
  std::string output = "";
  adventofcode::answer_t a = day04::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
