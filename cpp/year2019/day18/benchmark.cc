#include "cpp/year2019/day18/day18.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day18Part1(benchmark::State& state) {
  std::ifstream input("inputs/2019/18");
  for (auto _ : state) {
    day18::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day18Part1);

static void Year2019Day18Part2(benchmark::State& state) {
  std::ifstream input("inputs/2019/18");
  for (auto _ : state) {
    day18::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day18Part2);
