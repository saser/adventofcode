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
std::vector<bool> neighbors(const point_t& point, const grid_t& grid);
grid_t step(const grid_t& grid);
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

std::vector<bool> neighbors(const point_t& point, const grid_t& grid) {
  auto n_rows = grid.size();
  auto n_cols = grid.at(0).size();
  auto [row_i, col_i] = point;
  std::vector<bool> n;
  if (row_i != 0) {
    n.push_back(grid.at(row_i - 1).at(col_i));
  }
  if (row_i != n_rows - 1) {
    n.push_back(grid.at(row_i + 1).at(col_i));
  }
  if (col_i != 0) {
    n.push_back(grid.at(row_i).at(col_i - 1));
  }
  if (col_i != n_cols - 1) {
    n.push_back(grid.at(row_i).at(col_i + 1));
  }
  return n;
}

grid_t step(const grid_t& grid) {
  auto copy = grid;
  for (row_i_t row_i = 0; row_i < grid.size(); row_i++) {
    for (col_i_t col_i = 0; col_i < grid.at(row_i).size(); col_i++) {
      auto count = 0;
      for (auto neighbor : neighbors({row_i, col_i}, grid)) {
        if (neighbor) {
          count++;
        }
      }
      auto is_bug = grid.at(row_i).at(col_i);
      auto new_is_bug = copy.at(row_i).at(col_i);
      if (is_bug && count != 1) {
        new_is_bug = false;
      } else if (!is_bug && (count == 1 || count == 2)) {
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
