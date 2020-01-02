#include "cpp/year2019/day20/day20.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day20Part1(benchmark::State& state) {
  std::ifstream input("inputs/2019/20");
  for (auto _ : state) {
    day20::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day20Part1);

static void Year2019Day20Part2(benchmark::State& state) {
  std::ifstream input("inputs/2019/20");
  for (auto _ : state) {
    day20::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day20Part2);
