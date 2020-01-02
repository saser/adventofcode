#include "cpp/year2019/day07/day07.h"

#include <algorithm>
#include <istream>
#include <optional>
#include <string>

#include "cpp/adventofcode.h"
#include "cpp/year2019/intcode/intcode.h"

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
  std::vector<int> phase_settings;
  if (part == 1) {
    phase_settings = {0, 1, 2, 3, 4};
  } else {
    phase_settings = {5, 6, 7, 8, 9};
  }
  auto perms = permutations(phase_settings);
  std::optional<int> max_signal;
  std::vector<intcode::execution> executions;
  executions.reserve(5);
  for (auto permutation : permutations(phase_settings)) {
    executions.clear();
    for (size_t i = 0; i < permutation.size(); i++) {
      auto e = intcode::execution {program};
      e.write(permutation.at(i));
      executions.push_back(e);
    }
    auto signal = 0;
    auto current = 0;
    auto& last = executions.at(executions.size() - 1);
    while (last.state != intcode::execution_state::halted) {
      auto &e = executions.at(current);
      e.write(signal);
      e.run();
      signal = e.read();
      current = (current + 1) % executions.size();
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
