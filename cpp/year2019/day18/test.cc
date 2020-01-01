#include "year2019/day18/day18.h"

#include <fstream>
#include <string>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day18, Part1Example1) {
  std::ifstream input("year2019/day18/testdata/p1ex1");
  std::string output = "8";
  adventofcode::answer_t a = day18::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day18, Part1Example2) {
  std::ifstream input("year2019/day18/testdata/p1ex2");
  std::string output = "86";
  adventofcode::answer_t a = day18::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day18, Part1Example3) {
  std::ifstream input("year2019/day18/testdata/p1ex3");
  std::string output = "132";
  adventofcode::answer_t a = day18::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day18, Part1Example4) {
  std::ifstream input("year2019/day18/testdata/p1ex4");
  std::string output = "136";
  adventofcode::answer_t a = day18::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day18, Part1Example5) {
  std::ifstream input("year2019/day18/testdata/p1ex5");
  std::string output = "81";
  adventofcode::answer_t a = day18::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day18, Part1Actual) {
  std::ifstream input("year2019/testdata/18");
  std::string output = "5402";
  adventofcode::answer_t a = day18::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day18, Part2Example1) {
  std::ifstream input("year2019/day18/testdata/p2ex1");
  std::string output = "8";
  adventofcode::answer_t a = day18::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day18, Part2Example2) {
  std::ifstream input("year2019/day18/testdata/p2ex2");
  std::string output = "24";
  adventofcode::answer_t a = day18::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day18, Part2Example3) {
  std::ifstream input("year2019/day18/testdata/p2ex3");
  std::string output = "32";
  adventofcode::answer_t a = day18::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day18, Part2Example4) {
  std::ifstream input("year2019/day18/testdata/p2ex4");
  std::string output = "72";
  adventofcode::answer_t a = day18::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day18, Part2Actual) {
  std::ifstream input("year2019/testdata/18");
  std::string output = "2138";
  adventofcode::answer_t a = day18::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
