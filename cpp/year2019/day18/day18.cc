#include "year2019/day18/day18.h"

#include <cctype>
#include <deque>
#include <istream>
#include <map>
#include <queue>
#include <set>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

#include "adventofcode.h"

using raw_grid_t = std::vector<std::vector<char>>;
using row_i_t = raw_grid_t::size_type;
using col_i_t = raw_grid_t::value_type::size_type;
using point_t = std::pair<row_i_t, col_i_t>;

struct grid_t {
  raw_grid_t g;

  const char& at(const point_t& point) const;
  point_t start() const;
  std::map<char, point_t> all_keys() const;
  std::set<point_t> neighbors(const point_t& point) const;
  std::map<char, unsigned int> adjacent_keys(const point_t& from, const std::set<char>& collected_keys) const;
  unsigned int collect_keys() const;
};

adventofcode::answer_t solve(std::istream& is, int part);
grid_t parse(std::istream& is);

bool is_wall(char c);
bool is_key(char c);
bool is_door(char c);

namespace day18 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  auto grid = parse(is);
  auto steps = grid.collect_keys();
  return adventofcode::ok(std::to_string(steps));
}

grid_t parse(std::istream& is) {
  raw_grid_t g;
  std::string line;
  while (std::getline(is, line)) {
    g.push_back(std::vector<char>(line.begin(), line.end()));
  }
  return grid_t {g};
}

bool is_wall(char c) {
  return c == '#';
}

bool is_key(char c) {
  return c >= 'a' && c <= 'z';
}

bool is_door(char c) {
  return c >= 'A' && c <= 'Z';
}

const char& grid_t::at(const point_t& point) const {
  auto [row_i, col_i] = point;
  return g.at(row_i).at(col_i);
}

point_t grid_t::start() const {
  for (row_i_t row_i = 0; row_i < g.size(); row_i++) {
    for (col_i_t col_i = 0; col_i < g.at(row_i).size(); col_i++) {
      if (g.at(row_i).at(col_i) == '@') {
        return {row_i, col_i};
      }
    }
  }
  return {0, 0};
}

std::map<char, point_t> grid_t::all_keys() const {
  std::map<char, point_t> k;
  for (row_i_t row_i = 0; row_i < g.size(); row_i++) {
    for (col_i_t col_i = 0; col_i < g.at(row_i).size(); col_i++) {
      if (auto c = g.at(row_i).at(col_i); is_key(c)) {
        k[c] = {row_i, col_i};
      }
    }
  }
  return k;
}

std::set<point_t> grid_t::neighbors(const point_t& point) const {
  std::set<point_t> n;
  auto [row_i, col_i] = point;
  if (row_i > 0) {
    n.insert({row_i - 1, col_i});
  }
  if (row_i < g.size()) {
    n.insert({row_i + 1, col_i});
  }
  if (col_i > 0) {
    n.insert({row_i, col_i - 1});
  }
  if (col_i < g.at(0).size()) {
    n.insert({row_i, col_i + 1});
  }
  return n;
}

std::map<char, unsigned int> grid_t::adjacent_keys(const point_t& from, const std::set<char>& collected_keys) const {
  std::map<char, unsigned int> adjacent;
  std::set<point_t> visited;
  std::deque<std::pair<point_t, unsigned int>> q;
  q.push_back({from, 0});
  while (!q.empty()) {
    auto [p, distance] = q.front();
    q.pop_front();
    auto c = at(p);
    if (is_wall(c)) {
      continue;
    }
    if (visited.find(p) != visited.end()) {
      continue;
    }
    visited.insert(p);
    if (is_door(c) && collected_keys.find(tolower(c)) == collected_keys.end()) {
      continue;
    }
    if (is_key(c) && collected_keys.find(c) == collected_keys.end()) {
      adjacent[c] = distance;
      continue;
    }
    for (auto neighbor : neighbors(p)) {
      q.push_back({neighbor, distance + 1});
    }
  }
  return adjacent;
}

unsigned int grid_t::collect_keys() const {
  using elem_t = std::tuple<point_t, unsigned int, std::set<char>>;
  auto compare = [](const elem_t& elem1, const elem_t& elem2) {
    auto [p1, d1, c1] = elem1;
    auto [p2, d2, c2] = elem2;
    return d2 < d1;
  };
  auto key_positions = all_keys();
  std::set<char> required;
  for (auto [key, _] : key_positions) {
    required.insert(key);
  }
  std::priority_queue<elem_t, std::vector<elem_t>, decltype(compare)> q(compare);
  q.push({start(), 0, {}});
  std::set<std::pair<std::set<char>, point_t>> visited;
  while (!q.empty()) {
    auto [point, distance, collected_keys] = q.top();
    q.pop();
    if (collected_keys == required) {
      return distance;
    }
    if (visited.find({collected_keys, point}) != visited.end()) {
      continue;
    }
    visited.insert({collected_keys, point});
    for (auto [next_key, next_distance] : adjacent_keys(point, collected_keys)) {
      auto next_collected_keys = collected_keys;
      next_collected_keys.insert(next_key);
      q.push({key_positions[next_key], distance + next_distance, next_collected_keys});
    }
  }
  return 0;
}
