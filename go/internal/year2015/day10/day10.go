package day10

import (
	"bufio"
	"fmt"
	"io"
	"strings"
)

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
	br := bufio.NewReader(r)
	line, err := br.ReadString('\n')
	if err != nil {
		return "", fmt.Errorf("year 2015, day 10, part 1: %w", err)
	}
	answer := strings.TrimSpace(line)
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
