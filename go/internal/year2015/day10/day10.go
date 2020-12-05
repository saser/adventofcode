package day10

import (
	"fmt"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	answer := strings.TrimSpace(input)
	var times int
	switch part {
	case 1:
		times = 40
	case 2:
		times = 50
	}
	for i := 0; i < times; i++ {
		answer = lookAndSay(answer)
	}
	return fmt.Sprint(len(answer)), nil
}

func lookAndSay(s string) string {
	current := rune(s[0])
	count := 1
	var sb strings.Builder
	sb.Grow(2 * len(s))
	for _, r := range s[1:] {
		if r == current {
			count++
		} else {
			sb.WriteString(fmt.Sprint(count))
			sb.WriteRune(current)
			current = r
			count = 1
		}
	}
	sb.WriteString(fmt.Sprint(count))
	sb.WriteRune(current)
	return sb.String()
}
