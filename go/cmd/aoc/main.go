package main

import (
	"flag"
	"fmt"
	"io"
	"os"
)

var (
	fYear  = flag.Uint("year", 0, "year of event")
	fDay   = flag.Uint("day", 0, "day in event")
	fPart  = flag.Uint("part", 0, "part of day")
	fInput = flag.String("input", "", "path to file to read input from (default: read input from stdin)")
)

type Solution func(io.Reader) (string, error)
type Day struct {
	One Solution
	Two Solution
}

var solutions map[uint]map[uint]Day

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
	var solution Solution
	if part == 1 {
		solution = perPart.One
	} else {
		solution = perPart.Two
	}
	if solution == nil {
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
	answer, err := solution(in)
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
