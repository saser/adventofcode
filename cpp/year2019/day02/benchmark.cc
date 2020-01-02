#include "cpp/year2019/day02/day02.h"

#include <fstream>
#include <istream>

#include "benchmark/benchmark.h"

static void Year2019Day02Part1(benchmark::State& state) {
  std::ifstream input("inputs/2019/02");
  for (auto _ : state) {
    day02::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day02Part1);


static void Year2019Day02Part2(benchmark::State& state) {
  std::ifstream input("inputs/2019/02");
  for (auto _ : state) {
    day02::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day02Part2);
