#include "year2019/day25/day25.h"

#include <fstream>
#include <string>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day25, Part1Actual) {
  std::ifstream input("year2019/testdata/25");
  std::string output = "2622472";
  adventofcode::answer_t a = day25::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

// TEST(Year2019Day25, Part2Example1) {
//   std::istringstream input("some input here");
//   std::string output = "some output here";
//   adventofcode::answer_t a = day25::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
// }

// TEST(Year2019Day25, Part2Actual) {
//   std::ifstream input("year2019/testdata/25");
//   std::string output = "some output here";
//   adventofcode::answer_t a = day25::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
//   input.close();
// }
