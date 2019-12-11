#include "year2019/day11/day11.h"

#include <istream>
#include <string>
#include <unordered_map>

#include "adventofcode.h"
#include "year2019/intcode/intcode.h"

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
  return adventofcode::ok(std::to_string(painted.size()));
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
