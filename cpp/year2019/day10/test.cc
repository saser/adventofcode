#include "cpp/year2019/day10/day10.h"

#include <fstream>
#include <string>

#include "gtest/gtest.h"

#include "cpp/adventofcode.h"

TEST(Year2019Day10, Part1Example1) {
  std::ifstream input("cpp/year2019/day10/testdata/ex1");
  std::string output = "8";
  adventofcode::answer_t a = day10::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day10, Part1Example2) {
  std::ifstream input("cpp/year2019/day10/testdata/ex2");
  std::string output = "33";
  adventofcode::answer_t a = day10::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day10, Part1Example3) {
  std::ifstream input("cpp/year2019/day10/testdata/ex3");
  std::string output = "35";
  adventofcode::answer_t a = day10::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day10, Part1Example4) {
  std::ifstream input("cpp/year2019/day10/testdata/ex4");
  std::string output = "41";
  adventofcode::answer_t a = day10::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day10, Part1Example5) {
  std::ifstream input("cpp/year2019/day10/testdata/ex5");
  std::string output = "210";
  adventofcode::answer_t a = day10::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day10, Part1Actual) {
  std::ifstream input("inputs/2019/10");
  std::string output = "286";
  adventofcode::answer_t a = day10::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day10, Part2Example1) {
  std::ifstream input("cpp/year2019/day10/testdata/ex5");
  std::string output = "802";
  adventofcode::answer_t a = day10::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day10, Part2Actual) {
  std::ifstream input("inputs/2019/10");
  std::string output = "504";
  adventofcode::answer_t a = day10::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
