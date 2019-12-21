#include "year2019/day19/day19.h"

#include <istream>
#include <string>

#include "adventofcode.h"
#include "year2019/intcode/intcode.h"

struct point_t {
  int64_t x;
  int64_t y;
};

struct tractor_t {
  intcode::memory program;

  int64_t test(point_t p);
  point_t next(const point_t& base);
  point_t beam_base(int64_t x_max, int64_t y_max);
  point_t upper_left_corner(const point_t& base, int64_t width, int64_t height);
};

adventofcode::answer_t solve(std::istream& is, int part);

namespace day19 {
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
  intcode::memory program = intcode::parse(line);
  tractor_t tractor {program};
  if (part == 1) {
    auto sum = 0;
    for (int64_t x = 0; x < 50; x++) {
      for (int64_t y = 0; y < 50; y++) {
        sum += tractor.test(point_t {x, y});
      }
    }
    return adventofcode::ok(std::to_string(sum));
  }
  auto base = tractor.beam_base(50, 50);
  auto corner = tractor.upper_left_corner(base, 100, 100);
  auto out = corner.x * 10000 + corner.y;
  return adventofcode::ok(std::to_string(out));
}

int64_t tractor_t::test(point_t p) {
  auto [_, output] = intcode::run(program, {p.x, p.y});
  return output[0];
}

point_t tractor_t::next(const point_t& base) {
  auto next = base;
  next.x++;
  while (test(next) == 1) {
    next.x++;
  }
  next.y++;
  while (test(next) != 1) {
    next.y++;
  }
  return next;
}

point_t tractor_t::beam_base(int64_t x_max, int64_t y_max) {
  // Skip over (0, 0) as that is guaranteed to contain a 1.
  for (int64_t x = 0; x <= x_max; x++) {
    for (int64_t y = 1; y <= y_max; y++) {
      point_t p {x, y};
      if (test(p) == 1) {
        return p;
      }
    }
  }
  return point_t {-1, -1};
}

point_t tractor_t::upper_left_corner(const point_t& base, int64_t width, int64_t height) {
  int64_t x_offset = width - 1;
  int64_t y_offset = height - 1;
  point_t upper_right_corner = base;
  while (upper_right_corner.x < x_offset
         || upper_right_corner.y < y_offset
         || test(point_t {upper_right_corner.x - x_offset, upper_right_corner.y + y_offset}) != 1) {
    upper_right_corner = next(upper_right_corner);
  }
  return point_t {upper_right_corner.x - width + 1, upper_right_corner.y};
}
