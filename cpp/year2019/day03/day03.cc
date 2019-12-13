#include "year2019/day03/day03.h"

#include <istream>
#include <unordered_map>

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

namespace day03 {
  std::vector<instruction> parse(const std::string line);
  adventofcode::answer_t solve(std::istream& is, int part);

  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
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

  adventofcode::answer_t solve(std::istream& is, int part) {
    std::string line1, line2;
    std::getline(is, line1);
    std::getline(is, line2);
    // Calculate points visited by the first wire.
    auto instructions1 = parse(line1);
    std::unordered_map<point, int> visited1;
    point p1 {x: 0, y: 0};
    int steps1 = 0;
    for (auto instruction : instructions1) {
      for (int i = 0; i < instruction.steps; i++) {
        p1.x += instruction.dx;
        p1.y += instruction.dy;
        steps1++;
        if (visited1.find(p1) == visited1.end()) {
          visited1.insert({p1, steps1});
        }
      }
    }
    // Calculate points visited by the second wire.
    auto instructions2 = parse(line2);
    std::unordered_map<point, int> visited2;
    point p2 {x: 0, y: 0};
    int steps2 = 0;
    for (auto instruction : instructions2) {
      for (int i = 0; i < instruction.steps; i++) {
        p2.x += instruction.dx;
        p2.y += instruction.dy;
        steps2++;
        if (visited2.find(p2) == visited2.end()) {
          visited2.insert({p2, steps2});
        }
      }
    }
    // Find intersection closest (by Manhattan distance) to the origin, as well as intersection the
    // fewest total number of steps away.
    int shortest_distance = -1;
    int fewest_steps = -1;
    for (auto &kv1 : visited1) {
      auto search = visited2.find(kv1.first);
      if (search != visited2.end()) {
        auto kv2 = *search;
        auto distance = kv1.first.manhattan_distance();
        if (shortest_distance == -1 || distance < shortest_distance) {
          shortest_distance = distance;
        }
        auto steps = kv1.second + kv2.second;
        if (fewest_steps == -1 || steps < fewest_steps) {
          fewest_steps = steps;
        }
      }
    }
    int answer;
    switch (part) {
    case 1:
      answer = shortest_distance;
      break;
    case 2:
      answer = fewest_steps;
      break;
    default:
      return adventofcode::err("invalid part");
    }
    return adventofcode::ok(std::to_string(answer));
  }
}

