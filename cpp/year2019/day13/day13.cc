#include "year2019/day13/day13.h"

#include <istream>
#include <map>
#include <string>
#include <utility>

#include "adventofcode.h"
#include "year2019/intcode/intcode.h"

adventofcode::answer_t solve(std::istream& is, int part);

namespace day13 {
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
  auto program = intcode::parse(input);
  auto [_, output] = intcode::run(program, {});
  auto it = output.begin();
  std::map<std::pair<int64_t, int64_t>, int64_t> tiles;
  while (it != output.end()) {
    auto x = *it;
    it++;
    auto y = *it;
    it++;
    auto tile = *it;
    it++;
    tiles[{x, y}] = tile;
  }
  auto block_tiles = 0;
  for (auto [pos, tile] : tiles) {
    if (tile == 2) {
      block_tiles++;
    }
  }
  return adventofcode::ok(std::to_string(block_tiles));
}
