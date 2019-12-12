#include "year2019/day12/day12.h"

#include <cmath>
#include <iostream>
#include <regex>
#include <string>
#include <vector>

#include "absl/strings/str_format.h"

#include "adventofcode.h"

struct point {
  int x;
  int y;
  int z;

  int energy() const;
};

struct moon {
  point position;
  point velocity;
};

adventofcode::answer_t solve(std::istream& is, unsigned int steps, int part);
std::vector<moon> parse(std::istream& is);
std::string format_moon(const moon& moon);
void apply_gravity_axis(const int& p1, const int& p2, int& v1, int& v2);
void apply_gravity(std::vector<moon>& moons);
void apply_velocity(std::vector<moon>& moons);
void n_body_step(std::vector<moon>& moons);
int potential_energy(const moon& moon);
int kinetic_energy(const moon& moon);
int total_energy(const std::vector<moon>& moons);

namespace day12 {
  adventofcode::answer_t part1(std::istream& is, unsigned int steps) {
    return solve(is, steps, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 0, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, unsigned int steps, int part) {
  auto moons = parse(is);
  for (unsigned int i = 0; i < steps; i++) {
    n_body_step(moons);
  }
  auto energy = total_energy(moons);
  return adventofcode::ok(std::to_string(energy));
}

std::vector<moon> parse(std::istream& is) {
  std::string line;
  std::vector<moon> moons;
  const std::regex re(R"(<x=(-?\d+), y=(-?\d+), z=(-?\d+)>)");
  std::smatch match;
  while (std::getline(is, line)) {
    if (!std::regex_match(line, match, re)) {
      continue;
    }
    auto x_match = match[1];
    auto x = std::stoi(x_match);
    auto y_match = match[2];
    auto y = std::stoi(y_match);
    auto z_match = match[3];
    auto z = std::stoi(z_match);
    point position {x: x, y: y, z: z};
    point velocity {x: 0, y: 0, z: 0};
    moon m {position: position, velocity: velocity};
    moons.push_back(m);
  }
  return moons;
}

std::string format_moon(const moon& moon) {
  auto p = moon.position;
  auto v = moon.velocity;
  return absl::StrFormat("pos=<x=% 3d, y=% 3d, z=% 3d>, vel=<x=% 3d, y=% 3d, z=% 3d>", p.x, p.y, p.z, v.x, v.y, v.z);
}

int point::energy() const {
  return std::abs(x) + std::abs(y) + std::abs(z);
}

void n_body_step(std::vector<moon>& moons) {
  apply_gravity(moons);
  apply_velocity(moons);
}

void apply_gravity_axis(const int& p1, const int& p2, int& v1, int& v2) {
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

void apply_gravity(std::vector<moon>& moons) {
  using size = std::vector<moon>::size_type;
  for (size i = 0; i < moons.size(); i++) {
    for (size j = i + 1; j < moons.size(); j++) {
      auto& moon1 = moons.at(i);
      auto& moon2 = moons.at(j);
      apply_gravity_axis(moon1.position.x, moon2.position.x, moon1.velocity.x, moon2.velocity.x);
      apply_gravity_axis(moon1.position.y, moon2.position.y, moon1.velocity.y, moon2.velocity.y);
      apply_gravity_axis(moon1.position.z, moon2.position.z, moon1.velocity.z, moon2.velocity.z);
    }
  }
}

void apply_velocity(std::vector<moon>& moons) {
  for (auto& moon : moons) {
    moon.position.x += moon.velocity.x;
    moon.position.y += moon.velocity.y;
    moon.position.z += moon.velocity.z;
  }
}

int potential_energy(const moon& moon) {
  return moon.position.energy();
}

int kinetic_energy(const moon& moon) {
  return moon.velocity.energy();
}

int total_energy(const std::vector<moon>& moons) {
  int sum = 0;
  for (auto& moon : moons) {
    sum += potential_energy(moon) * kinetic_energy(moon);
  }
  return sum;
}
