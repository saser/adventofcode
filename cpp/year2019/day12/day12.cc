#include "year2019/day12/day12.h"

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
};

struct moon {
  point position;
  point velocity;
};

adventofcode::answer_t solve(std::istream& is, int part);
std::vector<moon> parse(std::istream& is);
std::string format_moon(const moon& moon);

namespace day12 {
  adventofcode::answer_t part1(std::istream& is, unsigned int steps) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  auto moons = parse(is);
  for (auto moon : moons) {
    std::cout << format_moon(moon) << std::endl;
  }
  return adventofcode::err("not implemented yet");
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
