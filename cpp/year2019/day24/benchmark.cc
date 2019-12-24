#include "year2019/day24/day24.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day24Part1(benchmark::State& state) {
  std::ifstream input("year2019/testdata/24");
  for (auto _ : state) {
    day24::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day24Part1);

static void Year2019Day24Part2(benchmark::State& state) {
  std::ifstream input("year2019/testdata/24");
  for (auto _ : state) {
    day24::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day24Part2);
