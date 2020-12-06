package day06

import (
	"fmt"
	"io"
	"io/ioutil"
	"strings"
)

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func parse(paragraph string) (int, [26]int) {
	people := 0
	var counts [26]int
	for _, line := range strings.Split(paragraph, "\n") {
		if line == "" {
			continue
		}
		people++
		for _, r := range line {
			counts[r-'a']++
		}
	}
	return people, counts
}

func solve(r io.Reader, part int) (string, error) {
	data, err := ioutil.ReadAll(r)
	if err != nil {
		return "", fmt.Errorf("part %v: %w", part, err)
	}
	input := string(data)
	anyone := 0
	everyone := 0
	for _, paragraph := range strings.Split(input, "\n\n") {
		people, counts := parse(paragraph)
		for _, n := range counts {
			if n > 0 {
				anyone++
			}
			if n == people {
				everyone++
			}
		}
	}
	if part == 1 {
		return fmt.Sprint(anyone), nil
	} else {
		return fmt.Sprint(everyone), nil
	}
}
