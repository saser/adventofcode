package main

import (
	"flag"
	"fmt"
	"io"
	"os"

	"github.com/Saser/adventofcode/internal/solution"
	"github.com/Saser/adventofcode/internal/year2015"
)

var (
	fYear  = flag.Uint("year", 0, "year of event")
	fDay   = flag.Uint("day", 0, "day in event")
	fPart  = flag.Uint("part", 0, "part of day")
	fInput = flag.String("input", "", "path to file to read input from (default: read input from stdin)")
)

type Day struct {
	One solution.Solution
	Two solution.Solution
}

var solutions = map[uint]map[uint]Day{
	2015: map[uint]Day{
		1: {One: year2015.Day01One, Two: year2015.Day01Two},
		2: {One: year2015.Day02One, Two: year2015.Day02Two},
		3: {One: year2015.Day03One, Two: year2015.Day03Two},
		4: {One: year2015.Day04One, Two: year2015.Day04Two},
		5: {One: year2015.Day05One, Two: year2015.Day05Two},
		6: {One: year2015.Day06One},
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
	var sol solution.Solution
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
	answer, err := sol(in)
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
