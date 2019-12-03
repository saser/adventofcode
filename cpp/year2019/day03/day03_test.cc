#include "year2019/day03/day03.h"

#include <fstream>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day03, Part1Example1) {
  std::ifstream input("year2019/day03/testdata/example1");
  std::string output = "6";
  adventofcode::answer_t a = day03::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day03, Part1Example2) {
  std::ifstream input("year2019/day03/testdata/example2");
  std::string output = "159";
  adventofcode::answer_t a = day03::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day03, Part1Example3) {
  std::ifstream input("year2019/day03/testdata/example3");
  std::string output = "135";
  adventofcode::answer_t a = day03::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day03, Part1Actual) {
  std::ifstream input("year2019/testdata/03");
  std::string output = "248";
  adventofcode::answer_t a = day03::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day03, Part2Example1) {
  std::ifstream input("year2019/day03/testdata/example1");
  std::string output = "30";
  adventofcode::answer_t a = day03::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day03, Part2Example2) {
  std::ifstream input("year2019/day03/testdata/example2");
  std::string output = "610";
  adventofcode::answer_t a = day03::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day03, Part2Example3) {
  std::ifstream input("year2019/day03/testdata/example3");
  std::string output = "410";
  adventofcode::answer_t a = day03::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day03, Part2Actual) {
  std::ifstream input("year2019/testdata/03");
  std::string output = "";
  adventofcode::answer_t a = day03::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
