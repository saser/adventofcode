package day17

import (
	"fmt"
	"strconv"
	"strings"
)

var Target = 150

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	parts, err := parse(input)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 17, part %d: %w", part, err)
	}
	proper := combinations(Target, parts)
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

func parse(input string) ([]int, error) {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	parts := make([]int, len(lines))
	for i, line := range lines {
		part, err := strconv.Atoi(line)
		if err != nil {
			return nil, fmt.Errorf("parse: invalid line: %s", line)
		}
		parts[i] = part
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
