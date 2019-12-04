#include "year2019/day03/day03.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day03Part1(benchmark::State& state) {
  std::ifstream input("year2019/testdata/03");
  for (auto _ : state) {
    day03::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day03Part1);

static void Year2019Day03Part2(benchmark::State& state) {
  std::ifstream input("year2019/testdata/03");
  for (auto _ : state) {
    day03::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day03Part2);
