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

func parse(paragraph string) [26]int {
	var counts [26]int
	for _, line := range strings.Split(paragraph, "\n") {
		for _, r := range line {
			counts[r-'a']++
		}
	}
	return counts
}

func solve(r io.Reader, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	data, err := ioutil.ReadAll(r)
	if err != nil {
		return "", fmt.Errorf("part %v: %w", part, err)
	}
	input := string(data)
	sum := 0
	for _, paragraph := range strings.Split(input, "\n\n") {
		for _, n := range parse(paragraph) {
			if n > 0 {
				sum++
			}
		}
	}
	return fmt.Sprint(sum), nil
}
