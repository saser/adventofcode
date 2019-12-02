#include "year2019/day02/day02_internal.h"

#include <fstream>
#include <sstream>
#include <vector>

#include "gtest/gtest.h"

TEST(Year2019Day02Internal, RunExample1) {
  std::vector<int> program {1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50};
  int result = 3500;
  EXPECT_EQ(result, day02::run(program));
}

TEST(Year2019Day02Internal, RunExample2) {
  std::vector<int> program {1, 0, 0, 0, 99};
  int result = 2;
  EXPECT_EQ(result, day02::run(program));
}

TEST(Year2019Day02Internal, RunExample3) {
  std::vector<int> program {2, 3, 0, 3, 99};
  int result = 2;
  EXPECT_EQ(result, day02::run(program));
}

TEST(Year2019Day02Internal, RunExample4) {
  std::vector<int> program {2, 4, 4, 5, 99, 0};
  int result = 2;
  EXPECT_EQ(result, day02::run(program));
}

TEST(Year2019Day02Internal, RunExample5) {
  std::vector<int> program {1, 1, 1, 4, 99, 5, 6, 0, 99};
  int result = 30;
  EXPECT_EQ(result, day02::run(program));
}
