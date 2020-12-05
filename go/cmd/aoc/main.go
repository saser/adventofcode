package main

import (
	"flag"
	"fmt"
	"io"
	"io/ioutil"
	"os"

	"github.com/Saser/adventofcode/internal/testcase"
	year2015day01 "github.com/Saser/adventofcode/internal/year2015/day01"
	year2015day02 "github.com/Saser/adventofcode/internal/year2015/day02"
	year2015day03 "github.com/Saser/adventofcode/internal/year2015/day03"
	year2015day04 "github.com/Saser/adventofcode/internal/year2015/day04"
	year2015day05 "github.com/Saser/adventofcode/internal/year2015/day05"
	year2015day06 "github.com/Saser/adventofcode/internal/year2015/day06"
	year2015day07 "github.com/Saser/adventofcode/internal/year2015/day07"
	year2015day08 "github.com/Saser/adventofcode/internal/year2015/day08"
	year2015day09 "github.com/Saser/adventofcode/internal/year2015/day09"
	year2015day10 "github.com/Saser/adventofcode/internal/year2015/day10"
	year2015day11 "github.com/Saser/adventofcode/internal/year2015/day11"
	year2015day12 "github.com/Saser/adventofcode/internal/year2015/day12"
	year2015day13 "github.com/Saser/adventofcode/internal/year2015/day13"
	year2015day14 "github.com/Saser/adventofcode/internal/year2015/day14"
	year2015day15 "github.com/Saser/adventofcode/internal/year2015/day15"
	year2015day16 "github.com/Saser/adventofcode/internal/year2015/day16"
	year2015day17 "github.com/Saser/adventofcode/internal/year2015/day17"
	year2015day18 "github.com/Saser/adventofcode/internal/year2015/day18"
	year2015day19 "github.com/Saser/adventofcode/internal/year2015/day19"
	year2015day20 "github.com/Saser/adventofcode/internal/year2015/day20"
	year2015day21 "github.com/Saser/adventofcode/internal/year2015/day21"
	year2015day22 "github.com/Saser/adventofcode/internal/year2015/day22"
	year2015day23 "github.com/Saser/adventofcode/internal/year2015/day23"
	year2015day24 "github.com/Saser/adventofcode/internal/year2015/day24"
	year2015day25 "github.com/Saser/adventofcode/internal/year2015/day25"
)

var (
	fYear  = flag.Uint("year", 0, "year of event")
	fDay   = flag.Uint("day", 0, "day in event")
	fPart  = flag.Uint("part", 0, "part of day")
	fInput = flag.String("input", "", "path to file to read input from (default: read input from stdin)")
)

type Day struct {
	One testcase.Solution
	Two testcase.Solution
}

var solutions = map[uint]map[uint]Day{
	2015: {
		1:  {One: year2015day01.Part1, Two: year2015day01.Part2},
		2:  {One: year2015day02.Part1, Two: year2015day02.Part2},
		3:  {One: year2015day03.Part1, Two: year2015day03.Part2},
		4:  {One: year2015day04.Part1, Two: year2015day04.Part2},
		5:  {One: year2015day05.Part1, Two: year2015day05.Part2},
		6:  {One: year2015day06.Part1, Two: year2015day06.Part2},
		7:  {One: year2015day07.Part1, Two: year2015day07.Part2},
		8:  {One: year2015day08.Part1, Two: year2015day08.Part2},
		9:  {One: year2015day09.Part1, Two: year2015day09.Part2},
		10: {One: year2015day10.Part1, Two: year2015day10.Part2},
		11: {One: year2015day11.Part1, Two: year2015day11.Part2},
		12: {One: year2015day12.Part1, Two: year2015day12.Part2},
		13: {One: year2015day13.Part1, Two: year2015day13.Part2},
		14: {One: year2015day14.Part1, Two: year2015day14.Part2},
		15: {One: year2015day15.Part1, Two: year2015day15.Part2},
		16: {One: year2015day16.Part1, Two: year2015day16.Part2},
		17: {One: year2015day17.Part1, Two: year2015day17.Part2},
		18: {One: year2015day18.Part1, Two: year2015day18.Part2},
		19: {One: year2015day19.Part1, Two: year2015day19.Part2},
		20: {One: year2015day20.Part1, Two: year2015day20.Part2},
		21: {One: year2015day21.Part1, Two: year2015day21.Part2},
		22: {One: year2015day22.Part1, Two: year2015day22.Part2},
		23: {One: year2015day23.Part1, Two: year2015day23.Part2},
		24: {One: year2015day24.Part1, Two: year2015day24.Part2},
		25: {One: year2015day25.Part1},
	},
}

func imain() (exitCode int) {
	flag.Parse()

	year := *fYear
	if year == 0 {
		fmt.Println("A year must be specified.")
		return 1
	}
	perDay, ok := solutions[year]
	if !ok {
		fmt.Printf("Invalid year: %d\n", *fYear)
		return 1
	}

	day := *fDay
	if day == 0 {
		fmt.Println("A day must be specified.")
		return 1
	}
	perPart, ok := perDay[day]
	if !ok {
		fmt.Printf("Year %d has no solution for day %d.\n", year, day)
		return 1
	}

	part := *fPart
	if part == 0 {
		fmt.Println("A part must be specified.")
		return 1
	}
	if part != 1 && part != 2 {
		fmt.Printf("Invalid part: %d\n", part)
	}
	var sol testcase.Solution
	if part == 1 {
		sol = perPart.One
	} else {
		sol = perPart.Two
	}
	if sol == nil {
		fmt.Printf("Year %d, day %d, part %d has no solution.\n", year, day, part)
		return 1
	}

	var in io.Reader
	input := *fInput
	if input == "" {
		in = os.Stdin
	} else {
		f, err := os.Open(input)
		if err != nil {
			fmt.Printf("Failed to open input file: %v\n", err)
			return 2
		}
		in = f
		defer func() {
			if err := f.Close(); err != nil {
				fmt.Printf("Failed to close input file: %v\n", err)
				exitCode = 2
			}
		}()
	}
	data, err := ioutil.ReadAll(in)
	if err != nil {
		fmt.Printf("Failed to read input: %v\n", err)
		return 2
	}
	answer, err := sol(string(data))
	if err != nil {
		fmt.Printf("Error while running solution: %v\n", err)
		return 3
	}
	fmt.Println(answer)

	return 0
}

func main() {
	os.Exit(imain())
}
