#include "cpp/year2019/day24/day24.h"

#include <fstream>
#include <string>

#include "gtest/gtest.h"

#include "cpp/adventofcode.h"

TEST(Year2019Day24, Part1Example1) {
  std::ifstream input("cpp/year2019/day24/testdata/ex");
  std::string output = "2129920";
  adventofcode::answer_t a = day24::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day24, Part1Actual) {
  std::ifstream input("inputs/2019/24");
  std::string output = "28717468";
  adventofcode::answer_t a = day24::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day24, Part2Actual) {
  std::ifstream input("inputs/2019/24");
  std::string output = "2014";
  adventofcode::answer_t a = day24::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
