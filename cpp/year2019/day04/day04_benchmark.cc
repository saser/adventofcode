#include "year2019/day04/day04.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day04Part1(benchmark::State& state) {
  std::ifstream input("year2019/testdata/04");
  for (auto _ : state) {
    day04::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day04Part1);
