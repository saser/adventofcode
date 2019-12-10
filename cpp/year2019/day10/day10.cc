#include "year2019/day10/day10.h"

#include <algorithm>
#include <istream>
#include <string>
#include <unordered_set>
#include <vector>

#include "adventofcode.h"

struct point {
  int x;
  int y;

  bool operator==(const point& p) const {
    return x == p.x && y == p.y;
  }

  point vector_to(const point& other) const;
  point canonical_vector_to(const point& other) const;
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
int gcd(int a, int b);

namespace day10 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  std::vector<std::string> lines;
  std::string line;
  unsigned n_cols = 0;
  while (std::getline(is, line)) {
    if (n_cols == 0) {
      n_cols = line.length();
    }
    lines.push_back(line);
  }
  unsigned n_rows = lines.size();
  std::unordered_set<point> asteroids;
  asteroids.reserve(n_rows * n_cols);
  for (int row_i = 0; (unsigned int) row_i < n_rows; row_i++) {
    auto row = lines[row_i];
    for (int col_i = 0; (unsigned int) col_i < n_cols; col_i++) {
      if (row[col_i] == '#') {
        asteroids.insert(point {x: col_i, y: row_i});
      }
    }
  }
  std::unordered_set<point>::size_type max_visible = 0;
  for (auto asteroid : asteroids) {
    std::unordered_set<point> canonical_vectors;
    for (auto other : asteroids) {
      if (asteroid == other) {
        continue;
      }
      canonical_vectors.insert(asteroid.canonical_vector_to(other));
    }
    max_visible = std::max(max_visible, canonical_vectors.size());
  }
  return adventofcode::ok(std::to_string(max_visible));
}

point point::vector_to(const point& other) const {
  return point {x: other.x - x, y: other.y - y};
}

point point::canonical_vector_to(const point& other) const {
  auto v = vector_to(other);
  auto d = gcd(std::abs(v.x), std::abs(v.y));
  return point {x: v.x / d, y: v.y / d};
}

int gcd(int a, int b) {
  if (b > a) {
    int temp = a;
    a = b;
    b = temp;
  }
  if (b == 0) {
    return a;
  }
  return gcd(b, a % b);
}
