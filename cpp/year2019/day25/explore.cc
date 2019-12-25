#include <fstream>
#include <iostream>
#include <regex>
#include <string>

#include "year2019/intcode/intcode.h"

int main(int argc, char** argv) {
  std::ifstream input_file("year2019/testdata/25");
  std::string input;
  std::getline(input_file, input);
  intcode::memory program = intcode::parse(input);
  intcode::execution e {program};
  e.run();
  while (e.state != intcode::execution_state::halted) {
    auto output = e.read_all();
    for (auto c : output) {
      std::cout << (char) c;
    }
    std::cout << std::endl;
    std::string instruction;
    std::getline(std::cin, instruction);
    for (auto c : instruction) {
      e.write(c);
    }
    e.write('\n');
    e.run();
  }
  auto output = e.read_all();
  for (auto c : output) {
    std::cout << (char) c;
  }
  std::cout << std::endl;
}
