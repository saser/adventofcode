#include "year2019/day15/day15.h"

#include <algorithm>
#include <iostream>
#include <deque>
#include <string>
#include <unordered_map>
#include <utility>

#include "adventofcode.h"
#include "year2019/intcode/intcode.h"

struct point {
  int x;
  int y;

  bool operator==(const point& p) const {
    return x == p.x && y == p.y;
  }

  point step(int64_t direction) const {
    int dx = 0;
    int dy = 0;
    switch (direction) {
    case 1:
      dy = 1;
      break;
    case 2:
      dy = -1;
      break;
    case 3:
      dx = -1;
      break;
    case 4:
      dx = 1;
      break;
    }
    return point {x + dx, y + dy};
  }
};

namespace std {
  template<>
  struct hash<point> {
    size_t operator()(const point& p) const {
      return p.x + 10 * p.y;
    }
  };
}

using world_t = std::unordered_map<point, char>;

struct droid {
  intcode::execution e;
  world_t world;
  point position;

  droid (const intcode::execution& _e) : e(_e), world(), position() {}

  void explore(int64_t direction);
  void explore_fully();
  point find_oxygen() const;
  std::unordered_map<point, unsigned int> distances_from(const point& start) const;
  void draw() const;
};

adventofcode::answer_t solve(std::istream& is, int part);

namespace day15 {
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
  intcode::memory program = intcode::parse(input);
  intcode::execution e {program};
  droid d {e};
  d.explore_fully();
  auto oxygen = d.find_oxygen();
  auto distances = d.distances_from(oxygen);
  if (part == 1) {
    return adventofcode::ok(std::to_string(distances[point {0, 0}]));
  }
  unsigned int max_distance = 0;
  for (auto [_, distance] : distances) {
    max_distance = std::max(max_distance, distance);
  }
  return adventofcode::ok(std::to_string(max_distance));
}

void droid::explore(int64_t direction) {
  auto new_position = position.step(direction);
  if (world.find(new_position) != world.end()) {
    return;
  }
  e.write(direction);
  e.run();
  auto reply = e.read();
  if (reply == 0) {
    world[new_position] = '#';
    return;
  }
  position = new_position;
  world[position] = reply == 2 ? 'O' : '.';
  int64_t backtrack;
  switch (direction) {
  case 1:
    backtrack = 2;
    break;
  case 2:
    backtrack = 1;
    break;
  case 3:
    backtrack = 4;
    break;
  case 4:
    backtrack = 3;
    break;
  }
  for (auto new_direction : {1, 2, 3, 4}) {
    if (new_direction == backtrack) {
      continue;
    }
    explore(new_direction);
  }
  e.write(backtrack);
  e.run();
  e.read();
  position = position.step(backtrack);
}

void droid::explore_fully() {
  world[point {0, 0}] = '.';
  for (auto direction : {1, 2, 3, 4}) {
    explore(direction);
  }
}

point droid::find_oxygen() const {
  for (auto [p, c] : world) {
    if (c == 'O') {
      return p;
    }
  }
  return point {0, 0};
}

std::unordered_map<point, unsigned int> droid::distances_from(const point& start) const {
  std::unordered_map<point, unsigned int> distances;
  std::deque<std::pair<point, unsigned int>> q;
  q.push_back({start, 0});
  while (!q.empty()) {
    auto [p, distance] = q.front();
    q.pop_front();
    if (distances.find(p) != distances.end()) {
      continue;
    }
    distances[p] = distance;
    for (auto direction : {1, 2, 3, 4}) {
      auto new_p = p.step(direction);
      if (world.at(new_p) == '#') {
        continue;
      }
      q.push_back({new_p, distance + 1});
    }
  }
  return distances;
}

void droid::draw() const {
  auto [min_x_pair, max_x_pair] = std::minmax_element(
    world.begin(), world.end(),
    [] (const auto& pair1, const auto& pair2) {
    return pair1.first.x < pair2.first.x;
    }
  );
  auto min_x = min_x_pair->first.x;
  auto max_x = max_x_pair->first.x;
  auto [min_y_pair, max_y_pair] = std::minmax_element(
    world.begin(), world.end(),
    [] (const auto& pair1, const auto& pair2) {
    return pair1.first.y < pair2.first.y;
    }
  );
  auto min_y = min_y_pair->first.y;
  auto max_y = max_y_pair->first.y;
  auto rows = max_y - min_y + 1;
  auto cols = max_x - min_x + 1;
  std::vector<std::vector<char>> grid(rows, std::vector<char>(cols, ' '));
  for (auto [p, c] : world) {
    if (p == position) {
      c = 'D';
    }
    grid[p.y - min_y][p.x - min_x] = c;
  }
  for (auto it = grid.crbegin(); it != grid.crend(); it++) {
    for (auto col : *it) {
      std::cout << col;
    }
    std::cout << std::endl;
  }
}
