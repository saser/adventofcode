#include "year2019/day07/day07.h"

#include <fstream>
#include <sstream>
#include <string>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day07, Part1Example1) {
  std::istringstream input("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
  std::string output = "43210";
  adventofcode::answer_t a = day07::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day07, Part1Example2) {
  std::ifstream input("year2019/day07/testdata/p1ex2");
  std::string output = "54321";
  adventofcode::answer_t a = day07::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day07, Part1Example3) {
  std::ifstream input("year2019/day07/testdata/p1ex3");
  std::string output = "65210";
  adventofcode::answer_t a = day07::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day07, Part1Actual) {
  std::ifstream input("year2019/testdata/07");
  std::string output = "30940";
  adventofcode::answer_t a = day07::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

// TEST(Year2019Day07, Part2Example1) {
//   std::istringstream input("some input here");
//   std::string output = "some output here";
//   adventofcode::answer_t a = day07::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
// }

// TEST(Year2019Day07, Part2Actual) {
//   std::ifstream input("year2019/testdata/07");
//   std::string output = "some output here";
//   adventofcode::answer_t a = day07::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
//   input.close();
// }
