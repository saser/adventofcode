package day01

import (
	"fmt"
	"strconv"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	ints := make([]int, len(lines))
	for idx, line := range lines {
		i, err := strconv.Atoi(line)
		if err != nil {
			return "", fmt.Errorf("part %v: %w", part, err)
		}
		ints[idx] = i
	}
	bools := buildBools(ints)
	switch part {
	case 1:
		n1, n2, ok := twoSum(2020, ints, bools)
		if !ok {
			break
		}
		return fmt.Sprint(n1 * n2), nil
	case 2:
		for i := 0; i < len(ints); i++ {
			n1 := ints[i]
			target := 2020 - n1
			n2, n3, ok := twoSum(target, ints[i+1:], bools)
			if !ok {
				continue
			}
			return fmt.Sprint(n1 * n2 * n3), nil
		}
	}
	return "", fmt.Errorf("part %v: no solution found", part)
}

func buildBools(ints []int) []bool {
	bools := make([]bool, 2020)
	for _, i := range ints {
		bools[i] = true
	}
	return bools
}

func twoSum(target int, ints []int, bools []bool) (int, int, bool) {
	for _, i := range ints {
		idx := target - i
		if idx < 0 {
			continue
		}
		if bools[idx] {
			return i, idx, true
		}
	}
	return 0, 0, false
}
