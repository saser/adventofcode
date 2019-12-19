#include "year2019/day17/day17.h"

#include <fstream>
#include <string>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day17, Part1Actual) {
  std::ifstream input("year2019/testdata/17");
  std::string output = "3336";
  adventofcode::answer_t a = day17::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day17, Part2Actual) {
  std::ifstream input("year2019/testdata/17");
  std::string output = "597517";
  adventofcode::answer_t a = day17::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
