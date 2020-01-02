#include "cpp/year2019/day17/day17.h"

#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "cpp/adventofcode.h"
#include "cpp/year2019/intcode/intcode.h"

using grid_t = std::vector<std::vector<char>>;

adventofcode::answer_t solve(std::istream& is, int part);
grid_t parse_grid(const intcode::output& output);
std::string render_grid(const grid_t& grid);
unsigned int sum_alignment_params(const grid_t& grid);
std::vector<char> neighbors(int row, int col, const grid_t& grid);
char safe_at(int row, int col, const grid_t& grid);

namespace day17 {
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
  if (part == 2) {
    program[0] = 2;
  }
  intcode::execution e {program};
  if (part == 1) {
    e.run();
    auto grid = parse_grid(e.read_all());
    return adventofcode::ok(std::to_string(sum_alignment_params(grid)));
  }
  // This program was found by hand by me.
  std::string instructions =
    "A,B,A,C,A,C,B,C,C,B\n"
    "L,4,L,4,L,10,R,4\n"
    "R,4,L,4,L,4,R,8,R,10\n"
    "R,4,L,10,R,10\n"
    "n\n";
  for (auto c : instructions) {
    e.write(c);
  }
  e.run();
  auto output = e.read_all();
  auto dust = output.back();
  return adventofcode::ok(std::to_string(dust));
}

grid_t parse_grid(const intcode::output& output) {
  grid_t grid;
  std::vector<char> row;
  for (auto o : output) {
    if (o == 10) {
      grid.push_back(row);
      row.clear();
      continue;
    }
    row.push_back((char) o);
  }
  return grid;
}

std::string render_grid(const grid_t& grid) {
  std::stringbuf sb;
  std::ostream os {&sb};
  for (auto row : grid) {
    for (auto c : row) {
      os << c;
    }
    os << std::endl;
  }
  return sb.str();
}

unsigned int sum_alignment_params(const grid_t& grid) {
  auto sum = 0;
  for (grid_t::size_type row_i = 0; row_i < grid.size(); row_i++) {
    auto row = grid.at(row_i);;
    for (grid_t::value_type::size_type col_i = 0; col_i < row.size(); col_i++) {
      auto c = grid.at(row_i).at(col_i);
      if (c == '.') {
        continue;
      }
      auto surrounding = 0;
      for (auto neighbor : neighbors(row_i, col_i, grid)) {
        if (neighbor != '.') {
          surrounding++;
        }
      }
      if (surrounding == 4) {
        sum += row_i * col_i;
      }
    }
  }
  return sum;
}

std::vector<char> neighbors(int row, int col, const grid_t& grid) {
  std::vector<char> v;
  v.push_back(safe_at(row + 1, col, grid));
  v.push_back(safe_at(row - 1, col, grid));
  v.push_back(safe_at(row, col + 1, grid));
  v.push_back(safe_at(row, col - 1, grid));
  return v;
}

char safe_at(int row, int col, const grid_t& grid) {
  if (row < 0 || row >= (int) grid.size() || col < 0 || col >= (int) grid.at(row).size()) {
    return '.';
  }
  return grid.at(row).at(col);
}
