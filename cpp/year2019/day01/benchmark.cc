#include "cpp/year2019/day01/day01.h"

#include <fstream>
#include <istream>

#include "benchmark/benchmark.h"

static void Year2019Day01Part1(benchmark::State& state) {
  std::ifstream input("inputs/2019/01");
  for (auto _ : state) {
    day01::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day01Part1);


static void Year2019Day01Part2(benchmark::State& state) {
  std::ifstream input("inputs/2019/01");
  for (auto _ : state) {
    day01::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year2019Day01Part2);
