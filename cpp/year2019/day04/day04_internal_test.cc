#include "year2019/day04/day04_internal.h"

#include <vector>

#include "gtest/gtest.h"

TEST(Year2019Day04Internal, DigitsExample1) {
  int password = 111111;
  std::vector<int> expected = {1, 1, 1, 1, 1, 1};
  EXPECT_EQ(expected, day04::digits(password));
}

TEST(Year2019Day04Internal, DigitsExample2) {
  int password = 223450;
  std::vector<int> expected = {2, 2, 3, 4, 5, 0};
  EXPECT_EQ(expected, day04::digits(password));
}

TEST(Year2019Day04Internal, DigitsExample3) {
  int password = 123789;
  std::vector<int> expected = {1, 2, 3, 7, 8, 9};
  EXPECT_EQ(expected, day04::digits(password));
}

TEST(Year2019Day04Internal, Part1Example1) {
  std::vector<int> digits = {1, 1, 1, 1, 1, 1};
  EXPECT_TRUE(day04::has_double(digits));
  EXPECT_TRUE(day04::non_decreasing(digits));
}

TEST(Year2019Day04Internal, Part1Example2) {
  std::vector<int> digits = {2, 2, 3, 4, 5, 0};
  EXPECT_TRUE(day04::has_double(digits));
  EXPECT_FALSE(day04::non_decreasing(digits));
}

TEST(Year2019Day04Internal, Part1Example3) {
  std::vector<int> digits = {1, 2, 3, 7, 8, 9};
  EXPECT_FALSE(day04::has_double(digits));
  EXPECT_TRUE(day04::non_decreasing(digits));
}
