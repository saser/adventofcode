#include "year2019/day10/day10.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day10Part1(benchmark::State& state) {
  std::ifstream input("year2019/testdata/10");
  for (auto _ : state) {
    day10::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day10Part1);

static void Year2019Day10Part2(benchmark::State& state) {
  std::ifstream input("year2019/testdata/10");
  for (auto _ : state) {
    day10::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day10Part2);
