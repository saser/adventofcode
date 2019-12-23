#include "year2019/day23/day23.h"

#include <deque>
#include <istream>
#include <string>
#include <vector>

#include "adventofcode.h"
#include "year2019/intcode/intcode.h"

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
          return adventofcode::ok(std::to_string(y));
        }
        queues[destination].push_back(x);
        queues[destination].push_back(y);
      }
    }
    for (std::vector<std::deque<int64_t>>::size_type i = 0; i < queues.size(); i++) {
      auto& q = queues[i];
      if (q.empty()) {
        q.push_back(-1);
      }
      computers.at(i).write_all(q);
      q.clear();
    }
  }
}
