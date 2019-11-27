#include "year2019/day01.h"

#include <fstream>
#include <sstream>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day01, Part1Example1) {
  std::istringstream input("12");
  std::string output = "2";
  adventofcode::answer_t a = day01::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day01, Part1Example2) {
  std::istringstream input("14");
  std::string output = "2";
  adventofcode::answer_t a = day01::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day01, Part1Example3) {
  std::istringstream input("1969");
  std::string output = "654";
  adventofcode::answer_t a = day01::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day01, Part1Example4) {
  std::istringstream input("100756");
  std::string output = "33583";
  adventofcode::answer_t a = day01::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day01, Part1Actual) {
  std::ifstream input("year2019/testdata/01");
  std::string output = "";
  adventofcode::answer_t a = day01::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
