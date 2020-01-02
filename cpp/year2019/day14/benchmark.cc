#include "cpp/year2019/day14/day14.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day14Part1(benchmark::State& state) {
  std::ifstream input("inputs/2019/14");
  for (auto _ : state) {
    day14::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day14Part1);

static void Year2019Day14Part2(benchmark::State& state) {
  std::ifstream input("inputs/2019/14");
  for (auto _ : state) {
    day14::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day14Part2);
