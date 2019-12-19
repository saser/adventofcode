#include "year2019/day16/day16.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day16Part1(benchmark::State& state) {
  std::ifstream input("year2019/testdata/16");
  for (auto _ : state) {
    day16::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day16Part1);

static void Year2019Day16Part2(benchmark::State& state) {
  std::ifstream input("year2019/testdata/16");
  for (auto _ : state) {
    day16::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day16Part2);
