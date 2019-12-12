#include "year2019/day12/day12.h"

#include <fstream>
#include <string>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day12, Part1Example1) {
  std::ifstream input("year2019/day12/testdata/ex1");
  std::string output = "179";
  adventofcode::answer_t a = day12::part1(input, 10);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day12, Part1Example2) {
  std::ifstream input("year2019/day12/testdata/ex2");
  std::string output = "1940";
  adventofcode::answer_t a = day12::part1(input, 100);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day12, Part1Actual) {
  std::ifstream input("year2019/testdata/12");
  std::string output = "10028";
  adventofcode::answer_t a = day12::part1(input, day12::STEPS);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day12, Part2Example1) {
  std::ifstream input("year2019/day12/testdata/ex1");
  std::string output = "2772";
  adventofcode::answer_t a = day12::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day12, Part2Example2) {
  std::ifstream input("year2019/day12/testdata/ex2");
  std::string output = "4686774924";
  adventofcode::answer_t a = day12::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day12, Part2Actual) {
  std::ifstream input("year2019/testdata/12");
  std::string output = "314610635824376";
  adventofcode::answer_t a = day12::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
