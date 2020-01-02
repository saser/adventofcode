#include "cpp/year2019/day13/day13.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day13Part1(benchmark::State& state) {
  std::ifstream input("inputs/2019/13");
  for (auto _ : state) {
    day13::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day13Part1);

static void Year2019Day13Part2(benchmark::State& state) {
  std::ifstream input("inputs/2019/13");
  for (auto _ : state) {
    day13::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day13Part2);
