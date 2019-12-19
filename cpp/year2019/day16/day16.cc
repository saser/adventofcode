#include "year2019/day16/day16.h"

#include <cmath>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "adventofcode.h"

adventofcode::answer_t solve(std::istream& is, int part);
int output(const std::vector<int>& v, unsigned int n);
void phase(std::vector<int>& v);

namespace day16 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  std::string input;
  std::getline(is, input);
  std::vector<int> v;
  for (auto c : input) {
    v.push_back(c - '0');
  }
  for (auto i = 0; i < 100; i++) {
    phase(v);
  }
  std::stringbuf sb;
  std::ostream os {&sb};
  for (std::vector<int>::size_type i = 0; i < 8; i++) {
    os << v[i];
  }
  return adventofcode::ok(sb.str());
}

int output(const std::vector<int>& v, unsigned int n) {
  auto cycle_length = 4 * n;
  std::vector<int> parts {0, 1, 0, -1};
  auto sum = 0;
  for (std::vector<int>::size_type i = 0; i < v.size(); i++) {
    auto k = (i + 1) % cycle_length;
    auto part = parts[k / n];
    sum += v[i] * part;
  }
  return std::abs(sum) % 10;
}

void phase(std::vector<int>& v) {
  auto copy = v;
  for (std::vector<int>::size_type i = 0; i < v.size(); i++) {
    v[i] = output(copy, i + 1);
  }
}
