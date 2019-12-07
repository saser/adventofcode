#include "year2019/day06/day06.h"

#include <istream>
#include <string>
#include <unordered_map>
#include <vector>

#include "absl/strings/str_split.h"

#include "adventofcode.h"

typedef std::unordered_map<std::string, std::vector<std::string>> orbit_tree;

int sum_depth(const orbit_tree& tree, const std::string& root);

namespace day06 {
  adventofcode::answer_t part1(std::istream& is) {
    orbit_tree tree;
    std::string line;
    while (std::getline(is, line)) {
      std::vector<std::string> parts = absl::StrSplit(line, ")");
      auto parent = parts[0];
      auto child = parts[1];
      std::vector<std::string> children;
      auto search = tree.find(parent);
      if (search != tree.end()) {
        auto pair = *search;
        children = pair.second;
      }
      children.push_back(child);
      tree[parent] = children;
    }
    int answer = sum_depth(tree, "COM");
    return adventofcode::ok(std::to_string(answer));
  }
}

int sum_depth_aux(const orbit_tree& tree, const std::string& node, const int depth) {
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

int sum_depth(const orbit_tree& tree, const std::string& root) {
  return sum_depth_aux(tree, root, 0);
}
