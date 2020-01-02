#include "cpp/year2019/day25/day25.h"

#include <deque>
#include <iostream>
#include <map>
#include <optional>
#include <regex>
#include <set>
#include <string>
#include <tuple>
#include <vector>

#include "cpp/adventofcode.h"
#include "cpp/year2019/intcode/intcode.h"

using lines_t = std::vector<std::string>;
using path_t = std::vector<std::string>;
using instructions_t = std::vector<std::string>;

adventofcode::answer_t solve(std::istream& is, int part);

lines_t output_lines(const intcode::output& output);
void print_lines(const lines_t& lines);
std::optional<std::string> extract_item(const lines_t& lines, const std::regex& item_re);
std::vector<std::string> extract_list(const lines_t& lines,
                                      const std::regex& header_re,
                                      const std::regex& item_re);
std::optional<std::string> extract_room_name(const lines_t& lines);
std::optional<std::string> extract_password(const lines_t& lines);
std::vector<std::string> extract_directions(const lines_t& lines);
std::vector<std::string> extract_items(const lines_t& lines);
bool cant_move(const lines_t& lines);
bool next_is_pressure_sensitive(const lines_t& lines);
bool is_lighter(const lines_t& lines);
bool is_heavier(const lines_t& lines);

std::string reverse(const std::string& direction);
path_t reverse_path(const path_t& path);

template<class T>
bool is_subset(const std::set<T>& a, const std::set<T>& b);

template<class T>
bool is_subset_any(const std::set<T>& a, const std::set<std::set<T>>& bs);

struct player_t {
  intcode::execution reset;
  std::string current_room;

  std::map<std::string, path_t> room_paths;
  std::optional<std::string> pressure_sensitive_room;
  std::map<std::string, std::string> item_locations;
  std::set<std::string> dangerous_items {"infinite loop"};
  std::set<std::string> required_items;

  void find_rooms();
  void find_items();
  void try_items();
  void find_required_items();

  std::set<std::string> safe_items() const;
  instructions_t collect_items(const std::set<std::string>& items) const;
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
  player.find_items();
  player.try_items();
  player.find_required_items();
  for (auto i : player.collect_items(player.required_items)) {
    e.write_stringln(i);
  }
  for (auto step : player.room_paths.at(*player.pressure_sensitive_room)) {
    e.write_stringln(step);
  }
  e.run();
  auto lines = output_lines(e.read_all());
  auto password = *extract_password(lines);
  return adventofcode::ok(password);
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

std::optional<std::string> extract_password(const lines_t& lines) {
  return extract_item(lines, std::regex(R"((\d+).+airlock)"));
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

bool next_is_pressure_sensitive(const lines_t& lines) {
  return extract_item(lines, std::regex(R"(next room, a pressure-sensitive)")).has_value();
}

bool is_lighter(const lines_t& lines) {
  return extract_item(lines, std::regex(R"(lighter)")).has_value();
}

bool is_heavier(const lines_t& lines) {
  return extract_item(lines, std::regex(R"(heavier)")).has_value();
}

std::string reverse(const std::string& direction) {
  if (direction == "north") {
    return "south";
  } else if (direction == "south") {
    return "north";
  } else if (direction == "east") {
    return "west";
  } else if (direction == "west") {
    return "east";
  } else {
    return "north";
  }
}

path_t reverse_path(const path_t& path) {
  path_t p;
  p.reserve(path.size());
  for (auto it = path.crbegin(); it != path.crend(); it++) {
    p.push_back(reverse(*it));
  }
  return p;
}

template<class T>
bool is_subset(const std::set<T>& a, const std::set<T>& b) {
  for (auto t : a) {
    if (b.find(t) == b.end()) {
      return false;
    }
  }
  return true;
}

template<class T>
bool is_subset_any(const std::set<T>& a, const std::set<std::set<T>>& bs) {
  for (auto b : bs) {
    if (is_subset(a, b)) {
      return true;
    }
  }
  return false;
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
    auto directions = extract_directions(lines);
    if (next_is_pressure_sensitive(lines)) {
      auto last_step = path.back();
      for (auto direction : directions) {
        if (direction == reverse(last_step)) {
          continue;
        }
        e.write_stringln(direction);
        e.run();
        auto ps_room_name = *(extract_room_name(output_lines(e.read_all())));
        pressure_sensitive_room = ps_room_name;
        auto ps_path = path;
        ps_path.push_back(direction);
        room_paths[ps_room_name] = ps_path;
      }
      continue;
    }
    for (auto direction : directions) {
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

std::set<std::string> player_t::safe_items() const {
  std::set<std::string> items;
  for (auto [item, _] : item_locations) {
    if (dangerous_items.find(item) != dangerous_items.end()) {
      continue;
    }
    items.insert(item);
  }
  return items;
}

instructions_t player_t::collect_items(const std::set<std::string>& items) const {
  instructions_t instructions;
  for (auto item : items) {
    if (item_locations.find(item) == item_locations.end()) {
      std::cout << "unknown item: " << item << std::endl;
      continue;
    }
    auto path = room_paths.at(item_locations.at(item));
    instructions.insert(instructions.end(), path.begin(), path.end());
    instructions.push_back("take " + item);
    auto backtrack = reverse_path(path);
    instructions.insert(instructions.end(), backtrack.begin(), backtrack.end());
  }
  return instructions;
}

void player_t::find_required_items() {
  auto e = reset;
  auto items = safe_items();
  for (auto step : collect_items(items)) {
    e.write_stringln(step);
  }
  auto path = room_paths.at(*pressure_sensitive_room);
  for (auto it = path.cbegin(); it != path.cend() - 1; it++) {
    e.write_stringln(*it);
  }
  for (auto item : items) {
    e.write_stringln("drop " + item);
  }
  e.run();
  e.read_all();
  using items_t = std::set<std::string>;
  std::set<items_t> too_heavy;
  using elem_t = std::tuple<intcode::execution, items_t>;
  std::deque<elem_t> q;
  q.push_back({e, {}});
  while (!q.empty()) {
    auto [e, carried_items] = q.front();
    q.pop_front();
    if (is_subset_any(carried_items, too_heavy)) {
      continue;
    }
    e.write_stringln(path.back());
    e.run();
    auto lines = output_lines(e.read_all());
    if (is_lighter(lines)) {
      too_heavy.insert(carried_items);
      continue;
    }
    if (!is_heavier(lines)) {
      required_items = carried_items;
      break;
    }
    for (auto item : items) {
      if (carried_items.find(item) != carried_items.end()) {
        continue;
      }
      auto new_e = e;
      new_e.write_stringln("take " + item);
      new_e.run();
      new_e.read_all();
      auto new_carried_items = carried_items;
      new_carried_items.insert(item);
      q.push_back({new_e, new_carried_items});
    }
  }
}
