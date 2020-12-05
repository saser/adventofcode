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

func parse(s string) int {
	n := 0
	for _, r := range s {
		n = n << 1
		if r == 'B' || r == 'R' {
			n += 1
		}
	}
	return n
}

func solve(r io.Reader, part int) (string, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	min := (1 << 10) - 1
	max := 0
	sum := 0
	for sc.Scan() {
		n := parse(sc.Text())
		if n < min {
			min = n
		}
		if n > max {
			max = n
		}
		sum += n
	}
	if part == 1 {
		return fmt.Sprint(max), nil
	} else {
		total := ((max - min + 1) * (min + max)) / 2
		missing := total - sum
		return fmt.Sprint(missing), nil
	}
}
