#include "year2019/day24/day24.h"

#include <fstream>
#include <string>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day24, Part1Example1) {
  std::ifstream input("year2019/day24/testdata/ex");
  std::string output = "2129920";
  adventofcode::answer_t a = day24::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day24, Part1Actual) {
  std::ifstream input("year2019/testdata/24");
  std::string output = "28717468";
  adventofcode::answer_t a = day24::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

// TEST(Year2019Day24, Part2Example1) {
//   std::istringstream input("some input here");
//   std::string output = "some output here";
//   adventofcode::answer_t a = day24::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
// }

// TEST(Year2019Day24, Part2Actual) {
//   std::ifstream input("year2019/testdata/24");
//   std::string output = "some output here";
//   adventofcode::answer_t a = day24::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
//   input.close();
// }
