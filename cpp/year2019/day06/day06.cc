#include "year2019/day06/day06.h"

#include <istream>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

#include "absl/strings/str_split.h"

#include "adventofcode.h"

typedef std::unordered_map<std::string, std::vector<std::string>> children_tree;
typedef std::unordered_map<std::string, std::string> parent_tree;

adventofcode::answer_t solve(std::istream& is, int part);
std::pair<children_tree, parent_tree> parse(std::istream& is);
int sum_depth(const children_tree& tree, const std::string& root);
std::vector<std::string> path_to_root(const parent_tree& tree, const std::string& from);

namespace day06 {
  adventofcode::answer_t part1(std::istream& is) {
    return solve(is, 1);
  }
  adventofcode::answer_t part2(std::istream& is) {
    return solve(is, 2);
  }
}

adventofcode::answer_t solve(std::istream& is, int part) {
  auto trees = parse(is);
  auto c_tree = trees.first;
  auto p_tree = trees.second;
  if (part == 1) {
    auto sum = sum_depth(c_tree, "COM");
    return adventofcode::ok(std::to_string(sum));
  }
  auto you_path = path_to_root(p_tree, "YOU");
  auto san_path = path_to_root(p_tree, "SAN");
  auto you_it = you_path.crbegin();
  auto san_it = san_path.crbegin();
  while (*you_it == *san_it) {
    you_it++;
    san_it++;
  }
  int diff = 0;
  while(you_it != you_path.crend()) {
    diff++;
    you_it++;
  }
  while(san_it != san_path.crend()) {
    diff++;
    san_it++;
  }
  return adventofcode::ok(std::to_string(diff));
}

std::pair<children_tree, parent_tree> parse(std::istream& is) {
  children_tree c_tree;
  parent_tree p_tree;
  std::string line;
  while (std::getline(is, line)) {
    std::vector<std::string> parts = absl::StrSplit(line, ")");
    auto parent = parts[0];
    auto child = parts[1];
    std::vector<std::string> children;
    auto search = c_tree.find(parent);
    if (search != c_tree.end()) {
      auto pair = *search;
      children = pair.second;
    }
    children.push_back(child);
    c_tree[parent] = children;
    p_tree[child] = parent;
  }
  return {c_tree, p_tree};
}

int sum_depth_aux(const children_tree& tree, const std::string& node, const int depth) {
  int sum = depth;
  auto search = tree.find(node);
  if (search != tree.end()) {
    auto children = search->second;
    for (auto child : children) {
      sum += sum_depth_aux(tree, child, depth + 1);
    }
  }
  return sum;
}

int sum_depth(const children_tree& tree, const std::string& root) {
  return sum_depth_aux(tree, root, 0);
}

std::vector<std::string> path_to_root(const parent_tree& tree, const std::string& from) {
  auto search = tree.find(from);
  if (search == tree.end()) {
    return {};
  }
  std::vector<std::string> path;
  std::string current = from;
  while (search != tree.end()) {
    auto pair = *search;
    auto parent = pair.second;
    path.push_back(parent);
    current = parent;
    search = tree.find(current);
  }
  return path;
}
