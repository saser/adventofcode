#include "{{.FullYear}}/{{.FullDay}}/{{.FullDay}}.h"

#include <fstream>

#include "benchmark/benchmark.h"

static void Year{{.Year}}Day{{.PaddedDay}}Part1(benchmark::State& state) {
  std::ifstream input("{{.FullYear}}/testdata/{{.PaddedDay}}");
  for (auto _ : state) {
    {{.FullDay}}::part1(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year{{.Year}}Day{{.PaddedDay}}Part1);

static void Year{{.Year}}Day{{.PaddedDay}}Part2(benchmark::State& state) {
  std::ifstream input("{{.FullYear}}/testdata/{{.PaddedDay}}");
  for (auto _ : state) {
    {{.FullDay}}::part2(input);
    input.clear();
    input.seekg(0);
  }
}
BENCHMARK(Year{{.Year}}Day{{.PaddedDay}}Part2);
