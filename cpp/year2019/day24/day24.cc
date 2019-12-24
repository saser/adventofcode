#include "year2019/day24/day24.h"

#include <istream>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "adventofcode.h"

using grid_t = std::vector<std::vector<bool>>;
using row_i_t = grid_t::size_type;
using col_i_t = grid_t::value_type::size_type;
using point_t = std::pair<row_i_t, col_i_t>;

adventofcode::answer_t solve(std::istream& is, int part);
grid_t parse(std::istream& is);

grid_t empty_grid();
grid_t unit_grid();
unsigned int neighbors(const point_t& point,
                       const grid_t& outer,
                       const grid_t& grid,
                       const grid_t& inner);
grid_t step(const grid_t& grid);
grid_t step_recursive(const grid_t& outer, const grid_t& grid, const grid_t& inner);

unsigned int biodiversity_rating(const grid_t& grid);

namespace day24 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  auto grid = parse(is);
  std::set<unsigned int> seen_ratings;
  auto rating = biodiversity_rating(grid);
  while (seen_ratings.find(rating) == seen_ratings.end()) {
    seen_ratings.insert(rating);
    grid = step(grid);
    rating = biodiversity_rating(grid);
  }
  return adventofcode::ok(std::to_string(rating));
}

grid_t parse(std::istream& is) {
  grid_t grid;
  std::string line;
  while (std::getline(is, line)) {
    std::vector<bool> row;
    for (auto c : line) {
      bool b;
      if (c == '#') {
        b = true;
      } else {
        b = false;
      }
      row.push_back(b);
    }
    grid.push_back(row);
  }
  return grid;
}

unsigned int neighbors_vertical(const point_t& point,
                                const grid_t& outer,
                                const grid_t& grid,
                                const grid_t& inner,
                                bool up) {
  unsigned int n = 0;
  row_i_t outer_edge_row;
  row_i_t outer_row;
  row_i_t inner_edge_row;
  row_i_t inner_row;
  if (up) {
    outer_edge_row = 0;
    outer_row = 1;
    inner_edge_row = 3;
    inner_row = 4;
  } else {
    outer_edge_row = 4;
    outer_row = 3;
    inner_edge_row = 1;
    inner_row = 0;
  }
  auto [row_i, col_i] = point;
  if (row_i == outer_edge_row) {
    n += outer.at(outer_row).at(2);
  } else if (row_i == inner_edge_row && col_i == 2) {
    for (auto b : inner.at(inner_row)) {
      n += b;
    }
  } else {
    row_i_t grid_row = up ? row_i - 1 : row_i + 1;
    n += grid.at(grid_row).at(col_i);
  }
  return n;
}

unsigned int neighbors_horizontal(const point_t& point,
                                  const grid_t& outer,
                                  const grid_t& grid,
                                  const grid_t& inner,
                                  bool left) {
  unsigned int n = 0;
  col_i_t outer_edge_col;
  col_i_t outer_col;
  col_i_t inner_edge_col;
  col_i_t inner_col;
  if (left) {
    outer_edge_col = 0;
    outer_col = 1;
    inner_edge_col = 3;
    inner_col = 4;
  } else {
    outer_edge_col = 4;
    outer_col = 3;
    inner_edge_col = 1;
    inner_col = 0;
  }
  auto [row_i, col_i] = point;
  if (col_i == outer_edge_col) {
    n += outer.at(2).at(outer_col);
  } else if (col_i == inner_edge_col && row_i == 2) {
    for (row_i_t inner_row = 0; inner_row < 5; inner_row++) {
      n += inner.at(inner_row).at(inner_col);
    }
  } else {
    col_i_t grid_col = left ? col_i - 1 : col_i + 1;
    n += grid.at(row_i).at(grid_col);
  }
  return n;
}

grid_t empty_grid() {
  return grid_t(5, std::vector<bool>(5, false));
}

grid_t unit_grid() {
  auto grid = empty_grid();
  std::vector<point_t> bugs {
    {0, 2},
    {4, 2},
    {2, 0},
    {2, 4},
  };
  for (auto [row_i, col_i] : bugs) {
    grid.at(row_i).at(col_i) = true;
  }
  return grid;
}

unsigned int neighbors_up(const point_t& point,
                          const grid_t& outer,
                          const grid_t& grid,
                          const grid_t& inner) {
  return neighbors_vertical(point, outer, grid, inner, true);
}

unsigned int neighbors_down(const point_t& point,
                            const grid_t& outer,
                            const grid_t& grid,
                            const grid_t& inner) {
  return neighbors_vertical(point, outer, grid, inner, false);
}

unsigned int neighbors_left(const point_t& point,
                            const grid_t& outer,
                            const grid_t& grid,
                            const grid_t& inner) {
  return neighbors_horizontal(point, outer, grid, inner, true);
}

unsigned int neighbors_right(const point_t& point,
                             const grid_t& outer,
                             const grid_t& grid,
                             const grid_t& inner) {
  return neighbors_horizontal(point, outer, grid, inner, false);
}

unsigned int neighbors(const point_t& point,
                       const grid_t& outer,
                       const grid_t& grid,
                       const grid_t& inner) {
  unsigned int n = 0;
  n += neighbors_up(point, outer, grid, inner);
  n += neighbors_down(point, outer, grid, inner);
  n += neighbors_left(point, outer, grid, inner);
  n += neighbors_right(point, outer, grid, inner);
  return n;
}

grid_t step(const grid_t& grid) {
  grid_t inner;
  if (grid.at(2).at(2)) {
    inner = unit_grid();
  } else {
    inner = empty_grid();
  }
  return step_recursive(empty_grid(), grid, inner);
}

grid_t step_recursive(const grid_t& outer, const grid_t& grid, const grid_t& inner) {
  auto copy = grid;
  for (row_i_t row_i = 0; row_i < 5; row_i++) {
    for (col_i_t col_i = 0; col_i < 5; col_i++) {
      auto surrounding = neighbors({row_i, col_i}, outer, grid, inner);
      auto is_bug = grid.at(row_i).at(col_i);
      auto new_is_bug = copy.at(row_i).at(col_i);
      if (is_bug && surrounding != 1) {
        new_is_bug = false;
      } else if (!is_bug && (surrounding == 1 || surrounding == 2)) {
        new_is_bug = true;
      }
    }
  }
  return copy;
}

unsigned int biodiversity_rating(const grid_t& grid) {
  auto rating = 0;
  for (auto row_it = grid.crbegin(); row_it != grid.crend(); row_it++) {
    auto row = *row_it;
    for (auto is_bug_it = row.crbegin(); is_bug_it != row.crend(); is_bug_it++) {
      rating <<= 1;
      if (*is_bug_it) {
        rating++;
      }
    }
  }
  return rating;
}
