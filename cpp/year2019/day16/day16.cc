#include "cpp/year2019/day16/day16.h"

#include <cmath>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "cpp/adventofcode.h"

adventofcode::answer_t solve(std::istream& is, int part);
int output(const std::vector<int>& v, unsigned int n);
void phase(std::vector<int>& v);
void fast_phase(std::vector<int>& v, std::vector<int>::size_type offset);

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
  std::vector<int>::size_type offset = 0;
  if (part == 2) {
    auto copy = v;
    for (auto i = 1; i < 10000; i++) {
      v.insert(v.end(), copy.begin(), copy.end());
    }
    for (auto it = v.begin(); it != v.begin() + 7; it++) {
      offset = offset * 10 + *it;
    }
  }
  for (auto i = 0; i < 100; i++) {
    if (part == 1) {
      phase(v);
    } else {
      fast_phase(v, offset);
    }
  }
  std::stringbuf sb;
  std::ostream os {&sb};
  for (std::vector<int>::size_type i = 0; i < 8; i++) {
    os << v[i + offset];
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

// This is based on the fact that if `offset` points to an element in the later
// half of the input signal, then we can simply calculate the new digits as the
// cumulative sum of the last digits, starting from `offset`. For an example of
// this, see how the last 4 digits are calculated in the description of the
// puzzle.
//
// I was very close to coming up with this solution myself, since I noted that
// cumulative sum pattern, but I needed Reddit as help to push me over the
// edge. I thought I still had to calculate the entire output signal, including
// the first half as well.
void fast_phase(std::vector<int>& v, std::vector<int>::size_type offset) {
  auto copy = v;
  auto sum = 0;
  for (std::vector<int>::size_type i = v.size() - 1; i >= offset; i--) {
    sum += copy[i];
    v[i] = sum % 10;
  }
}
