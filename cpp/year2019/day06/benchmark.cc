#include "cpp/year2019/day06/day06.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day06Part1(benchmark::State& state) {
  std::ifstream input("inputs/2019/06");
  for (auto _ : state) {
    day06::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day06Part1);

static void Year2019Day06Part2(benchmark::State& state) {
  std::ifstream input("inputs/2019/06");
  for (auto _ : state) {
    day06::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day06Part2);
