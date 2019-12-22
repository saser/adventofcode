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
  std::map<point_t, point_t> warps;
  point_t start;
  point_t end;

  maze_t(const grid_t& _grid) : grid(_grid), warps(), start(), end() {}

  void process_grid();
  unsigned int start_to_end() const;
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
  auto steps = maze.start_to_end();
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
      } else if (is_alpha(down)) {
        next = down;
        if (row_i == 0 || grid.at(row_i - 1).at(col_i) == ' ') {
          portal.entrance = {row_i + 1, col_i};
          portal.in_maze = {row_i + 2, col_i};
        } else {
          portal.entrance = {row_i, col_i};
          portal.in_maze = {row_i - 1, col_i};
        }
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
      auto out1 = portals[1].in_maze;
      warps[in1] = out1;
      auto in2 = portals[1].entrance;
      auto out2 = portals[0].in_maze;
      warps[in2] = out2;
    }
  }
}

unsigned int maze_t::start_to_end() const {
  std::map<point_t, unsigned int> distances;
  std::deque<std::pair<point_t, unsigned int>> q;
  q.push_back({start, 0});
  while (!q.empty()) {
    auto [p, distance] = q.front();
    if (p == end) {
      return distance;
    }
    q.pop_front();
    auto [row, col] = p;
    if (grid.at(row).at(col) != '.') {
      continue;
    }
    if (distances.find(p) != distances.end()) {
      continue;
    }
    distances[p] = distance;
    std::vector<point_t> neighbors;
    neighbors.push_back({row - 1, col});
    neighbors.push_back({row + 1, col});
    neighbors.push_back({row, col - 1});
    neighbors.push_back({row, col + 1});
    for (auto neighbor : neighbors) {
      auto warp = warps.find(neighbor);
      if (warp != warps.end()) {
        neighbor = warp->second;
      }
      q.push_back({neighbor, distance + 1});
    }
  }
  return 0;
}
