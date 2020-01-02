#include "cpp/year2019/day06/day06.h"

#include <fstream>

#include "gtest/gtest.h"

TEST(Year2019Day06, Part1Example) {
  std::ifstream input("cpp/year2019/day06/testdata/p1ex");
  std::string output = "42";
  adventofcode::answer_t a = day06::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day06, Part1Actual) {
  std::ifstream input("inputs/2019/06");
  std::string output = "154386";
  adventofcode::answer_t a = day06::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day06, Part2Example) {
  std::ifstream input("cpp/year2019/day06/testdata/p2ex");
  std::string output = "4";
  adventofcode::answer_t a = day06::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day06, Part2Actual) {
  std::ifstream input("inputs/2019/06");
  std::string output = "346";
  adventofcode::answer_t a = day06::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
