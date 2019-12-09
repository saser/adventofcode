#include "year2019/day07/day07.h"

#include <algorithm>
#include <istream>
#include <optional>
#include <string>

#include "adventofcode.h"
#include "year2019/intcode/intcode.h"

adventofcode::answer_t solve(std::istream& is, int part);
template<class T>
std::vector<std::vector<T>> permutations(const std::vector<T>& is);

namespace day07 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  std::string line;
  std::getline(is, line);
  intcode::memory program = intcode::parse(line);
  std::vector<int> phase_settings {0, 1, 2, 3, 4};
  std::optional<int> max_signal;
  for (auto permutation : permutations(phase_settings)) {
    int signal = 0;
    for (auto phase_setting : permutation) {
      auto [ _, output ] = intcode::run(program, {phase_setting, signal});
      signal = output[0];
    }
    max_signal = std::max(max_signal.value_or(signal), signal);
  }
  return adventofcode::ok(std::to_string(*max_signal));
}

template<class T>
std::vector<std::vector<T>> permutations(const std::vector<T>& is) {
  auto n = is.size();
  if (n == 1) {
    return {is};
  }
  std::vector<std::vector<int>> perms;
  for (size_t i = 0; i < n; i++) {
    std::vector<int> rest;
    rest.reserve(n - 1);
    auto mid = is.begin() + i;
    rest.insert(rest.begin(), is.begin(), mid);
    rest.insert(rest.end(), mid + 1, is.end());
    for (auto subperm : permutations(rest)) {
      std::vector<int> perm;
      perm.reserve(n);
      perm.push_back(*mid);
      perm.insert(perm.end(), subperm.begin(), subperm.end());
      perms.push_back(perm);
    }
  }
  return perms;
}
