#include "year2019/day08/day08.h"

#include <istream>
#include <string>

#include "adventofcode.h"

adventofcode::answer_t solve(std::istream& is, int part);

namespace day08 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }

  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  auto width = 25;
  auto height = 6;
  auto layer_size = width * height;
  std::string line;
  std::getline(is, line);
  auto zeroes = 0;
  auto ones = 0;
  auto twos = 0;
  auto fewest_zeroes = layer_size;
  auto product = 0;
  for (std::string::size_type i = 0; i < line.length(); i++) {
    if (i != 0 && i % layer_size == 0) {
      if (zeroes < fewest_zeroes) {
        fewest_zeroes = zeroes;
        product = ones * twos;
      }
      zeroes = 0;
      ones = 0;
      twos = 0;
    }
    switch (line[i]) {
    case '0':
      zeroes++;
      break;
    case '1':
      ones++;
      break;
    case '2':
      twos++;
      break;
    }
  }
  return adventofcode::ok(std::to_string(product));
}
