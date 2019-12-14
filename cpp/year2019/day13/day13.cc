#include "year2019/day13/day13.h"

#include <istream>
#include <map>
#include <string>
#include <utility>
#include <vector>

#include "adventofcode.h"
#include "year2019/intcode/intcode.h"

using screen_t = std::vector<std::vector<int>>;

struct breakout {
  intcode::execution e;
  screen_t screen;
  int64_t score = 0;

  breakout(const intcode::memory& program, const screen_t& _screen) : e (program), screen (_screen) {}

  void read();
  void move_paddle();
  void run();
};

adventofcode::answer_t solve(std::istream& is, int part);
screen_t to_screen(const std::map<std::pair<int64_t, int64_t>, int64_t>& tiles);

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
    auto x = *it++;
    auto y = *it++;
    auto tile = *it++;
    tiles[{x, y}] = tile;
  }
  if (part == 1) {
    auto block_tiles = 0;
    for (auto [pos, tile] : tiles) {
      if (tile == 2) {
        block_tiles++;
      }
    }
    return adventofcode::ok(std::to_string(block_tiles));
  }
  program.at(0) = 2;
  screen_t screen = to_screen(tiles);
  breakout b {program, screen};
  b.run();
  return adventofcode::ok(std::to_string(b.score));
}

screen_t to_screen(const std::map<std::pair<int64_t, int64_t>, int64_t>& tiles) {
  screen_t screen;
  for (auto [p, tile] : tiles) {
    auto [x, y] = p;
    auto ux = (unsigned long int) x;
    auto uy = (unsigned long int) y;
    screen.resize(std::max(uy + 1, screen.size()));
    for (auto& row : screen) {
      row.resize(std::max(ux + 1, row.size()));
    }
    screen.at(uy).at(ux) = tile;
  }
  return screen;
}

void breakout::read() {
  auto output = e.read_all();
  auto it = output.begin();
  while (it != output.end()) {
    auto x = *it++;
    auto y = *it++;
    auto value = *it++;
    if (x == -1 && y == 0) {
      score = value;
      continue;
    }
    screen.at((unsigned) y).at((unsigned) x) = value;
  }
}

void breakout::move_paddle() {
  auto ball_x = screen.at(0).size();
  auto paddle_x = ball_x;
  for (auto row : screen) {
    for (auto x = 0lu; x < row.size(); x++) {
      switch (row[x]) {
      case 3:
        paddle_x = x;
        break;
      case 4:
        ball_x = x;
        break;
      }
    }
  }
  int64_t input;
  if (paddle_x > ball_x) {
    input = -1;
  } else if (paddle_x < ball_x) {
    input = 1;
  } else {
    input = 0;
  }
  e.write(input);
}

void breakout::run() {
  while (e.state != intcode::execution_state::halted) {
    e.run();
    read();
    move_paddle();
  }
}
