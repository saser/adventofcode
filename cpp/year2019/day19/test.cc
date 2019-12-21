#include "year2019/day19/day19.h"

#include <fstream>
#include <sstream>
#include <string>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day19, Part1Actual) {
  std::ifstream input("year2019/testdata/19");
  std::string output = "";
  adventofcode::answer_t a = day19::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

// TEST(Year2019Day19, Part2Example1) {
//   std::istringstream input("some input here");
//   std::string output = "some output here";
//   adventofcode::answer_t a = day19::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
// }

// TEST(Year2019Day19, Part2Actual) {
//   std::ifstream input("year2019/testdata/19");
//   std::string output = "some output here";
//   adventofcode::answer_t a = day19::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
//   input.close();
// }
