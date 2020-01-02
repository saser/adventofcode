#include "cpp/year2019/day08/day08.h"

#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "cpp/adventofcode.h"

adventofcode::answer_t solve(std::istream& is, int part);
std::string render(const std::vector<std::vector<char>>& grid);

namespace day08 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  std::string line;
  std::getline(is, line);
  auto n_rows = 6;
  auto n_cols = 25;
  std::vector<std::vector<char>> grid(n_rows, std::vector<char>(n_cols, '\0'));
  auto layer_size = n_rows * n_cols;
  auto zeroes = 0;
  auto ones = 0;
  auto twos = 0;
  auto fewest_zeroes = layer_size;
  auto product = 0;
  for (std::string::size_type i = 0; i < line.length(); i += layer_size) {
    auto layer = line.substr(i, layer_size);
    for (auto row_i = 0; row_i < n_rows; row_i++) {
      auto row = layer.substr(row_i * n_cols, n_cols);
      for (auto col_i = 0; col_i < n_cols; col_i++) {
        auto c = row[col_i];
        char& color = grid[row_i][col_i];
        switch (c) {
        case '0':
        case '1':
          if (color == '\0') {
            color = c;
          }
          if (c == '0') {
            zeroes++;
          } else {
            ones++;
          }
          break;
        case '2':
          twos++;
          break;
        }
      }
    }
    if (zeroes < fewest_zeroes) {
      fewest_zeroes = zeroes;
      product = ones * twos;
    }
    zeroes = 0;
    ones = 0;
    twos = 0;
  }
  if (part == 1) {
    return adventofcode::ok(std::to_string(product));
  }
  return adventofcode::ok(render(grid));
}

std::string render(const std::vector<std::vector<char>>& grid) {
  std::stringbuf sb;
  std::ostream os(&sb);
  for (auto row : grid) {
    for (auto c : row) {
      if (c == '1') {
        c = '#';
      } else {
        c = '.';
      }
      os << c;
    }
    os << std::endl;
  }
  return sb.str();
}
