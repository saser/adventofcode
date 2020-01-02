#include "cpp/year2019/day15/day15.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day15Part1(benchmark::State& state) {
  std::ifstream input("inputs/2019/15");
  for (auto _ : state) {
    day15::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day15Part1);

static void Year2019Day15Part2(benchmark::State& state) {
  std::ifstream input("inputs/2019/15");
  for (auto _ : state) {
    day15::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day15Part2);
