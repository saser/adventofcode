package day05

import (
	"bufio"
	"fmt"
	"io"
)

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func parse(s string) int16 {
	if len(s) != 10 {
		panic(fmt.Sprintf("invalid string: %q", s))
	}
	n := int16(0)
	for _, r := range s {
		n = n << 1
		if r == 'B' || r == 'R' {
			n += 1
		}
	}
	return n
}

func solve(r io.Reader, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	var max int16
	for sc.Scan() {
		if n := parse(sc.Text()); n > max {
			max = n
		}
	}
	return fmt.Sprint(max), nil
}
