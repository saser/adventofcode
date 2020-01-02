#include "cpp/year2019/day11/day11.h"

#include <algorithm>
#include <iostream>
#include <sstream>
#include <string>
#include <unordered_map>

#include "cpp/adventofcode.h"
#include "cpp/year2019/intcode/intcode.h"

adventofcode::answer_t solve(std::istream& is, int part);

struct point {
  int x;
  int y;

  bool operator==(const point& other) const;
};

struct robot {
  point p;
  int dx;
  int dy;

  robot() : p({x: 0, y: 0}), dx(0), dy(1) {}

  void step();
  void turn_left();
  void turn_right();
};

namespace std {
  template<>
  struct hash<point> {
    size_t operator()(const point& p) const {
      return p.x + 10 * p.y;
    }
  };
}

std::vector<std::vector<char>> to_grid(const std::unordered_map<point, bool>& painted);
std::string render(const std::vector<std::vector<char>>& grid);

namespace day11 {
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
  robot r {};
  std::unordered_map<point, bool> painted;
  if (part == 2) {
    painted[r.p] = true;
  }
  while (e.state != intcode::execution_state::halted) {
    auto color = false;
    auto lookup = painted.find(r.p);
    if (lookup != painted.end()) {
      color = lookup->second;
    }
    e.write(color ? 1 : 0);
    e.run();
    auto paint_color = e.read();
    auto turn = e.read();
    painted[r.p] = paint_color == 1;
    if (turn == 0) {
      r.turn_left();
    } else {
      r.turn_right();
    }
    r.step();
  }
  if (part == 1) {
    return adventofcode::ok(std::to_string(painted.size()));
  }
  auto grid = to_grid(painted);
  auto output = render(grid);
  return adventofcode::ok(output);
}

bool point::operator==(const point& other) const {
  return x == other.x && y == other.y;
}

void robot::step() {
  p.x += dx;
  p.y += dy;
}

void robot::turn_left() {
  if (dx == 0 && dy == 1) { // up -> left
    dx = -1;
    dy = 0;
  } else if (dx == -1 && dy == 0) { // left -> down
    dx = 0;
    dy = -1;
  } else if (dx == 0 && dy == -1) { // down -> right
    dx = 1;
    dy = 0;
  } else { // right -> up
    dx = 0;
    dy = 1;
  }
}

void robot::turn_right() {
  if (dx == 0 && dy == 1) { // up -> right
    dx = 1;
    dy = 0;
  } else if (dx == 1 && dy == 0) { // right -> down
    dx = 0;
    dy = -1;
  } else if (dx == 0 && dy == -1) { // down -> left
    dx = -1;
    dy = 0;
  } else { // left -> up
    dx = 0;
    dy = 1;
  }
}

std::vector<std::vector<char>> to_grid(const std::unordered_map<point, bool>& painted) {
  using pair_type = std::unordered_map<point, bool>::value_type;
  auto [ min_x_pair, max_x_pair ] = std::minmax_element(
    painted.begin(), painted.end(),
    [] (const pair_type& pair1, const pair_type& pair2) {
      return pair1.first.x < pair2.first.x;
    }
  );
  auto min_x = min_x_pair->first.x;
  auto max_x = max_x_pair->first.x;
  auto cols = max_x - min_x + 1;
  auto [ min_y_pair, max_y_pair ] = std::minmax_element(
    painted.begin(), painted.end(),
    [] (const pair_type& pair1, const pair_type& pair2) {
      return pair1.first.y < pair2.first.y;
    }
  );
  auto min_y = min_y_pair->first.y;
  auto max_y = max_y_pair->first.y;
  auto rows = max_y - min_y + 1;
  std::vector<std::vector<char>> grid(rows, std::vector<char>(cols, '.'));
  for (auto [ p, color ] : painted) {
    grid[p.y - min_y][p.x - min_x] = color ? '#' : '.';
  }
  return grid;
}

std::string render(const std::vector<std::vector<char>>& grid) {
  std::stringbuf sb;
  std::ostream os {&sb};
  for (auto it = grid.crbegin(); it != grid.crend(); it++) {
    auto row = *it;
    for (auto c : row) {
      os << c;
    }
    os << std::endl;
  }
  return sb.str();
}
