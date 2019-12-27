#include "year2019/day25/day25.h"

#include <deque>
#include <iostream>
#include <map>
#include <optional>
#include <regex>
#include <set>
#include <string>
#include <tuple>
#include <vector>

#include "adventofcode.h"
#include "year2019/intcode/intcode.h"

using lines_t = std::vector<std::string>;
using path_t = std::vector<std::string>;

adventofcode::answer_t solve(std::istream& is, int part);

lines_t output_lines(const intcode::output& output);
void print_lines(const lines_t& lines);
std::optional<std::string> extract_item(const lines_t& lines, const std::regex& item_re);
std::vector<std::string> extract_list(const lines_t& lines,
                                      const std::regex& header_re,
                                      const std::regex& item_re);
std::optional<std::string> extract_room_name(const lines_t& lines);
std::vector<std::string> extract_directions(const lines_t& lines);
std::vector<std::string> extract_items(const lines_t& lines);
bool cant_move(const lines_t& lines);

struct player_t {
  intcode::execution reset;
  std::string current_room;

  std::map<std::string, path_t> room_paths;
  std::map<std::string, std::string> item_locations;
  std::set<std::string> dangerous_items {"infinite loop"};

  void find_rooms();
  void find_items();
  void try_items();
};

namespace day25 {
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
  intcode::execution e {program};
  player_t player {e};
  player.find_rooms();
  for (auto [room, path] : player.room_paths) {
    std::cout << room << ": ";
    for (auto step : path) {
      std::cout << step << ", ";
    }
    std::cout << std::endl;
  }
  std::cout << "-----------------" << std::endl;
  player.find_items();
  for (auto [item, room] : player.item_locations) {
    std::cout << item << ": " << room << std::endl;
  }
  std::cout << "-----------------" << std::endl;
  player.try_items();
  for (auto item : player.dangerous_items) {
    std::cout << item << std::endl;
  }
  return adventofcode::err("not implemented yet");
}

lines_t output_lines(const intcode::output& output) {
  std::vector<std::string> lines;
  std::string line;
  for (auto c : output) {
    if (c == '\n') {
      lines.push_back(line);
      line.clear();
      continue;
    }
    line += c;
  }
  return lines;
}

void print_lines(const lines_t& lines) {
  for (auto line : lines) {
    std::cout << line << std::endl;
  }
}

std::optional<std::string> extract_item(const lines_t& lines, const std::regex& item_re) {
  for (auto line : lines) {
    std::smatch match;
    if (std::regex_search(line, match, item_re)) {
      return std::make_optional(match[1]);
    }
  }
  return std::nullopt;
}

std::vector<std::string> extract_list(const lines_t& lines,
                                      const std::regex& header_re,
                                      const std::regex& item_re) {
  std::vector<std::string> v;
  bool header_found = false;
  for (auto line : lines) {
    if (!header_found) {
      if (std::regex_search(line, header_re)) {
        header_found = true;
      }
    } else {
      std::smatch match;
      if (std::regex_search(line, match, item_re)) {
        v.push_back(match[1]);
      } else {
        break;
      }
    }
  }
  return v;
}

std::optional<std::string> extract_room_name(const lines_t& lines) {
  return extract_item(lines, std::regex(R"(== (.+) ==)"));
}

std::vector<std::string> extract_directions(const lines_t& lines) {
  return extract_list(lines,
                      std::regex(R"(Doors here lead:)"),
                      std::regex(R"((north|east|south|west))"));
}

std::vector<std::string> extract_items(const lines_t& lines) {
  return extract_list(lines,
                      std::regex(R"(Items here:)"),
                      std::regex(R"(- (.+))"));
}

bool cant_move(const lines_t& lines) {
  return extract_item(lines, std::regex(R"(can't move)")).has_value();
}

void player_t::find_rooms() {
  using elem_t = std::tuple<intcode::execution, path_t>;
  std::deque<elem_t> q;
  q.push_back({reset, {}});
  while (!q.empty()) {
    auto [e, path] = q.front();
    q.pop_front();
    e.run();
    auto lines = output_lines(e.read_all());
    auto room_name = extract_room_name(lines);
    if (!room_name.has_value()) {
      std::cout << "no room name! full output:" << std::endl;
      std::cout << "--------------------------" << std::endl;
      print_lines(lines);
      std::cout << "--------------------------" << std::endl;
      continue;
    }
    if (room_paths.find(*room_name) != room_paths.end()) {
      continue;
    }
    room_paths[*room_name] = path;
    for (auto direction : extract_directions(lines)) {
      auto new_e = e;
      new_e.write_stringln(direction);
      auto new_path = path;
      new_path.push_back(direction);
      q.push_back({new_e, new_path});
    }
  }
}

void player_t::find_items() {
  for (auto [room, path] : room_paths) {
    if (path.empty()) {
      continue;
    }
    auto ec = reset;
    for (auto it = path.cbegin(); it != path.cend() - 1; it++) {
      ec.write_stringln(*it);
    }
    ec.run();
    ec.read_all();
    ec.write_stringln(path.back());
    ec.run();
    auto lines = output_lines(ec.read_all());
    for (auto item : extract_items(lines)) {
      item_locations[item] = room;
    }
  }
}

void player_t::try_items() {
  for (auto [item, room] : item_locations) {
    if (dangerous_items.find(item) != dangerous_items.end()) {
      continue;
    }
    auto ec = reset;
    for (auto step : room_paths.at(room)) {
      ec.write_stringln(step);
    }
    ec.write_stringln("take " + item);
    ec.run();
    if (ec.state == intcode::execution_state::halted) {
      dangerous_items.insert(item);
    }
    ec.read_all();
    ec.write_stringln("north");
    ec.run();
    auto lines = output_lines(ec.read_all());
    if (cant_move(lines)) {
      dangerous_items.insert(item);
    }
  }
}
