#include "year2019/day20/day20.h"

#include <istream>
#include <map>
#include <deque>
#include <string>
#include <utility>
#include <vector>

#include "adventofcode.h"

using grid_t = std::vector<std::vector<char>>;
using point_t = std::pair<grid_t::size_type, grid_t::value_type::size_type>;

struct maze_t {
  grid_t grid;
  std::map<point_t, std::tuple<point_t, bool>> warps;
  point_t start;
  point_t end;

  maze_t(const grid_t& _grid) : grid(_grid), warps(), start(), end() {}

  void process_grid();
  unsigned int start_to_end(bool recursive) const;
};

adventofcode::answer_t solve(std::istream& is, int part);
grid_t read_grid(std::istream& is);

namespace day20 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  auto grid = read_grid(is);
  maze_t maze {grid};
  maze.process_grid();
  auto steps = maze.start_to_end(part == 2);
  return adventofcode::ok(std::to_string(steps));
}

grid_t read_grid(std::istream& is) {
  grid_t grid;
  std::vector<char> row;
  std::string line;
  while (std::getline(is, line)) {
    row.reserve(line.length());
    for (auto c : line) {
      row.push_back(c);
    }
    grid.push_back(row);
    row.clear();
  }
  return grid;
}

bool is_alpha(char c) {
  return c >= 'A' && c <= 'Z';
}

void maze_t::process_grid() {
  struct portal_t {
    point_t entrance;
    point_t in_maze;
    bool is_outer;
  };
  std::map<std::string, std::vector<portal_t>> portals_by;
  for (grid_t::size_type row_i = 0; row_i < grid.size() - 1; row_i++) {
    auto row = grid.at(row_i);
    for (grid_t::value_type::size_type col_i = 0; col_i < row.size() - 1; col_i++) {
      auto base = row.at(col_i);
      if (!is_alpha(base)) {
        continue;
      }
      portal_t portal;
      char next;
      auto right = row.at(col_i + 1);
      auto down = grid.at(row_i + 1).at(col_i);
      if (is_alpha(right)) {
        next = right;
        if (col_i == 0 || row.at(col_i - 1) == ' ') {
          portal.entrance = {row_i, col_i + 1};
          portal.in_maze = {row_i, col_i + 2};
        } else {
          portal.entrance = {row_i, col_i};
          portal.in_maze = {row_i, col_i - 1};
        }
        portal.is_outer = col_i == 0 || col_i == row.size() - 2;
      } else if (is_alpha(down)) {
        next = down;
        if (row_i == 0 || grid.at(row_i - 1).at(col_i) == ' ') {
          portal.entrance = {row_i + 1, col_i};
          portal.in_maze = {row_i + 2, col_i};
        } else {
          portal.entrance = {row_i, col_i};
          portal.in_maze = {row_i - 1, col_i};
        }
        portal.is_outer = row_i == 0 || row_i == grid.size() - 2;
      } else {
        continue;
      }
      std::string name {base, next};
      portals_by[name].push_back(portal);
    }
  }
  for (auto [name, portals] : portals_by) {
    if (name == "AA") {
      start = portals[0].in_maze;
    } else if (name == "ZZ") {
      end = portals[0].in_maze;
    } else {
      auto in1 = portals[0].entrance;
      auto in1_is_outer = portals[0].is_outer;
      auto out1 = portals[1].in_maze;
      warps[in1] = {out1, in1_is_outer};
      auto in2 = portals[1].entrance;
      auto in2_is_outer = portals[1].is_outer;
      auto out2 = portals[0].in_maze;
      warps[in2] = {out2, in2_is_outer};
    }
  }
}

unsigned int maze_t::start_to_end(bool recursive) const {
  std::map<unsigned int, std::map<point_t, unsigned int>> distances;
  std::deque<std::tuple<point_t, unsigned int, unsigned int>> q;
  q.push_back({start, 0, 0});
  while (!q.empty()) {
    auto [p, distance, level] = q.front();
    q.pop_front();
    if (p == end && level == 0) {
      return distance;
    }
    auto [row, col] = p;
    if (grid.at(row).at(col) != '.') {
      continue;
    }
    auto& level_distances = distances[level];
    if (level_distances.find(p) != level_distances.end()) {
      continue;
    }
    level_distances[p] = distance;
    std::vector<point_t> neighbors;
    neighbors.push_back({row - 1, col});
    neighbors.push_back({row + 1, col});
    neighbors.push_back({row, col - 1});
    neighbors.push_back({row, col + 1});
    for (auto neighbor : neighbors) {
      auto new_level = level;
      auto search = warps.find(neighbor);
      if (search != warps.end()) {
        auto [_, tuple] = *search;
        auto [out, is_outer] = tuple;
        if (recursive) {
          if (level == 0 && is_outer) {
            continue;
          }
          if (is_outer) {
            new_level--;
          } else {
            new_level++;
          }
        }
        neighbor = out;
      }
      q.push_back({neighbor, distance + 1, new_level});
    }
  }
  return 0;
}
