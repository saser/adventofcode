#include "year2019/day20/day20.h"

#include <fstream>
#include <string>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day20, Part1Example1) {
  std::ifstream input("year2019/day20/testdata/p1ex1");
  std::string output = "23";
  adventofcode::answer_t a = day20::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day20, Part1Example2) {
  std::ifstream input("year2019/day20/testdata/p1ex2");
  std::string output = "58";
  adventofcode::answer_t a = day20::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day20, Part1Actual) {
  std::ifstream input("year2019/testdata/20");
  std::string output = "714";
  adventofcode::answer_t a = day20::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day20, Part2Example1) {
  std::ifstream input("year2019/day20/testdata/p2ex1");
  std::string output = "396";
  adventofcode::answer_t a = day20::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day20, Part2Actual) {
  std::ifstream input("year2019/testdata/20");
  std::string output = "";
  adventofcode::answer_t a = day20::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
