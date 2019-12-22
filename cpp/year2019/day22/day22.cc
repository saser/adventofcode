#include "year2019/day22/day22.h"

#include <algorithm>
#include <cmath>
#include <deque>
#include <istream>
#include <regex>
#include <string>

#include "adventofcode.h"

struct deck_t {
  std::deque<uint16_t> d;

  deck_t into_new() const;
  deck_t cut(int n) const;
  deck_t increment(unsigned int n) const;
};

adventofcode::answer_t solve(std::istream& is, int part);

namespace day22 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  std::deque<uint16_t> d;
  for (auto i = 0u; i < 10007; i++) {
    d.push_back(i);
  }
  deck_t deck {d};
  std::string line;
  while (std::getline(is, line)) {
    if (line == "deal into new stack") {
      deck = deck.into_new();
      continue;
    }
    std::smatch match;
    const std::regex cut_re(R"(cut (-?\d+))");
    if (std::regex_match(line, match, cut_re)) {
      auto n = std::stoi(match[1]);
      deck = deck.cut(n);
      continue;
    }
    const std::regex increment_re(R"(deal with increment (\d+))");
    if (std::regex_match(line, match, increment_re)) {
      auto n = std::stoi(match[1]);
      deck = deck.increment(n);
      continue;
    }
  }
  for (auto i = 0u; i < deck.d.size(); i++) {
    if (deck.d.at(i) == 2019) {
      return adventofcode::ok(std::to_string(i));
    }
  }
  return adventofcode::err("no solution found");
}

deck_t deck_t::into_new() const {
  auto copy = d;
  std::reverse(copy.begin(), copy.end());
  return deck_t {copy};
}

deck_t deck_t::cut(int n) const {
  auto copy = d;
  if (n >= 0) {
    for (auto i = 0; i < n; i++) {
      auto popped = copy.front();
      copy.pop_front();
      copy.push_back(popped);
    }
  } else {
    auto an = std::abs(n);
    for (auto i = 0; i < an; i++) {
      auto popped = copy.back();
      copy.pop_back();
      copy.push_front(popped);
    }
  }
  return deck_t {copy};
}

deck_t deck_t::increment(unsigned int n) const {
  auto copy = d;
  auto len = copy.size();
  auto i = 0;
  auto it = d.cbegin();
  while (it != d.cend()) {
    copy[i] = *it;
    i = (i + n) % len;
    it++;
  }
  return deck_t {copy};
}
