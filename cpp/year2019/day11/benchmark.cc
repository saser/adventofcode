#include "cpp/year2019/day11/day11.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day11Part1(benchmark::State& state) {
  std::ifstream input("inputs/2019/11");
  for (auto _ : state) {
    day11::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day11Part1);

static void Year2019Day11Part2(benchmark::State& state) {
  std::ifstream input("inputs/2019/11");
  for (auto _ : state) {
    day11::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day11Part2);
