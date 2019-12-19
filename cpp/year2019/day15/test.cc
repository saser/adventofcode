#include "year2019/day15/day15.h"

#include <fstream>
#include <string>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day15, Part1Actual) {
  std::ifstream input("year2019/testdata/15");
  std::string output = "232";
  adventofcode::answer_t a = day15::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day15, Part2Actual) {
  std::ifstream input("year2019/testdata/15");
  std::string output = "320";
  adventofcode::answer_t a = day15::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
