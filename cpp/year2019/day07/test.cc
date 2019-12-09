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
  std::istringstream input("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
  std::string output = "54321";
  adventofcode::answer_t a = day07::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day07, Part1Example3) {
  std::istringstream input("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
  std::string output = "65210";
  adventofcode::answer_t a = day07::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
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
