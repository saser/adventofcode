#include "cpp/{{.FullYear}}/{{.FullDay}}/{{.FullDay}}.h"

#include <fstream>
#include <sstream>
#include <string>

#include "gtest/gtest.h"

#include "cpp/adventofcode.h"

TEST(Year{{.Year}}Day{{.PaddedDay}}, Part1Example1) {
  std::istringstream input("");
  std::string output = "";
  adventofcode::answer_t a = {{.FullDay}}::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year{{.Year}}Day{{.PaddedDay}}, Part1Actual) {
  std::ifstream input("inputs/{{.Year}}/{{.PaddedDay}}");
  std::string output = "";
  adventofcode::answer_t a = {{.FullDay}}::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

// TEST(Year{{.Year}}Day{{.PaddedDay}}, Part2Example1) {
//   std::istringstream input("");
//   std::string output = "";
//   adventofcode::answer_t a = {{.FullDay}}::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
// }

// TEST(Year{{.Year}}Day{{.PaddedDay}}, Part2Actual) {
//   std::ifstream input("inputs/{{.Year}}/{{.PaddedDay}}");
//   std::string output = "";
//   adventofcode::answer_t a = {{.FullDay}}::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
//   input.close();
// }
