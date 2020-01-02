#include "cpp/year2019/day11/day11.h"

#include <fstream>
#include <string>

#include "gtest/gtest.h"

#include "cpp/adventofcode.h"

TEST(Year2019Day11, Part1Actual) {
  std::ifstream input("inputs/2019/11");
  std::string output = "2093";
  adventofcode::answer_t a = day11::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day11, Part2Actual) {
  std::ifstream input("inputs/2019/11");
  std::ifstream output_file("cpp/year2019/day11/testdata/out");
  std::string output(std::istreambuf_iterator<char>(output_file), {});
  adventofcode::answer_t a = day11::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
