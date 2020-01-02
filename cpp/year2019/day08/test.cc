#include "cpp/year2019/day08/day08.h"

#include <istream>
#include <fstream>
#include <string>

#include "gtest/gtest.h"

#include "cpp/adventofcode.h"

TEST(Year2019Day08, Part1Actual) {
  std::ifstream input("inputs/2019/08");
  std::string output = "2032";
  adventofcode::answer_t a = day08::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

TEST(Year2019Day08, Part2Actual) {
  std::ifstream input("inputs/2019/08");
  std::ifstream output_file("cpp/year2019/day08/testdata/p2out");
  std::string output(std::istreambuf_iterator<char>(output_file), {});
  adventofcode::answer_t a = day08::part2(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}
