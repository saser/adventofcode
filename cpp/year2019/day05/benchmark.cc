#include "cpp/year2019/day05/day05.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day05Part1(benchmark::State& state) {
  std::ifstream input("inputs/2019/05");
  for (auto _ : state) {
    day05::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day05Part1);

static void Year2019Day05Part2(benchmark::State& state) {
  std::ifstream input("inputs/2019/05");
  for (auto _ : state) {
    day05::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day05Part2);
