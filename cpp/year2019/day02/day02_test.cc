#include "year2019/day02/day02.h"

#include <fstream>
#include <sstream>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day02, Part1Example1) {
  std::istringstream input("1,9,10,3,2,3,11,0,99,30,40,50");
  std::string output = "3500";
  adventofcode::answer_t a = day02::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day02, Part1Example2) {
  std::istringstream input("1,0,0,0,99");
  std::string output = "2";
  adventofcode::answer_t a = day02::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day02, Part1Example3) {
  std::istringstream input("2,3,0,3,99");
  std::string output = "2";
  adventofcode::answer_t a = day02::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day02, Part1Example4) {
  std::istringstream input("2,4,4,5,99,0");
  std::string output = "2";
  adventofcode::answer_t a = day02::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day02, Part1Example5) {
  std::istringstream input("1,1,1,4,99,5,6,0,99");
  std::string output = "30";
  adventofcode::answer_t a = day02::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year2019Day02, Part1Actual) {
  std::ifstream input("year2019/testdata/02");
  std::string output = "";
  adventofcode::answer_t a = day02::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
