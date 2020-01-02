#include "cpp/year2019/day23/day23.h"

#include <deque>
#include <istream>
#include <optional>
#include <string>
#include <vector>

#include "cpp/adventofcode.h"
#include "cpp/year2019/intcode/intcode.h"

adventofcode::answer_t solve(std::istream& is, int part);

namespace day23 {
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
  std::vector<intcode::execution> computers;
  computers.reserve(50);
  for (auto i = 0; i < 50; i++) {
    intcode::execution e {program};
    e.write(i);
    computers.push_back(e);
  }
  std::vector<std::deque<int64_t>> queues(50);
  int64_t nat_x = 0;
  int64_t nat_y = 0;
  std::optional<int64_t> last_y;
  while (true) {
    for (std::vector<intcode::execution>::size_type i = 0; i < computers.size(); i++) {
      auto& e = computers[i];
      e.run();
      auto output = e.read_all();
      if (output.empty()) {
        continue;
      }
      auto it = output.begin();
      while (it != output.end()) {
        auto destination = *it++;
        auto x = *it++;
        auto y = *it++;
        if (destination == 255) {
          if (part == 1) {
            return adventofcode::ok(std::to_string(y));
          }
          nat_x = x;
          nat_y = y;
        } else {
          queues.at(destination).push_back(x);
          queues.at(destination).push_back(y);
        }
      }
    }
    auto idle = true;
    for (std::vector<std::deque<int64_t>>::size_type i = 0; i < queues.size(); i++) {
      auto& q = queues[i];
      if (q.empty()) {
        q.push_back(-1);
      } else {
        idle = false;
      }
      computers.at(i).write_all(q);
      q.clear();
    }
    if (part == 2 && idle) {
      if (last_y.has_value() && *last_y == nat_y) {
        return adventofcode::ok(std::to_string(nat_y));
      } else {
        last_y = nat_y;
      }
      auto& e = computers.at(0);
      e.write_all({nat_x, nat_y});
    }
  }
}
