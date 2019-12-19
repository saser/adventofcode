#include "year2019/day15/day15.h"

#include <algorithm>
#include <iostream>
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

adventofcode::answer_t solve(std::istream& is, int part);
std::pair<unsigned int, std::unordered_map<point, char>> find_oxygen(intcode::execution& e);
void draw(const std::unordered_map<point, char>& visited, const point& position);

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
  auto [steps, _] = find_oxygen(e);
  return adventofcode::ok(std::to_string(steps));
}

unsigned int explore(intcode::execution& e, int64_t direction, std::unordered_map<point, char>& visited, const point& position) {
  auto new_position = position.step(direction);
  e.write(direction);
  e.run();
  auto reply = e.read();
  if (reply == 0) {
    visited[new_position] = '#';
    return 0;
  }
  if (reply == 2) {
    visited[new_position] = 'X';
    return 1;
  }
  visited[new_position] = '.';
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
    auto next_position = new_position.step(new_direction);
    if (visited.find(next_position) != visited.end()) {
      continue;
    }
    auto steps = explore(e, new_direction, visited, new_position);
    if (steps > 0) {
      return 1 + steps;
    }
  }
  e.write(backtrack);
  e.run();
  e.read();
  return 0;
}

std::pair<unsigned int, std::unordered_map<point, char>> find_oxygen(intcode::execution& e) {
  unsigned int steps = 0;
  std::unordered_map<point, char> visited;
  point origin {0, 0};
  visited[origin] = '@';
  for (auto direction : {1, 2, 3, 4}) {
    auto explored_steps = explore(e, direction, visited, origin);
    if (explored_steps > 0) {
      steps = explored_steps;
      break;
    }
  }
  return {steps, visited};
}

void draw(const std::unordered_map<point, char>& visited, const point& position) {
  using pair_type = std::unordered_map<point, char>::value_type;
  auto [min_x_point, max_x_point] = std::minmax_element(
    visited.begin(), visited.end(),
    [] (const pair_type& pair1, const pair_type& pair2) {
      return pair1.first.x < pair2.first.x;
    }
  );
  auto min_x = min_x_point->first.x;
  auto max_x = max_x_point->first.x;
  auto cols = max_x - min_x + 1;
  auto [min_y_point, max_y_point] = std::minmax_element(
    visited.begin(), visited.end(),
    [] (const pair_type& pair1, const pair_type& pair2) {
      return pair1.first.y < pair2.first.y;
    }
  );
  auto min_y = min_y_point->first.y;
  auto max_y = max_y_point->first.y;
  auto rows = max_y - min_y + 1;
  std::vector<std::vector<char>> grid(rows, std::vector<char>(cols, ' '));
  for (auto [p, c] : visited) {
    if (p == position) {
      c = 'D';
    }
    grid[p.y - min_y][p.x - min_x] = c;
  }
  for (auto it = grid.crbegin(); it != grid.crend(); it++) {
    auto row = *it;
    for (auto col : row) {
      std::cout << col;
    }
    std::cout << std::endl;
  }
}
