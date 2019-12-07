#include "year2019/day05/day05.h"

#include <fstream>

#include "gtest/gtest.h"

TEST(Year2019Day05, Part1Actual) {
  std::ifstream input("year2019/testdata/05");
  std::string output = "";
  adventofcode::answer_t a = day05::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
