#include "year2019/day12/day12.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day12Part1(benchmark::State& state) {
  std::ifstream input("year2019/testdata/12");
  for (auto _ : state) {
    day12::part1(input, day12::STEPS);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day12Part1);

static void Year2019Day12Part2(benchmark::State& state) {
  std::ifstream input("year2019/testdata/12");
  for (auto _ : state) {
    day12::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day12Part2);
