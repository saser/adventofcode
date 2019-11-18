package day17

import (
	"bufio"
	"fmt"
	"io"
	"strconv"

	"github.com/Saser/adventofcode/internal/solution"
)

func Part1(target int) solution.Solution {
	return func(r io.Reader) (string, error) {
		parts, err := parse(r)
		if err != nil {
			return "", fmt.Errorf("year 2015, day 17, part 1: %w", err)
		}
		return fmt.Sprint(sumCombinations(target, parts)), nil
	}
}

func parse(r io.Reader) ([]int, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	parts := make([]int, 0)
	for sc.Scan() {
		part, err := strconv.Atoi(sc.Text())
		if err != nil {
			return nil, fmt.Errorf("parse: invalid line: %s", sc.Text())
		}
		parts = append(parts, part)
	}
	return parts, nil
}

func sumCombinations(target int, parts []int) int {
	if target < 0 {
		return 0
	}
	var rest []int
	switch len(parts) {
	case 0:
		if target == 0 {
			return 1
		} else {
			return 0
		}
	case 1:
		rest = []int{}
	default:
		rest = parts[1:]
	}
	return sumCombinations(target-parts[0], rest) + sumCombinations(target, rest)
}
