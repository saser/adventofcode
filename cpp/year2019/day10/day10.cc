#include "cpp/year2019/day10/day10.h"

#include <algorithm>
#include <cmath>
#include <istream>
#include <map>
#include <set>
#include <string>
#include <unordered_set>
#include <vector>

#include "cpp/adventofcode.h"

struct point {
  int x;
  int y;

  bool operator==(const point& p) const {
    return x == p.x && y == p.y;
  }

  unsigned distance_to(const point& other) const;
  point vector_to(const point& other) const;
  point canonical_vector_to(const point& other) const;
};

struct angle_compare {
  bool operator()(const point& cv1, const point& cv2) const {
    const double pi = std::acos(-1);
    auto laser = std::atan2(-1.0, 0.0);
    auto angle1 = std::atan2(cv1.y, cv1.x) - laser;
    if (angle1 < 0) {
      angle1 += 2 * pi;
    }
    auto angle2 = std::atan2(cv2.y, cv2.x) - laser;
    if (angle2 < 0) {
      angle2 += 2 * pi;
    }
    return angle1 < angle2;
  }
};

struct dist_compare {
  point base;

  bool operator()(const point& p1, const point& p2) const {
    return base.distance_to(p1) < base.distance_to(p2);
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
  // Parse the asteroids into a collection of points.
  std::vector<point> asteroids;
  asteroids.reserve(n_rows * n_cols);
  for (int row_i = 0; (unsigned int) row_i < n_rows; row_i++) {
    auto row = lines[row_i];
    for (int col_i = 0; (unsigned int) col_i < n_cols; col_i++) {
      if (row[col_i] == '#') {
        asteroids.push_back(point {x: col_i, y: row_i});
      }
    }
  }
  // Count the number of visible asteroids for each asteroid, keeping track of
  // which asteroid can see the most other asteroids.
  std::unordered_set<point>::size_type max_visible = 0;
  point base = asteroids[0];
  for (auto asteroid : asteroids) {
    std::unordered_set<point> canonical_vectors;
    for (auto other : asteroids) {
      if (asteroid == other) {
        continue;
      }
      canonical_vectors.insert(asteroid.canonical_vector_to(other));
    }
    auto size = canonical_vectors.size();
    if (size > max_visible) {
      max_visible = size;
      base = asteroid;
    }
  }
  if (part == 1) {
    return adventofcode::ok(std::to_string(max_visible));
  }
  // Group all other asteroids by their canonical vector from `base`. The map
  // will sort the canonical vectors in ascending order based on their angle
  // from the laser mounted at `base`. The set of asteroids for each canonical
  // vector will be sorted in ascending order based on their distance from
  // `base`.
  std::map<point, std::set<point, dist_compare>, angle_compare> destroyed;
  for (auto other : asteroids) {
    if (other == base) {
      continue;
    }
    auto cv = base.canonical_vector_to(other);
    if (destroyed.find(cv) == destroyed.end()) {
      destroyed[cv] = std::set<point, dist_compare>(dist_compare {base: base});
    }
    destroyed[cv].insert(other);
  }
  // Begin "destroying" asteroids, by destroying one asteroid at a time for all
  // the canonical vectors. If we reach the end of a full loop, begin from the
  // start again.
  auto it = destroyed.begin();
  auto i = 0;
  point target = asteroids[0];
  while (i != 200) {
    if (it == destroyed.end()) {
      it = destroyed.begin();
    }
    if (it->second.empty()) {
      it++;
      continue;
    }
    auto& set = it->second;
    auto set_it = set.begin();
    target = *set_it;
    set.erase(set_it);
    i++;
    it++;
  }
  auto coordinates = 100 * target.x + target.y;
  return adventofcode::ok(std::to_string(coordinates));
}

unsigned point::distance_to(const point& other) const {
  return std::abs(x - other.x) + std::abs(y - other.y);
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
