#include "cpp/year2019/day21/day21.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year2019Day21Part1(benchmark::State& state) {
  std::ifstream input("inputs/2019/21");
  for (auto _ : state) {
    day21::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day21Part1);

static void Year2019Day21Part2(benchmark::State& state) {
  std::ifstream input("inputs/2019/21");
  for (auto _ : state) {
    day21::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day21Part2);
