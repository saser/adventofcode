package day01

import (
	"fmt"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	floor := 0
	position := 0
	for _, r := range input {
		position++
		switch r {
		case '(':
			floor++
		case ')':
			floor--
		case '\n':
			break
		}
		if part == 2 && floor == -1 {
			return fmt.Sprint(position), nil
		}
	}
	return fmt.Sprint(floor), nil
}
