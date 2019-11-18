package day17

import (
	"bufio"
	"fmt"
	"io"
	"strconv"

	"github.com/Saser/adventofcode/internal/solution"
)

func Part1(target int) solution.Solution {
	return solve(target, 1)
}

func Part2(target int) solution.Solution {
	return solve(target, 2)
}

func solve(target int, part int) solution.Solution {
	return func(r io.Reader) (string, error) {
		parts, err := parse(r)
		if err != nil {
			return "", fmt.Errorf("year 2015, day 17, part %d: %w", part, err)
		}
		proper := combinations(target, parts)
		if part == 1 {
			return fmt.Sprint(len(proper)), nil
		}
		count := 0
		minCombination := len(proper[0])
		for _, combination := range proper {
			n := len(combination)
			switch {
			case n == minCombination:
				count++
			case n < minCombination:
				count = 1
				minCombination = n
			}
		}
		return fmt.Sprint(count), nil
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

func combinations(target int, parts []int) (proper [][]int) {
	if target < 0 {
		return [][]int{}
	}
	i := parts[0]
	if i == target {
		proper = append(proper, []int{i})
	}
	if len(parts) == 1 {
		return
	}
	rest := parts[1:]
	for _, combination := range combinations(target-i, rest) {
		proper = append(proper, append([]int{i}, combination...))
	}
	proper = append(proper, combinations(target, rest)...)
	return
}
