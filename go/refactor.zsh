#!/usr/bin/zsh

sed -E -i \
    -e '/const inputFile/r tcvars.go' \
    -e '/func BenchmarkPart[12]/{n;d;}' \
    ./internal/year*/**/*_test.go

sed -E -i \
    -e 's/func Part(1|2)\(r io\.Reader/func Part\1(input string/' \
    -e 's/func solve\(r io\.Reader/func solve(input string/' \
    -e 's/return solve\(r/return solve(input/' \
    ./internal/year*/**/*.go

rules=( \
	'testcase.TestCase -> testcase.TestCase2' \
	'solution.Solution -> testcase.Solution' \
	'testcase.FromString(n, i, w) -> testcase.New(n, i, w)' \
	'testcase.FromFile(t, f, w) -> testcase.NewFile(f, f, w)' \
	'testcase.Run(t, c, s) -> c.Test(t, s)' \
	'testcase.Bench(b, c, Part1) -> tcPart1.Benchmark(b, Part1)' \
	'testcase.Bench(b, c, Part2) -> tcPart2.Benchmark(b, Part2)' \
)

for rule in "${rules[@]}"; do
    gofmt -w -r "$rule" ./internal/year*/**/*_test.go
done

gofumports -w ./internal
