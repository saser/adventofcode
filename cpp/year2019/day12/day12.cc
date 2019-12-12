#include "year2019/day12/day12.h"

#include <cmath>
#include <istream>
#include <regex>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "adventofcode.h"

using state = std::vector<std::pair<int, int>>;

adventofcode::answer_t solve(std::istream& is, unsigned int steps, int part);
std::tuple<state, state, state> parse(std::istream& is);
std::vector<state> find_cycle(const state& state);
unsigned int total_energy(const std::vector<state>& axes);
long unsigned int lcm_all(const std::vector<long unsigned int>& ns);

namespace day12 {
  adventofcode::answer_t part1(std::istream& is, unsigned int steps) {
    return solve(is, steps, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 0, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, unsigned int steps, int part) {
  auto [x_state, y_state, z_state] = parse(is);
  auto x_cycle = find_cycle(x_state);
  auto x_n = x_cycle.size();
  state final_x_state = x_cycle[steps % x_n];
  auto y_cycle = find_cycle(y_state);
  auto y_n = y_cycle.size();
  state final_y_state = y_cycle[steps % y_n];
  auto z_cycle = find_cycle(z_state);
  auto z_n = z_cycle.size();
  state final_z_state = z_cycle[steps % z_n];
  if (part == 1) {
    auto total = total_energy({final_x_state, final_y_state, final_z_state});
    return adventofcode::ok(std::to_string(total));
  }
  auto full_cycle_n = lcm_all({x_n, y_n, z_n});
  return adventofcode::ok(std::to_string(full_cycle_n));
}

std::tuple<state, state, state> parse(std::istream& is) {
  std::string line;
  state x_state;
  state y_state;
  state z_state;
  const std::regex re(R"(<x=(-?\d+), y=(-?\d+), z=(-?\d+)>)");
  std::smatch match;
  while (std::getline(is, line)) {
    if (!std::regex_match(line, match, re)) {
      continue;
    }
    auto x = std::stoi(match[1]);
    x_state.push_back({x, 0});
    auto y = std::stoi(match[2]);
    y_state.push_back({y, 0});
    auto z = std::stoi(match[3]);
    z_state.push_back({z, 0});
  }
  return {x_state, y_state, z_state};
}

void apply_gravity_axis(std::pair<int, int>& body1, std::pair<int, int>& body2) {
  auto& [p1, v1] = body1;
  auto& [p2, v2] = body2;
  if (p1 == p2) {
    return;
  }
  if (p1 > p2) {
    v1 -= 1;
    v2 += 1;
  } else {
    v1 += 1;
    v2 -= 1;
  }
}

void apply_gravity(state& state) {
  using size = state::size_type;
  for (size i = 0; i < state.size(); i++) {
    for (size j = i + 1; j < state.size(); j++) {
      apply_gravity_axis(state.at(i), state.at(j));
    }
  }
}

void apply_velocity(state& state) {
  for (auto& [ p, v ] : state) {
    p += v;
  }
}

void n_body_step(state& state) {
  apply_gravity(state);
  apply_velocity(state);
}

bool states_equal(const state& s1, const state& s2) {
  if (s1.size() != s2.size()) {
    return false;
  }
  for (auto i = 0lu; i < s1.size(); i++) {
    auto [p1, v1] = s1[i];
    auto [p2, v2] = s2[i];
    if (p1 != p2 || v1 != v2) {
      return false;
    }
  }
  return true;
}

std::vector<state> find_cycle(const state& initial_state) {
  std::vector<state> states_v;
  state state = initial_state;
  do {
    states_v.push_back(state);
    n_body_step(state);
  } while (!states_equal(state, initial_state));
  return states_v;
}

unsigned int total_energy(const std::vector<state>& axes) {
  auto bodies = axes.at(0).size();
  std::vector<unsigned int> potential_energies(bodies);
  std::vector<unsigned int> kinetic_energies(bodies);
  for (auto axis : axes) {
    for (auto body = 0lu; body < bodies; body++) {
      auto [p, v] = axis.at(body);
      potential_energies.at(body) += std::abs(p);
      kinetic_energies.at(body) += std::abs(v);
    }
  }
  unsigned int total_energy = 0;
  for (auto body = 0lu; body < bodies; body++) {
    total_energy += potential_energies.at(body) * kinetic_energies.at(body);
  }
  return total_energy;
}

long unsigned int gcd(const long unsigned int& a, const long unsigned int& b) {
  if (b > a) {
    return gcd(b, a);
  }
  if (b == 0) {
    return a;
  }
  return gcd(b, a % b);
}

long unsigned int lcm(const long unsigned int& a, const long unsigned int& b) {
  return (a / gcd(a, b)) * b;
}

long unsigned int lcm_all(const std::vector<long unsigned int>& ns) {
  auto acc = ns.front();
  for (auto it = ns.begin() + 1; it != ns.end(); it++) {
    acc = lcm(acc, *it);
  }
  return acc;
}
