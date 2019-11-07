package day10

import (
	"bufio"
	"fmt"
	"io"
	"strings"
)

func Part1(r io.Reader) (string, error) {
	br := bufio.NewReader(r)
	line, err := br.ReadString('\n')
	if err != nil {
		return "", fmt.Errorf("year 2015, day 10, part 1: %w", err)
	}
	answer := strings.TrimSpace(line)
	for i := 0; i < 40; i++ {
		answer = lookAndSay(answer)
	}
	return fmt.Sprint(len(answer)), nil
}

func lookAndSay(s string) string {
	current := rune(s[0])
	count := 1
	runes := make([]rune, 0)
	counts := make([]int, 0)
	for _, r := range s[1:] {
		if r == current {
			count++
		} else {
			runes = append(runes, current)
			counts = append(counts, count)
			current = r
			count = 1
		}
	}
	runes = append(runes, current)
	counts = append(counts, count)
	var sb strings.Builder
	for i := 0; i < len(runes); i++ {
		sb.WriteString(fmt.Sprint(counts[i]))
		sb.WriteRune(runes[i])
	}
	return sb.String()
}
