#include "year2019/day03/day03.h"

#include <istream>
#include <unordered_set>

#include "absl/strings/str_split.h"

#include "adventofcode.h"

struct point {
  int x;
  int y;

  bool operator==(const point& p) const {
    return x == p.x && y == p.y;
  }

  int manhattan_distance() const {
    return std::abs(x) + std::abs(y);
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

struct instruction {
  int dx;
  int dy;
  int steps;
};

std::vector<instruction> parse(const std::string line);

namespace day03 {
  adventofcode::answer_t part1(std::istream& is) {
    std::string line1, line2;
    std::getline(is, line1);
    std::getline(is, line2);
    // Calculate points visited by the first wire.
    auto instructions1 = parse(line1);
    std::unordered_set<point> visited;
    point p1 {x: 0, y: 0};
    for (auto instruction : instructions1) {
      for (int i = 0; i < instruction.steps; i++) {
        p1.x += instruction.dx;
        p1.y += instruction.dy;
        visited.insert(p1);
      }
    }
    // Calculate points visited by the second wire.
    auto instructions2 = parse(line2);
    point p2 {x: 0, y: 0};
    int shortest_distance = 0;
    for (auto instruction : instructions2) {
      for (int i = 0; i < instruction.steps; i++) {
        p2.x += instruction.dx;
        p2.y += instruction.dy;
        if (visited.find(p2) != visited.end()) {
          auto distance = p2.manhattan_distance();
          if (shortest_distance == 0) {
            shortest_distance = distance;
          } else {
            shortest_distance = std::min(shortest_distance, p2.manhattan_distance());
          }
        }
      }
    }
    return adventofcode::ok(std::to_string(shortest_distance));
  }

  adventofcode::answer_t part2(std::istream& is) {
    return adventofcode::err("not implemented yet");
  }
}

std::vector<instruction> parse(const std::string line) {
  std::vector<std::string> parts = absl::StrSplit(line, ",");
  std::vector<instruction> instructions;
  for (auto part : parts) {
    int dx = 0;
    int dy = 0;
    switch (part[0]) {
    case 'U':
      dx = 0;
      dy = 1;
      break;
    case 'R':
      dx = 1;
      dy = 0;
      break;
    case 'D':
      dx = 0;
      dy = -1;
      break;
    case 'L':
      dx = -1;
      dy = 0;
      break;
    }
    int steps = std::stoi(part.substr(1));
    instruction i {dx: dx, dy: dy, steps: steps};
    instructions.push_back(i);
  }
  return instructions;
}
