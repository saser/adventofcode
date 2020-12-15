package day15

import "fmt"

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	return "", fmt.Errorf("solution not implemented for part %v", part)
}
