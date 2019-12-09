#include "{{.FullYear}}/{{.FullDay}}/{{.FullDay}}.h"

#include <fstream>
#include <sstream>
#include <string>

#include "gtest/gtest.h"

#include "adventofcode.h"

TEST(Year{{.Year}}Day{{.PaddedDay}}, Part1Example1) {
  std::istringstream input("some input here");
  std::string output = "some output here";
  adventofcode::answer_t a = {{.FullDay}}::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
}

TEST(Year{{.Year}}Day{{.PaddedDay}}, Part1Actual) {
  std::ifstream input("{{.FullYear}}/testdata/{{.PaddedDay}}");
  std::string output = "some output here";
  adventofcode::answer_t a = {{.FullDay}}::part1(input);
  EXPECT_EQ("", a.error);
  EXPECT_EQ(output, a.answer);
  input.close();
}

// TEST(Year{{.Year}}Day{{.PaddedDay}}, Part2Example1) {
//   std::istringstream input("some input here");
//   std::string output = "some output here";
//   adventofcode::answer_t a = {{.FullDay}}::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
// }

// TEST(Year{{.Year}}Day{{.PaddedDay}}, Part2Actual) {
//   std::ifstream input("{{.FullYear}}/testdata/{{.PaddedDay}}");
//   std::string output = "some output here";
//   adventofcode::answer_t a = {{.FullDay}}::part2(input);
//   EXPECT_EQ("", a.error);
//   EXPECT_EQ(output, a.answer);
//   input.close();
// }
