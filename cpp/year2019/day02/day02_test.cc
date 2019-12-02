#include "year2019/day02/day02.h"

#include <fstream>
#include <sstream>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day02, Part1Actual) {
  std::ifstream input("year2019/testdata/02");
  std::string output = "";
  adventofcode::answer_t a = day02::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
