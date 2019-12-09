#include "year2019/day08/day08.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day08Part1(benchmark::State& state) {
  std::ifstream input("year2019/testdata/08");
  for (auto _ : state) {
    day08::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day08Part1);

static void Year2019Day08Part2(benchmark::State& state) {
  std::ifstream input("year2019/testdata/08");
  for (auto _ : state) {
    day08::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day08Part2);
