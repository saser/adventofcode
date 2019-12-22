#include "year2019/day22/day22.h"

#include <algorithm>
#include <cmath>
#include <deque>
#include <istream>
#include <regex>
#include <string>

#include "adventofcode.h"

struct deck_t {
  uint64_t modulo;
  uint64_t offset;
  uint64_t increment;

  deck_t(uint64_t _modulo) : modulo(_modulo), offset(0), increment(1) {}

  uint64_t get(uint64_t n) const;
  void into_new();
  void cut_by(int64_t n);
  void increment_by(uint64_t n);
};

uint64_t mod_mul(uint64_t a, uint64_t b, uint64_t modulo);
uint64_t mod_exp(uint64_t base, uint64_t exponent, uint64_t modulo);
uint64_t mod_geo_sum(uint64_t a, uint64_t r, uint64_t n, uint64_t modulo);

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
  uint64_t modulo = 10007;
  if (part == 2) {
    modulo = 119315717514047;
  }
  deck_t deck(modulo);
  std::string line;
  while (std::getline(is, line)) {
    if (line == "deal into new stack") {
      deck.into_new();
      continue;
    }
    std::smatch match;
    const std::regex cut_re(R"(cut (-?\d+))");
    if (std::regex_match(line, match, cut_re)) {
      auto n = std::stoi(match[1]);
      deck.cut_by(n);
      continue;
    }
    const std::regex increment_re(R"(deal with increment (\d+))");
    if (std::regex_match(line, match, increment_re)) {
      auto n = std::stoi(match[1]);
      deck.increment_by(n);
      continue;
    }
  }
  if (part == 1) {
    for (auto i = 0u; i < modulo; i++) {
      if (deck.get(i) == 2019) {
        return adventofcode::ok(std::to_string(i));
      }
    }
    return adventofcode::err("no solution found");
  }
  auto iterations = 101741582076661;
  deck.offset = mod_geo_sum(deck.offset, deck.increment, iterations, modulo);
  deck.increment = mod_exp(deck.increment, iterations, modulo);
  return adventofcode::ok(std::to_string(deck.get(2020)));
}

uint64_t mod_mul(uint64_t a, uint64_t b, uint64_t modulo) {
  uint64_t result = 0;
  a %= modulo;
  while (b > 0) {
    if (b % 2 == 1) {
      result = (result + a) % modulo;
    }
    a = (a * 2) % modulo;
    b /= 2;
  }
  return result;
}

uint64_t mod_exp(uint64_t base, uint64_t exponent, uint64_t modulo) {
  if (exponent == 0) {
    return 1;
  } else if (exponent % 2 == 0) {
    return mod_exp(mod_mul(base, base, modulo), exponent / 2, modulo);
  } else {
    return mod_mul(base, mod_exp(base, exponent - 1, modulo), modulo);
  }
}

uint64_t mod_geo_sum(uint64_t a, uint64_t r, uint64_t n, uint64_t modulo) {
  auto enumerator = modulo + 1 - mod_exp(r, n, modulo);
  auto denominator = mod_exp(modulo + 1 - r, modulo - 2, modulo);
  return mod_mul(a, mod_mul(enumerator, denominator, modulo), modulo);
}

uint64_t deck_t::get(uint64_t n) const {
  return (offset + mod_mul(increment, n, modulo)) % modulo;
}

void deck_t::into_new() {
  increment = modulo - increment;
  offset = (offset + increment) % modulo;
}

void deck_t::cut_by(int64_t n) {
  if (n >= 0) {
    offset = get(n);
  } else {
    offset = get(modulo + n);
  }
}

void deck_t::increment_by(uint64_t n) {
  increment = mod_mul(increment, mod_exp(n, modulo - 2, modulo), modulo);
}
