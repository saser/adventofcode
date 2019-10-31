package main

import (
	"flag"
	"fmt"
	"io"
	"os"
)

var (
	fYear = flag.Uint("year", 0, "year of event")
	fDay  = flag.Uint("day", 0, "day in event")
	fPart = flag.Uint("part", 0, "part of day")
)

type Solution func(io.Reader) (string, error)
type Day struct {
	One Solution
	Two Solution
}
type Years map[uint]Days
type Days map[uint]Day

var okYears Years

func imain() int {
	flag.Parse()

	year := *fYear
	if year == 0 {
		fmt.Println("A year must be specified.")
		return 1
	}
	okDays, ok := okYears[year]
	if !ok {
		fmt.Printf("Invalid year: %d\n", *fYear)
		return 1
	}

	day := *fDay
	if day == 0 {
		fmt.Println("A day must be specified.")
		return 1
	}
	solutions, ok := okDays[day]
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
		solution = solutions.One
	} else {
		solution = solutions.Two
	}
	if solution == nil {
		fmt.Printf("Year %d, day %d, part %d has no solution.\n", year, day, part)
		return 1
	}
	fmt.Printf("Should run solution for year %d, day %d, part %d here.\n", year, day, part)

	return 0
}

func main() {
	os.Exit(imain())
}
