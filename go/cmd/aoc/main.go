package main

import (
	"flag"
	"fmt"
	"os"
)

var (
	fYear = flag.Uint("year", 0, "year of event")
)

var okYears = map[uint]struct{}{}

func imain() int {
	flag.Parse()

	year := *fYear
	if year == 0 {
		fmt.Println("A year must be specified.")
		return 1
	}
	if _, ok := okYears[year]; !ok {
		fmt.Printf("Invalid year: %d\n", *fYear)
		return 1
	}

	return 0
}

func main() {
	os.Exit(imain())
}
