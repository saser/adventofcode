#include "year2019/day13/day13.h"

#include <istream>
#include <string>
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
  screen_t screen;
  auto block_tiles = 0;
  auto it = output.begin();
  while (it != output.end()) {
    auto x = *it++;
    auto y = *it++;
    auto tile = *it++;
    auto n_rows = std::max((unsigned long int) y + 1, screen.size());
    auto n_cols = std::max((unsigned long int) x + 1, screen.size());
    screen.resize(n_rows);
    for (auto& row : screen) {
      row.resize(n_cols);
    }
    screen.at(y).at(x) = tile;
    if (tile == 2) {
      block_tiles++;
    }
  }
  if (part == 1) {
    return adventofcode::ok(std::to_string(block_tiles));
  }
  program.at(0) = 2;
  breakout b {program, screen};
  b.run();
  return adventofcode::ok(std::to_string(b.score));
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
