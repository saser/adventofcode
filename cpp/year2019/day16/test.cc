#include "year2019/day16/day16.h"

#include <fstream>
#include <sstream>
#include <string>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day16, Part1Example1) {
  std::istringstream input("80871224585914546619083218645595");
  std::string output = "24176176";
  adventofcode::answer_t a = day16::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day16, Part1Example2) {
  std::istringstream input("19617804207202209144916044189917");
  std::string output = "73745418";
  adventofcode::answer_t a = day16::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day16, Part1Example3) {
  std::istringstream input("69317163492948606335995924319873");
  std::string output = "52432133";
  adventofcode::answer_t a = day16::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day16, Part1Actual) {
  std::ifstream input("year2019/testdata/16");
  std::string output = "";
  adventofcode::answer_t a = day16::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

// TEST(Year2019Day16, Part2Example1) {
//   std::istringstream input("some input here");
//   std::string output = "some output here";
//   adventofcode::answer_t a = day16::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
// }

// TEST(Year2019Day16, Part2Actual) {
//   std::ifstream input("year2019/testdata/16");
//   std::string output = "some output here";
//   adventofcode::answer_t a = day16::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
//   input.close();
// }
