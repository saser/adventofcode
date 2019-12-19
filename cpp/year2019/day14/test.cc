#include "year2019/day14/day14.h"

#include <fstream>
#include <string>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year2019Day14, Part1Example1) {
  std::ifstream input("year2019/day14/testdata/p1ex1");
  std::string output = "31";
  adventofcode::answer_t a = day14::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day14, Part1Example2) {
  std::ifstream input("year2019/day14/testdata/p1ex2");
  std::string output = "165";
  adventofcode::answer_t a = day14::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day14, Part1Example3) {
  std::ifstream input("year2019/day14/testdata/p1ex3");
  std::string output = "13312";
  adventofcode::answer_t a = day14::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day14, Part1Example4) {
  std::ifstream input("year2019/day14/testdata/p1ex4");
  std::string output = "180697";
  adventofcode::answer_t a = day14::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day14, Part1Example5) {
  std::ifstream input("year2019/day14/testdata/p1ex5");
  std::string output = "2210736";
  adventofcode::answer_t a = day14::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day14, Part1Actual) {
  std::ifstream input("year2019/testdata/14");
  std::string output = "2486514";
  adventofcode::answer_t a = day14::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day14, Part2Example1) {
  std::ifstream input("year2019/day14/testdata/p1ex3");
  std::string output = "82892753";
  adventofcode::answer_t a = day14::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day14, Part2Example2) {
  std::ifstream input("year2019/day14/testdata/p1ex4");
  std::string output = "5586022";
  adventofcode::answer_t a = day14::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day14, Part2Example3) {
  std::ifstream input("year2019/day14/testdata/p1ex5");
  std::string output = "460664";
  adventofcode::answer_t a = day14::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day14, Part2Actual) {
  std::ifstream input("year2019/testdata/14");
  std::string output = "998536";
  adventofcode::answer_t a = day14::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
