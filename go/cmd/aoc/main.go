package main

import (
	"flag"
	"fmt"
	"os"
)

var (
	fYear = flag.Uint("year", 0, "year of event")
	fDay  = flag.Uint("day", 0, "day in event")
)

type Years map[uint]Days
type Days map[uint]struct{}

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
	if _, ok := okDays[day]; !ok {
		fmt.Printf("Year %d has no solution for day %d.\n", year, day)
		return 1
	}

	return 0
}

func main() {
	os.Exit(imain())
}
