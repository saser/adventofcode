#include "year2019/day04/day04_internal.h"

#include <vector>

#include "gtest/gtest.h"

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
