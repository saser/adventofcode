#include "year2019/day05/day05.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day05Part1(benchmark::State& state) {
  std::ifstream input("year2019/testdata/05");
  for (auto _ : state) {
    day05::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day05Part1);
