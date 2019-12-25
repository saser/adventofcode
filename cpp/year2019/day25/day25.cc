#include "year2019/day25/day25.h"

#include <iostream>
#include <optional>
#include <regex>
#include <string>
#include <vector>

#include "adventofcode.h"
#include "year2019/intcode/intcode.h"

using lines_t = std::vector<std::string>;

adventofcode::answer_t solve(std::istream& is, int part);

lines_t output_lines(const intcode::output& output);
std::optional<std::string> extract_item(const lines_t& lines, const std::regex& item_re);
std::vector<std::string> extract_list(const lines_t& lines,
                                      const std::regex& header_re,
                                      const std::regex& item_re);
std::optional<std::string> extract_room_name(const lines_t& lines);
std::vector<std::string> extract_directions(const lines_t& lines);
std::vector<std::string> extract_items(const lines_t& lines);

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
  e.run();
  auto lines = output_lines(e.read_all());
  if (auto room_name = extract_room_name(lines); room_name.has_value()) {
    std::cout << "room name: " << *room_name << std::endl;
  }
  std::cout << "directions: ";
  for (auto direction : extract_directions(lines)) {
    std::cout << direction << ", ";
  }
  std::cout << std::endl;
  std::cout << "items: ";
  for (auto item : extract_items(lines)) {
    std::cout << item << ", ";
  }
  std::cout << std::endl;
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
