#include "cpp/year2019/day07/day07.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day07Part1(benchmark::State& state) {
  std::ifstream input("inputs/2019/07");
  for (auto _ : state) {
    day07::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day07Part1);

static void Year2019Day07Part2(benchmark::State& state) {
  std::ifstream input("inputs/2019/07");
  for (auto _ : state) {
    day07::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day07Part2);
