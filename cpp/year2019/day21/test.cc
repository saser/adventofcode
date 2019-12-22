#include "year2019/day21/day21.h"

#include <fstream>
#include <string>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day21, Part1Actual) {
  std::ifstream input("year2019/testdata/21");
  std::string output = "19352720";
  adventofcode::answer_t a = day21::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day21, Part2Actual) {
  std::ifstream input("year2019/testdata/21");
  std::string output = "1143652885";
  adventofcode::answer_t a = day21::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
